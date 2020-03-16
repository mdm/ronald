use crate::memory;
use crate::instruction::{ Decoder, Instruction, JumpTest, Operand };

pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    I,
    R,
    IXH,
    IXL,
    IYH,
    IYL,
}

pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    IX,
    IY,
    SP,
    PC,
}

pub struct RegisterFile {
    data: Vec<u16>,
}

impl RegisterFile {
    fn new() -> RegisterFile {
        RegisterFile { data: vec![0; 14] }
    }

    pub fn read_byte(&self, register: &Register8) -> u8 {
        let value = match register {
            Register8::A => self.data[0] >> 8,
            Register8::F => self.data[0] & 0xff,
            Register8::B => self.data[1] >> 8,
            Register8::C => self.data[1] & 0xff,
            Register8::D => self.data[2] >> 8,
            Register8::E => self.data[2] & 0xff,
            Register8::H => self.data[3] >> 8,
            Register8::L => self.data[3] & 0xff,
            Register8::I => self.data[8] & 0xff,
            Register8::R => self.data[9] & 0xff,
            Register8::IXH => self.data[10] >> 8,
            Register8::IXL => self.data[10] & 0xff,
            Register8::IYH => self.data[11] >> 8,
            Register8::IYL => self.data[11] & 0xff,
        };
        
        value as u8
    }

    fn write_byte(&mut self, register: &Register8, value: u8) { // TODO: add tests
        let value = value as u16;

        match register {
            Register8::A => { self.data[0] = (value << 8) + (self.data[0] & 0xff); }
            Register8::F => { self.data[0] = (self.data[0] & 0xff00) + value; }
            Register8::B => { self.data[1] = (value << 8) + (self.data[1] & 0xff); }
            Register8::C => { self.data[1] = (self.data[1] & 0xff00) + value; }
            Register8::D => { self.data[2] = (value << 8) + (self.data[2] & 0xff); }
            Register8::E => { self.data[2] = (self.data[2] & 0xff00) + value; }
            Register8::H => { self.data[3] = (value << 8) + (self.data[3] & 0xff); }
            Register8::L => { self.data[3] = (self.data[3] & 0xff00) + value; }
            Register8::I => { self.data[8] = (self.data[8] & 0xff00) + value; }
            Register8::R => { self.data[9] = (self.data[9] & 0xff00) + value; }
            Register8::IXH => { self.data[10] = (value << 8) + (self.data[10] & 0xff); }
            Register8::IXL => { self.data[10] = (self.data[10] & 0xff00) + value; }
            Register8::IYH => { self.data[11] = (value << 8) + (self.data[11] & 0xff); }
            Register8::IYL => { self.data[11] = (self.data[11] & 0xff00) + value; }
        }
    }

    pub fn read_word(&self, register: &Register16) -> u16 {
        match register {
            Register16::AF => self.data[0],
            Register16::BC => self.data[1],
            Register16::DE => self.data[2],
            Register16::HL => self.data[3],
            Register16::IX => self.data[10],
            Register16::IY => self.data[11],
            Register16::SP => self.data[12],
            Register16::PC => self.data[13],
        }
    }

    fn write_word(&mut self, register: &Register16, value: u16) {
        match register {
            Register16::AF => { self.data[0] = value; },
            Register16::BC => { self.data[1] = value; },
            Register16::DE => { self.data[2] = value; },
            Register16::HL => { self.data[3] = value; },
            Register16::IX => { self.data[10] = value; },
            Register16::IY => { self.data[11] = value; },
            Register16::SP => { self.data[12] = value; },
            Register16::PC => { self.data[13] = value; },
        }
    }
}

enum Flag {
    Carry,
    AddSubtract,
    ParityOverflow,
    HalfCarry,
    Zero,
    Sign,
}

impl Flag {
    fn mask(&self) -> u8 {
        match self {
            Flag::Carry => 1 << 0,
            Flag::AddSubtract => 1 << 1,
            Flag::ParityOverflow => 1 << 2,
            Flag::HalfCarry => 1 << 4,
            Flag::Zero => 1 << 6,
            Flag::Sign => 1 << 7,
        }
    }
}

pub struct CPU {
    pub registers: RegisterFile,
    pub halted: bool,
    decoder: Decoder,
}

impl CPU
{
    pub fn new(initial_pc: u16) -> CPU {
        let mut registers = RegisterFile::new();
        registers.write_word(&Register16::PC, initial_pc);

        let mut cpu = CPU {
            registers,
            halted: false,
            decoder: Decoder::new(),
        };

        cpu.reset();

        cpu
    }

    pub fn fetch_and_execute<M>(&mut self, memory: &mut M) // TODO: pass bus instead of individual devices
    where M: memory::Read + memory::Write {
        let (instruction, next_address) = self.decoder.decode_at(
            memory, self.registers.read_word(&Register16::PC) as usize
        );

        println!("{:#06x}: {}", self.registers.read_word(&Register16::PC), &instruction);
        match instruction {
            Instruction::Add(destination, source) => {
                match destination {
                    Operand::Register8(Register8::A) => {
                        let left = self.load_byte(memory, &destination);
                        let right = self.load_byte(memory, &source);
                        let value = left as u16 + right as u16;
                        self.store_byte(memory, &destination, value as u8);

                        self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                        self.set_flag(Flag::Zero, (value as u8) == 0);
                        self.set_flag(Flag::HalfCarry, (((left & 0xf) + (right & 0xf)) & 0x10) != 0);
                        self.set_flag(Flag::ParityOverflow, ((left & 0x80) == (right & 0x80)) && ((left & 0x80) == ((value as u8) & 0x80)));
                        self.set_flag(Flag::AddSubtract, false);
                        self.set_flag(Flag::Carry, (value & 0x100) != 0);
                    }
                    Operand::Register16(Register16::HL) => {
                        let left = self.load_word(memory, &destination);
                        let right = self.load_word(memory, &source);
                        let value = left as u32 + right as u32;
                        self.store_word(memory, &destination, value as u16);

                        self.set_flag(Flag::HalfCarry, (((left & 0xfff) + (right & 0xfff)) & 0x1000) != 0);
                        self.set_flag(Flag::AddSubtract, false);
                        self.set_flag(Flag::Carry, (value & 0x10000) != 0);
                    }
                    _ => unreachable!(),
                }

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::And(operand) => {
                let value = self.load_byte(memory, &operand);

                let result = self.registers.read_byte(&Register8::A) & value;
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::Sign, (result as i8) < 0);
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, true);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, false);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Call(jump_test, Operand::Immediate16(address)) => {
                if self.check_jump(jump_test) {
                    let new_sp = self.registers.read_word(&Register16::SP) - 2;
                    self.registers.write_word(&Register16::SP, new_sp);
                    memory.write_word(new_sp as usize, next_address as u16);
                    self.registers.write_word(&Register16::PC, address);
                } else {
                    self.registers.write_word(&Register16::PC, next_address as u16);
                }

            }
            Instruction::Cp(operand) => {
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &operand);
                let value = left as u16 + !right as u16 + 1;

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, (value as u8) == 0);
                self.set_flag(Flag::HalfCarry, (((left & 0xf) + (right & 0xf)) & 0x10) != 0);
                self.set_flag(Flag::ParityOverflow, ((left & 0x80) == (right & 0x80)) && ((left & 0x80) == ((value as u8) & 0x80)));
                self.set_flag(Flag::AddSubtract, true);
                self.set_flag(Flag::Carry, (value & 0x100) != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Dec(destination) => {
                match destination {
                    Operand::Register8(register) => {
                        let mut value = self.registers.read_byte(&register);
                        self.set_flag(Flag::ParityOverflow, value == 0x80);
                        value -= 1;
                        self.registers.write_byte(&register, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, false); // TODO: fix this
                        self.set_flag(Flag::AddSubtract, true);
                    }
                    Operand::Register16(register) => {
                        let value = self.registers.read_word(&register);
                        self.registers.write_word(&register, value - 1);
                    }
                    _ => unreachable!(),
                }

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Ex(left, right) => {
                let left_value = self.load_word(memory, &left);
                let right_value = self.load_word(memory, &right);

                self.store_word(memory, &left, right_value);
                self.store_word(memory, &right, left_value);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Inc(destination) => {
                match destination {
                    Operand::Register8(register) => {
                        let mut value = self.registers.read_byte(&register);
                        self.set_flag(Flag::ParityOverflow, value == 0x7f);
                        value += 1;
                        self.registers.write_byte(&register, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, false); // TODO: fix this
                        self.set_flag(Flag::AddSubtract, false);
                    }
                    Operand::Register16(register) => {
                        let value = self.registers.read_word(&register);
                        self.registers.write_word(&register, value + 1);
                    }
                    _ => unreachable!(),
                }

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Jp(jump_test, target) => {
                if self.check_jump(jump_test) {
                    let address = match target {
                        Operand::Immediate16(address) => address,
                        _ => {
                            unimplemented!();
                        }
                    };
                    self.registers.write_word(&Register16::PC, address);
                } else {
                    self.registers.write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Ld(destination, source) => {
                match (&destination, &source) {
                    (Operand::Register16(_), _) => {
                        let value = self.load_word(memory, &source);
                        self.store_word(memory, &destination, value);
                    }
                    (_, Operand::Register16(_)) => {
                        let value = self.load_word(memory, &source);
                        self.store_word(memory, &destination, value);
                    }
                    _ => {
                        let value = self.load_byte(memory, &source);
                        self.store_byte(memory, &destination, value);
                    }
                }

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Ldir => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::DE), value);

                let address = self.registers.read_word(&Register16::DE) + 1;
                self.registers.write_word(&Register16::DE, address);

                let address = self.registers.read_word(&Register16::HL) + 1;
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_word(&Register16::BC) - 1;
                self.registers.write_word(&Register16::BC, counter);
                if counter == 0 {
                    self.registers.write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Or(operand) => {
                let value = self.load_byte(memory, &operand);

                let result = self.registers.read_byte(&Register8::A) | value;
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::Sign, (result as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, false);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Pop(Operand::Register16(destination)) => {
                let old_sp = self.registers.read_word(&Register16::SP);
                self.registers.write_word(&Register16::SP, old_sp + 2);
                self.registers.write_word(&destination, memory.read_word(old_sp as usize));
                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Push(Operand::Register16(source)) => {
                let new_sp = self.registers.read_word(&Register16::SP) - 2;
                self.registers.write_word(&Register16::SP, new_sp);
                memory.write_word(new_sp as usize, self.registers.read_word(&source));
                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Ret(jump_test) => {
                if self.check_jump(jump_test) {
                    let old_sp = self.registers.read_word(&Register16::SP);
                    self.registers.write_word(&Register16::SP, old_sp + 2);
                    self.registers.write_word(&Register16::PC, memory.read_word(old_sp as usize));
                } else {
                    self.registers.write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Rlca => {
                let value = (self.registers.read_byte(&Register8::A) as u16) << 1;
                let carry = value >> 8;
                self.registers.write_byte(&Register8::A, (value | carry) as u8);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rrca => {
                let value = (self.registers.read_byte(&Register8::A) as u16) << 1;
                let carry = value & 1;
                self.registers.write_byte(&Register8::A, (value | (carry << 7)) as u8);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            _ => {
                unimplemented!();
            }
        }
    }

    fn reset(&mut self) {
        // TODO: implement reset
    }

    fn check_flag(&self, flag: Flag) -> bool {
        let flags = self.registers.read_byte(&Register8::F);

        flags & flag.mask() != 0
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        let old_flags = self.registers.read_byte(&Register8::F);

        let new_flags = if value {
            old_flags | flag.mask()
        } else {
            old_flags & (!flag.mask()) // TODO: add tests
        };
        
        self.registers.write_byte(&Register8::F, new_flags);
    }

    fn check_jump(&self, jump_test: JumpTest) -> bool {
        match jump_test {
            JumpTest::Unconditional => true,
            JumpTest::NonZero => !self.check_flag(Flag::Zero),
            JumpTest::Zero => self.check_flag(Flag::Zero),
            JumpTest::NoCarry => !self.check_flag(Flag::Carry),
            JumpTest::Carry => self.check_flag(Flag::Carry),
            JumpTest::ParityOdd => !self.check_flag(Flag::ParityOverflow),
            JumpTest::ParityEven => self.check_flag(Flag::ParityOverflow),
            JumpTest::SignPositive => !self.check_flag(Flag::Sign),
            JumpTest::SignNegative => self.check_flag(Flag::Sign),
        }
    }

    fn load_byte<M>(&self, memory: &M, operand: &Operand) -> u8
    where M: memory::Read {
        match operand {
            Operand::Immediate8(value) => *value,
            Operand::Register8(register) => self.registers.read_byte(register),
            Operand::Direct8(_) => unimplemented!(),
            Operand::Direct16(address) => memory.read_byte(*address as usize),
            Operand::RegisterIndirect(register) => {
                let address = self.registers.read_word(register);
                memory.read_byte(address as usize)
            }
            Operand::Indexed(register, displacement) => {
                let address = self.registers.read_word(register);
                memory.read_byte((address as i64 + *displacement as i64) as usize)
            }
            _ => unreachable!(),
        }
    }

    fn store_byte<M>(&mut self, memory: &mut M, operand: &Operand, value: u8)
    where M: memory::Read + memory::Write {
        match operand {
            Operand::Register8(register) => self.registers.write_byte(register, value),
            Operand::Direct8(_) => unimplemented!(),
            Operand::Direct16(address) => memory.write_byte(*address as usize, value),
            Operand::RegisterIndirect(register) => {
                let address = self.registers.read_word(register);
                memory.write_byte(address as usize, value)
            }
            Operand::Indexed(register, displacement) => {
                let address = self.registers.read_word(register);
                memory.write_byte((address as i64 + *displacement as i64) as usize, value)
            }
            _ => unreachable!(),
        }
    }

    fn load_word<M>(&self, memory: &M, operand: &Operand) -> u16
    where M: memory::Read {
        match operand {
            Operand::Immediate16(value) => *value,
            Operand::Register16(register) => self.registers.read_word(register),
            Operand::Direct16(address) => memory.read_word(*address as usize),
            Operand::RegisterIndirect(register) => {
                let address = self.registers.read_word(register);
                memory.read_word(address as usize)
            }
            Operand::Indexed(register, displacement) => {
                let address = self.registers.read_word(register);
                memory.read_word((address as i64 + *displacement as i64) as usize)
            }
            _ => unreachable!(),
        }
    }

    fn store_word<M>(&mut self, memory: &mut M, operand: &Operand, value: u16)
    where M: memory::Read + memory::Write {
        match operand {
            Operand::Register16(register) => self.registers.write_word(register, value),
            Operand::Direct16(address) => memory.write_word(*address as usize, value),
            Operand::RegisterIndirect(register) => {
                let address = self.registers.read_word(register);
                memory.write_word(address as usize, value)
            }
            Operand::Indexed(register, displacement) => {
                let address = self.registers.read_word(register);
                memory.write_word((address as i64 + *displacement as i64) as usize, value)
            }
            _ => unreachable!(),
        }
    }
}

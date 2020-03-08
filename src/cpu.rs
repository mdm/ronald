use crate::memory;
use crate::instruction::{ Decoder, Instruction, JumpTest, Operand };

pub enum Register {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    I,
    R,
    IX,
    IXH,
    IXL,
    IY,
    IYH,
    IYL,
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

    pub fn read_byte(&self, register: &Register) -> u8 {
        let value = match register {
            Register::A => self.data[0] >> 8,
            Register::F => self.data[0] & 0xff,
            Register::B => self.data[1] >> 8,
            Register::C => self.data[1] & 0xff,
            Register::D => self.data[2] >> 8,
            Register::E => self.data[2] & 0xff,
            Register::H => self.data[3] >> 8,
            Register::L => self.data[3] & 0xff,
            Register::I => self.data[8] & 0xff,
            Register::R => self.data[9] & 0xff,
            Register::IXH => self.data[10] >> 8,
            Register::IXL => self.data[10] & 0xff,
            Register::IYH => self.data[11] >> 8,
            Register::IYL => self.data[11] & 0xff,
            _ => unreachable!(),
        };
        
        value as u8
    }

    fn write_byte(&mut self, register: &Register, value: u8) { // TODO: add tests
        let value = value as u16;

        match register {
            Register::A => { self.data[0] = (value << 8) + (self.data[0] & 0xff); }
            Register::F => { self.data[0] = (self.data[0] & 0xff00) + value; }
            Register::B => { self.data[1] = (value << 8) + (self.data[1] & 0xff); }
            Register::C => { self.data[1] = (self.data[1] & 0xff00) + value; }
            Register::D => { self.data[2] = (value << 8) + (self.data[2] & 0xff); }
            Register::E => { self.data[2] = (self.data[2] & 0xff00) + value; }
            Register::H => { self.data[3] = (value << 8) + (self.data[3] & 0xff); }
            Register::L => { self.data[3] = (self.data[3] & 0xff00) + value; }
            Register::I => { self.data[8] = (self.data[8] & 0xff00) + value; }
            Register::R => { self.data[9] = (self.data[9] & 0xff00) + value; }
            Register::IXH => { self.data[10] = (value << 8) + (self.data[10] & 0xff); }
            Register::IXL => { self.data[10] = (self.data[10] & 0xff00) + value; }
            Register::IYH => { self.data[11] = (value << 8) + (self.data[11] & 0xff); }
            Register::IYL => { self.data[11] = (self.data[11] & 0xff00) + value; }
            _ => unreachable!(),
        }
    }

    pub fn read_word(&self, register: &Register) -> u16 {
        match register {
            Register::AF => self.data[0],
            Register::BC => self.data[1],
            Register::DE => self.data[2],
            Register::HL => self.data[3],
            Register::IX => self.data[10],
            Register::IY => self.data[11],
            Register::SP => self.data[12],
            Register::PC => self.data[13],
            _ => unreachable!(),
        }
    }

    fn write_word(&mut self, register: &Register, value: u16) {
        match register {
            Register::AF => { self.data[0] = value; },
            Register::BC => { self.data[1] = value; },
            Register::DE => { self.data[2] = value; },
            Register::HL => { self.data[3] = value; },
            Register::IX => { self.data[10] = value; },
            Register::IY => { self.data[11] = value; },
            Register::SP => { self.data[12] = value; },
            Register::PC => { self.data[13] = value; },
            _ => unreachable!(),
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
        registers.write_word(&Register::PC, initial_pc);

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
            memory, self.registers.read_word(&Register::PC) as usize
        );

        println!("{:#06x}: {}", self.registers.read_word(&Register::PC), &instruction);
        match instruction {
            Instruction::Add(destination, source) => {
                match destination {
                    Operand::Register(Register::A) => {
                        let left = self.load_byte(memory, &destination);
                        let right = self.load_byte(memory, &source);
                        let value = left as u16 + right as u16;
                        self.store_byte(memory, &destination, value as u8);

                        self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (((left & 0xf) + (right & 0xf)) & 0x10) != 0);
                        self.set_flag(Flag::ParityOverflow, ((left & 0x80) == (right & 0x80)) && ((left & 0x80) == ((value as u8) & 0x80)));
                        self.set_flag(Flag::AddSubtract, false);
                        self.set_flag(Flag::Carry, (value & 0x100) != 0);
                    }
                    Operand::Register(Register::HL) => {
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

                self.registers.write_word(&Register::PC, next_address as u16);
            }
            Instruction::Call(jump_test, Operand::Immediate16(address)) => {
                if self.check_jump(jump_test) {
                    let new_sp = self.registers.read_word(&Register::SP) - 2;
                    self.registers.write_word(&Register::SP, new_sp);
                    memory.write_word(new_sp as usize, next_address as u16);
                    self.registers.write_word(&Register::PC, address);
                } else {
                    self.registers.write_word(&Register::PC, next_address as u16);
                }

            }
            Instruction::Dec(Operand::Register(register)) => {
                let value = self.registers.read_word(&register);
                self.registers.write_word(&register, value - 1);

                self.registers.write_word(&Register::PC, next_address as u16);
            }
            Instruction::Ex(left, right) => {
                let left_value = self.load_word(memory, &left);
                let right_value = self.load_word(memory, &right);

                self.store_word(memory, &left, right_value);
                self.store_word(memory, &right, left_value);

                self.registers.write_word(&Register::PC, next_address as u16);
            }
            Instruction::Inc(Operand::Register(register)) => {
                let value = self.registers.read_word(&register);
                self.registers.write_word(&register, value + 1);

                self.registers.write_word(&Register::PC, next_address as u16);
            }
            Instruction::Jp(jump_test, target) => {
                if self.check_jump(jump_test) {
                    let address = match target {
                        Operand::Immediate16(address) => address,
                        _ => {
                            unimplemented!();
                        }
                    };
                    self.registers.write_word(&Register::PC, address);
                } else {
                    self.registers.write_word(&Register::PC, next_address as u16);
                }
            }
            Instruction::Ld8(destination, source) => {
                let value = self.load_byte(memory, &source);
                self.store_byte(memory, &destination, value);

                self.registers.write_word(&Register::PC, next_address as u16);
            }
            Instruction::Ld16(destination, source) => {
                let value = self.load_word(memory, &source);
                self.store_word(memory, &destination, value);

                self.registers.write_word(&Register::PC, next_address as u16);
            }
            Instruction::Or(operand) => {
                let value = self.load_byte(memory, &operand);

                let result = self.registers.read_byte(&Register::A) | value;
                self.registers.write_byte(&Register::A, result);

                self.set_flag(Flag::Sign, (result as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, false);

                self.registers.write_word(&Register::PC, next_address as u16);
            }
            Instruction::Pop(Operand::Register(destination)) => {
                let old_sp = self.registers.read_word(&Register::SP);
                self.registers.write_word(&Register::SP, old_sp + 2);
                self.registers.write_word(&destination, memory.read_word(old_sp as usize));
                self.registers.write_word(&Register::PC, next_address as u16);
            }
            Instruction::Push(Operand::Register(source)) => {
                let new_sp = self.registers.read_word(&Register::SP) - 2;
                self.registers.write_word(&Register::SP, new_sp);
                memory.write_word(new_sp as usize, self.registers.read_word(&source));
                self.registers.write_word(&Register::PC, next_address as u16);
            }
            Instruction::Ret(jump_test) => {
                if self.check_jump(jump_test) {
                    let old_sp = self.registers.read_word(&Register::SP);
                    self.registers.write_word(&Register::SP, old_sp + 2);
                    self.registers.write_word(&Register::PC, memory.read_word(old_sp as usize));
                } else {
                    self.registers.write_word(&Register::PC, next_address as u16);
                }
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
        let flags = self.registers.read_byte(&Register::F);

        flags & flag.mask() != 0
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        let old_flags = self.registers.read_byte(&Register::F);

        let new_flags = if value {
            old_flags | flag.mask()
        } else {
            old_flags & (!flag.mask()) // TODO: add tests
        };
        
        self.registers.write_byte(&Register::F, new_flags);
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
            Operand::Register(register) => self.registers.read_byte(register),
            Operand::Direct8(_) => unimplemented!(),
            Operand::Direct16(address) => memory.read_byte(*address as usize),
            Operand::RegisterIndirect(Register::C) => unimplemented!(),
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
            Operand::Register(register) => self.registers.write_byte(register, value),
            Operand::Direct8(_) => unimplemented!(),
            Operand::Direct16(address) => memory.write_byte(*address as usize, value),
            Operand::RegisterIndirect(Register::C) => unimplemented!(),
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
            Operand::Register(register) => self.registers.read_word(register),
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
            Operand::Register(register) => self.registers.write_word(register, value),
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

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
    pub iff1: bool,
    pub iff2: bool,
    pub halted: bool,
    decoder: Decoder,
    enable_interrupt: bool,
}

impl CPU
{
    pub fn new(initial_pc: u16) -> CPU {
        let mut registers = RegisterFile::new();
        registers.write_word(&Register16::PC, initial_pc);

        let mut cpu = CPU {
            registers,
            iff1: false,
            iff2: false,
            halted: false,
            decoder: Decoder::new(),
            enable_interrupt: false,
        };

        cpu.reset();

        cpu
    }

    pub fn fetch_and_execute<M>(&mut self, memory: &mut M) // TODO: pass bus instead of individual devices
    where M: memory::Read + memory::Write {
        if self.enable_interrupt {
            self.iff1 = true;
            self.iff2 = true;
            self.enable_interrupt = false;
        }

        let (instruction, next_address) = self.decoder.decode_at(
            memory, self.registers.read_word(&Register16::PC) as usize
        );

        match instruction {
            Instruction::Adc(destination, source) => {
                match destination {
                    Operand::Register8(Register8::A) => {
                        let left = self.load_byte(memory, &destination);
                        let right = self.load_byte(memory, &source);
                        // let (value, carry1) = left.overflowing_add(right);
                        // let (_, overflow1) = (left as i8).overflowing_add(right as i8);
                        let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                        // let (value, carry2) = value.overflowing_add(carry_value);
                        let (value, carry) = left.overflowing_add(right.wrapping_add(carry_value));
                        // let (_, overflow2) = (value as i8).overflowing_add(carry_value as i8);
                        let overflow = (left & 0x80) == (right & 0x80) && (right & 0x80) != (value & 0x80);
                        self.store_byte(memory, &destination, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (((left & 0xf) + (right & 0xf) + carry_value) & 0x10) != 0);
                        self.set_flag(Flag::ParityOverflow, overflow);
                        self.set_flag(Flag::AddSubtract, false);
                        // self.set_flag(Flag::Carry, carry1 || carry2);
                        self.set_flag(Flag::Carry, carry);
                    }
                    Operand::Register16(Register16::HL) => {
                        let left = self.load_word(memory, &destination);
                        let right = self.load_word(memory, &source);
                        let (value, carry1) = left.overflowing_add(right);
                        let (_, overflow1) = (left as i16).overflowing_add(right as i16);
                        let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                        let (value, carry2) = value.overflowing_add(carry_value);
                        let (_, overflow2) = (value as i16).overflowing_add(carry_value as i16);
                        self.store_word(memory, &destination, value);

                        self.set_flag(Flag::Sign, (value as i16) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (((left & 0xfff) + (right & 0xfff)) & 0x1000) != 0);
                        self.set_flag(Flag::ParityOverflow, overflow1 || overflow2);
                        self.set_flag(Flag::AddSubtract, false);
                        self.set_flag(Flag::Carry, carry1 || carry2);
                    }
                    _ => unreachable!(),
                }

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Add(destination, source) => {
                match destination {
                    Operand::Register8(Register8::A) => { // ALUOP / checked
                        let left = self.load_byte(memory, &destination);
                        let right = self.load_byte(memory, &source);
                        let (value, carry) = left.overflowing_add(right);
                        let (_, overflow) = (left as i8).overflowing_add(right as i8);
                        self.store_byte(memory, &destination, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (((left & 0xf) + (right & 0xf)) & 0x10) != 0);
                        self.set_flag(Flag::ParityOverflow, overflow);
                        self.set_flag(Flag::AddSubtract, false);
                        self.set_flag(Flag::Carry, carry);
                    }
                    _ => {
                        let left = self.load_word(memory, &destination);
                        let right = self.load_word(memory, &source);
                        let (value, carry) = left.overflowing_add(right);
                        self.store_word(memory, &destination, value);

                        self.set_flag(Flag::HalfCarry, (((left & 0xfff) + (right & 0xfff)) & 0x1000) != 0);
                        self.set_flag(Flag::AddSubtract, false);
                        self.set_flag(Flag::Carry, carry);
                    }
                }

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::And(operand) => {
                // println!("BEGIN AND");
                // self.print_state();
                let value = self.load_byte(memory, &operand);
                // println!("OPERANDS: {:#04x}, {:#04x}", self.registers.read_byte(&Register8::A), value);

                let result = self.registers.read_byte(&Register8::A) & value;
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::Sign, (result as i8) < 0);
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, true);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, false);

                // self.print_state();
                // println!("END AND");

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Bit(Operand::Bit(bit), operand) => {
                let value = self.load_byte(memory, &operand);

                self.set_flag(Flag::Zero, (value & (1 << bit)) == 0);
                self.set_flag(Flag::HalfCarry, true);
                self.set_flag(Flag::AddSubtract, false);

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
            Instruction::Ccf => {
                let carry = self.check_flag(Flag::Carry);

                self.set_flag(Flag::HalfCarry, carry);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, !carry);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Cp(operand) => { // ALUOP / checked
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &operand);
                let (value, carry) = left.overflowing_sub(right);
                let (_, overflow) = (left as i8).overflowing_sub(right as i8);

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, overflow);
                self.set_flag(Flag::AddSubtract, true);
                self.set_flag(Flag::Carry, carry);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Cpd => {
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let value = left.wrapping_sub(right);

                let source = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, source);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, true);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Cpdr => {
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let value = left.wrapping_sub(right);

                let source = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, source);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, true);

                if counter == 0 || value == 0 {
                    self.registers.write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Cpi => {
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let value = left.wrapping_sub(right);

                let source = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, source);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, true);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Cpir => {
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let value = left.wrapping_sub(right);

                let source = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, source);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, true);

                if counter == 0 || value == 0 {
                    self.registers.write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Cpl => {
                let value = self.registers.read_byte(&Register8::A);
                self.registers.write_byte(&Register8::A, !value);

                self.set_flag(Flag::HalfCarry, true);
                self.set_flag(Flag::AddSubtract, true);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Daa => {
                let mut value = self.registers.read_byte(&Register8::A);
                let mut half_carry = self.check_flag(Flag::HalfCarry);
                let mut carry = self.check_flag(Flag::Carry);
                match self.check_flag(Flag::AddSubtract) {
                    true => {
                        if (value & 0xf) > 0x9 || half_carry {
                            value -= 0x6;
                        }

                        // if (value >> 4) > 0x9 || carry {
                        if value > 0x99 || carry {
                            value -= 0x60;
                            carry = true;
                        }

                        half_carry = half_carry && (value & 0xf) < 0x6;
                    }
                    false => {
                        if (value & 0xf) > 0x9 || half_carry {
                            value += 0x6;
                        }

                        // if (value >> 4) > 0x9 || carry {
                        if value > 0x99 || carry {
                            value += 0x60;
                            carry = true;
                        }

                        half_carry = (value & 0xf) >= 0xa;
                    }
                }

                self.registers.write_byte(&Register8::A, value);

                self.set_flag(Flag::Sign, (value as i8) < 0);
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, half_carry);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::Carry, carry);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Dec(destination) => {
                match destination {
                    Operand::Register16(register) => {
                        let value = self.registers.read_word(&register);
                        self.registers.write_word(&register, value.wrapping_sub(1));
                    }
                    _ => {
                        let old_value = self.load_byte(memory, &destination);
                        self.set_flag(Flag::ParityOverflow, old_value == 0x80);
                        let (value, _) = old_value.overflowing_sub(1);
                        self.store_byte(memory, &destination, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (old_value & 0xf) < 1);
                        self.set_flag(Flag::AddSubtract, true);
                    }
                }

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Defb(_operand) => {
                // TODO: disable interrupts
                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Di => {
                self.iff1 = false;
                self.iff2 = false;
    
                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Ei => {
                self.enable_interrupt = false;

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
                    Operand::Register16(register) => {
                        let value = self.registers.read_word(&register);
                        self.registers.write_word(&register, value.wrapping_add(1));
                    }
                    _ => {
                        let old_value = self.load_byte(memory, &destination);
                        self.set_flag(Flag::ParityOverflow, old_value == 0x7f);
                        let (value, _) = old_value.overflowing_add(1);
                        self.store_byte(memory, &destination, value);

                        self.set_flag(Flag::Sign, (value as i8) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (((old_value & 0xf) + 1) & 0x10) != 0);
                        self.set_flag(Flag::AddSubtract, false);
                    }
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
            Instruction::Ldd => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::DE), value);

                let address = self.registers.read_word(&Register16::DE).wrapping_sub(1);
                self.registers.write_word(&Register16::DE, address);

                let address = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, false);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Lddr => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::DE), value);

                let address = self.registers.read_word(&Register16::DE).wrapping_sub(1);
                self.registers.write_word(&Register16::DE, address);

                let address = self.registers.read_word(&Register16::HL).wrapping_sub(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, false);

                if counter == 0 {
                    self.registers.write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Ldi => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::DE), value);

                let address = self.registers.read_word(&Register16::DE).wrapping_add(1);
                self.registers.write_word(&Register16::DE, address);

                let address = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, false);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Ldir => {
                let value = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::DE), value);

                let address = self.registers.read_word(&Register16::DE).wrapping_add(1);
                self.registers.write_word(&Register16::DE, address);

                let address = self.registers.read_word(&Register16::HL).wrapping_add(1);
                self.registers.write_word(&Register16::HL, address);

                let counter = self.registers.read_word(&Register16::BC).wrapping_sub(1);
                self.registers.write_word(&Register16::BC, counter);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, counter != 0);
                self.set_flag(Flag::AddSubtract, false);

                if counter == 0 {
                    self.registers.write_word(&Register16::PC, next_address as u16);
                }
            }
            Instruction::Neg => {
                let right = self.registers.read_byte(&Register8::A);
                let (value, carry) = (0 as u8).overflowing_sub(right);
                let (_, overflow) = (0 as i8).overflowing_sub(right as i8);
                self.registers.write_byte(&Register8::A, value);

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, 0 < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, overflow);
                self.set_flag(Flag::AddSubtract, true);
                self.set_flag(Flag::Carry, carry);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Nop => {
                self.registers.write_word(&Register16::PC, next_address as u16);
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
            Instruction::Res(destination, Operand::Bit(bit), operand) => {
                let value = self.load_byte(memory, &operand) & (!(1 << bit));
                self.store_byte(memory, &destination, value); // copy for undocumented instructions
                self.store_byte(memory, &operand, value);

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
            Instruction::Rl(destination, operand) => {
                let value = self.load_byte(memory, &operand);
                let carry = value >> 7;
                let value = (value.rotate_left(1) & 0xfe) | carry;
                self.store_byte(memory, &destination, value); // copy for undocumented instructions
                self.store_byte(memory, &operand, value);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rla => {
                let value = self.registers.read_byte(&Register8::A);
                let carry = value >> 7;
                let value = (value.rotate_left(1) & 0xfe) | carry;
                self.registers.write_byte(&Register8::A, value);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rlc(destination, operand) => {
                let value = self.load_byte(memory, &operand).rotate_left(1);
                self.store_byte(memory, &destination, value); // copy for undocumented instructions
                self.store_byte(memory, &operand, value);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, (value & 1) != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rlca => {
                let value = self.registers.read_byte(&Register8::A).rotate_left(1);
                self.registers.write_byte(&Register8::A, value);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, (value & 1) != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rr(destination, operand) => {
                let value = self.load_byte(memory, &operand);
                let carry = value & 1;
                let value = (value.rotate_right(1) & 0x7f) | (carry << 7);
                self.store_byte(memory, &destination, value); // copy for undocumented instructions
                self.store_byte(memory, &operand, value);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rra => {
                let value = self.registers.read_byte(&Register8::A);
                let carry = value & 1;
                self.registers.write_byte(&Register8::A, (value.rotate_right(1) & 0x7f) | (carry << 7));

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rrc(destination, operand) => {
                let value = self.load_byte(memory, &operand).rotate_right(1);
                self.store_byte(memory, &destination, value); // copy for undocumented instructions
                self.store_byte(memory, &operand, value);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, (value & 0x80) != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rrca => {
                let value = self.registers.read_byte(&Register8::A).rotate_right(1);
                self.registers.write_byte(&Register8::A, value);

                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, (value & 0x80) != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rld => {
                let accumulator = self.registers.read_byte(&Register8::A);
                let memory_hl = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let new_accumulator = (accumulator & 0xf0) | (memory_hl >> 4);
                self.registers.write_byte(&Register8::A, new_accumulator);
                let new_memory_hl = (memory_hl << 4) | (accumulator & 0xf);
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::HL), new_memory_hl);

                self.set_flag(Flag::Sign, (new_accumulator as i8) < 0);
                self.set_flag(Flag::Zero, new_accumulator == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (new_accumulator.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Rrd => {
                let accumulator = self.registers.read_byte(&Register8::A);
                let memory_hl = self.load_byte(memory, &Operand::RegisterIndirect(Register16::HL));
                let new_accumulator = (accumulator & 0xf0) | (memory_hl & 0xf);
                self.registers.write_byte(&Register8::A, new_accumulator);
                let new_memory_hl = ((accumulator & 0xf) << 4) | (memory_hl >> 4);
                self.store_byte(memory, &Operand::RegisterIndirect(Register16::HL), new_memory_hl);

                self.set_flag(Flag::Sign, (new_accumulator as i8) < 0);
                self.set_flag(Flag::Zero, new_accumulator == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (new_accumulator.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Sbc(destination, source) => {
                match destination {
                    Operand::Register8(Register8::A) => {
                        let left = self.load_byte(memory, &destination) as u16;
                        let right = self.load_byte(memory, &source) as u16;
                        // let (value, carry1) = left.overflowing_sub(right);
                        // let (_, overflow1) = (left as i8).overflowing_sub(right as i8);
                        let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                        // let (value, carry2) = value.overflowing_sub(carry_value);
                        // let (value, carry) = left.overflowing_sub(right.wrapping_sub(carry_value));
                        let value = left.wrapping_sub(right).wrapping_sub(carry_value);
                        let carry = (value & 0x100) != 0;
                        // let (_, overflow2) = (value as i8).overflowing_sub(carry_value as i8);
                        let overflow = (left & 0x80) != (right & 0x80) && (right & 0x80) == (value & 0x80);
                        self.store_byte(memory, &destination, value as u8);

                        self.set_flag(Flag::Sign, (value as i8) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf) + carry_value);
                        self.set_flag(Flag::ParityOverflow, overflow);
                        self.set_flag(Flag::AddSubtract, true);
                        // self.set_flag(Flag::Carry, carry1 || carry2);
                        self.set_flag(Flag::Carry, carry);
                    }
                    Operand::Register16(Register16::HL) => {
                        let left = self.load_word(memory, &destination);
                        let right = self.load_word(memory, &source);
                        let (value, carry1) = left.overflowing_sub(right);
                        let (_, overflow1) = (left as i16).overflowing_sub(right as i16);
                        let carry_value = if self.check_flag(Flag::Carry) { 1 } else { 0 };
                        let (value, carry2) = value.overflowing_sub(carry_value);
                        let (_, overflow2) = (value as i16).overflowing_sub(carry_value as i16);
                        self.store_word(memory, &destination, value);

                        self.set_flag(Flag::Sign, (value as i16) < 0);
                        self.set_flag(Flag::Zero, value == 0);
                        self.set_flag(Flag::HalfCarry, (left & 0xfff) < (right & 0xfff));
                        self.set_flag(Flag::ParityOverflow, overflow1 || overflow2);
                        self.set_flag(Flag::AddSubtract, true);
                        self.set_flag(Flag::Carry, carry1 || carry2);
                    }
                    _ => unreachable!(),
                }

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Scf => {
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, true);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Set(destination, Operand::Bit(bit), operand) => {
                let value = self.load_byte(memory, &operand) | (1 << bit);
                self.store_byte(memory, &destination, value); // copy for undocumented instructions
                self.store_byte(memory, &operand, value);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Sla(destination, operand) => {
                let value = self.load_byte(memory, &operand);
                let carry = value >> 7;
                let value = value << 1;
                self.store_byte(memory, &destination, value); // copy for undocumented instructions
                self.store_byte(memory, &operand, value);

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Sll(destination, operand) => {
                let value = self.load_byte(memory, &operand);
                let carry = value >> 7;
                let value = value << 1;
                self.store_byte(memory, &destination, value); // copy for undocumented instructions
                self.store_byte(memory, &operand, value);

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Sra(destination, operand) => {
                let value = self.load_byte(memory, &operand);
                let sign = value >> 7;
                let carry = value & 1;
                let value = sign | (value >> 1);
                self.store_byte(memory, &destination, value); // copy for undocumented instructions
                self.store_byte(memory, &operand, value);

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Srl(destination, operand) => {
                let value = self.load_byte(memory, &operand);
                let carry = value & 1;
                let value = value >> 1;
                self.store_byte(memory, &destination, value); // copy for undocumented instructions
                self.store_byte(memory, &operand, value);

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (value.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, carry != 0);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Sub(operand) => { // ALUOP / checked
                let left = self.registers.read_byte(&Register8::A);
                let right = self.load_byte(memory, &operand);
                let (value, carry) = left.overflowing_sub(right);
                let (_, overflow) = (left as i8).overflowing_sub(right as i8);
                self.registers.write_byte(&Register8::A, value);

                self.set_flag(Flag::Sign, (value as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, value == 0);
                self.set_flag(Flag::HalfCarry, (left & 0xf) < (right & 0xf));
                self.set_flag(Flag::ParityOverflow, overflow);
                self.set_flag(Flag::AddSubtract, true);
                self.set_flag(Flag::Carry, carry);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            Instruction::Xor(operand) => {
                let value = self.load_byte(memory, &operand);

                let result = self.registers.read_byte(&Register8::A) ^ value;
                self.registers.write_byte(&Register8::A, result);

                self.set_flag(Flag::Sign, (result as i8) < 0); // TODO: make this reusable?
                self.set_flag(Flag::Zero, result == 0);
                self.set_flag(Flag::HalfCarry, false);
                self.set_flag(Flag::ParityOverflow, (result.count_ones() & 1) == 0);
                self.set_flag(Flag::AddSubtract, false);
                self.set_flag(Flag::Carry, false);

                self.registers.write_word(&Register16::PC, next_address as u16);
            }
            _ => {
                println!("{:#06x}: {}", self.registers.read_word(&Register16::PC), &instruction);
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

    fn print_state(&self) {
        let ix = self.registers.read_word(&Register16::IX);
        let iy = self.registers.read_word(&Register16::IY);
        let hl = self.registers.read_word(&Register16::HL);
        let de = self.registers.read_word(&Register16::DE);
        let bc = self.registers.read_word(&Register16::BC);
        let af = self.registers.read_word(&Register16::AF);
        let sp = self.registers.read_word(&Register16::SP);
        println!(
            "IX = {:#06x}, IY = {:#06x}, HL = {:#06x}, DE = {:#06x}, BC = {:#06x}, AF = {:#06x}, SP = {:#06x}",
            ix, iy, hl, de, bc, af, sp
        );
        
        let s = self.check_flag(Flag::Sign);
        let z = self.check_flag(Flag::Zero);
        let h = self.check_flag(Flag::HalfCarry);
        let pv = self.check_flag(Flag::ParityOverflow);
        let n = self.check_flag(Flag::AddSubtract);
        let c = self.check_flag(Flag::Carry);
        println!("S = {}, Z = {}, H = {}, P/V = {}, N = {}, C = {}", s, z, h, pv, n, c);
    }
}

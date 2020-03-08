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

    pub fn read_byte(&self, register: Register) -> u8 {
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

    fn write_byte(&mut self, register: Register, value: u8) { // TODO: add tests
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

    pub fn read_word(&self, register: Register) -> u16 {
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

    fn write_word(&mut self, register: Register, value: u16) {
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
        registers.write_word(Register::PC, initial_pc);

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
            memory, self.registers.read_word(Register::PC) as usize
        );

        println!("{:#06x}: {}", self.registers.read_word(Register::PC), &instruction);
        match instruction {
            Instruction::Call(jump_test, Operand::Immediate16(address)) => {
                if self.check_jump(jump_test) {
                    let new_sp = self.registers.read_word(Register::SP) - 2;
                    self.registers.write_word(Register::SP, new_sp);
                    memory.write_word(new_sp as usize, next_address as u16);
                    self.registers.write_word(Register::PC, address);
                } else {
                    self.registers.write_word(Register::PC, next_address as u16);
                }

            }
            Instruction::Jp(jump_test, target) => {
                if self.check_jump(jump_test) {
                    let address = match target {
                        Operand::Immediate16(address) => address,
                        _ => {
                            unimplemented!();
                        }
                    };
                    self.registers.write_word(Register::PC, address);
                } else {
                    self.registers.write_word(Register::PC, next_address as u16);
                }
            }
            Instruction::Ld8(destination, source) => {
                let value = match source {
                    Operand::Immediate8(value) => value,
                    _ => {
                        unimplemented!();
                    }
                };

                match destination {
                    Operand::Register(register) => self.registers.write_byte(register, value),
                    _ => {
                        unimplemented!();
                    }
                }

                self.registers.write_word(Register::PC, next_address as u16);
            }
            Instruction::Ld16(destination, source) => {
                let value = match source {
                    Operand::Direct16(address) => memory.read_word(address as usize),
                    Operand::Register(register) => self.registers.read_word(register),
                    Operand::Immediate16(value) => value,
                    _ => {
                        unimplemented!();
                    }
                };

                match destination {
                    Operand::Register(register) => self.registers.write_word(register, value),
                    _ => {
                        unimplemented!();
                    }
                }

                self.registers.write_word(Register::PC, next_address as u16);
            }
            Instruction::Push(Operand::Register(source)) => {
                let new_sp = self.registers.read_word(Register::SP) - 2;
                self.registers.write_word(Register::SP, new_sp);
                memory.write_word(new_sp as usize, self.registers.read_word(source));
                self.registers.write_word(Register::PC, next_address as u16);
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
        let flags = self.registers.read_byte(Register::F);

        flags & flag.mask() != 0
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
}

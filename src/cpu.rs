use crate::memory;
use crate::instruction::{ Decoder, Instruction };

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

struct RegisterFile {
    data: Vec<u16>,
}

impl RegisterFile {
    fn new() -> RegisterFile {
        RegisterFile { data: vec![0; 14] }
    }

    fn read_byte(self, register: Register) -> u8 {
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

    fn write_byte(&mut self, register: Register, value: u8) {
        let value = value as u16;

        match register {
            Register::A => { self.data[0] = value << 8 + self.data[0] & 0xff; }
            Register::F => { self.data[0] = self.data[0] & 0xff00 + value; }
            Register::B => { self.data[1] = value << 8 + self.data[1] & 0xff; }
            Register::C => { self.data[1] = self.data[1] & 0xff00 + value; }
            Register::D => { self.data[2] = value << 8 + self.data[2] & 0xff; }
            Register::E => { self.data[2] = self.data[2] & 0xff00 + value; }
            Register::H => { self.data[3] = value << 8 + self.data[3] & 0xff; }
            Register::L => { self.data[3] = self.data[3] & 0xff00 + value; }
            Register::I => { self.data[8] = self.data[8] & 0xff00 + value; }
            Register::R => { self.data[9] = self.data[9] & 0xff00 + value; }
            Register::IXH => { self.data[10] = value << 8 + self.data[0] & 0xff; }
            Register::IXL => { self.data[10] = self.data[10] & 0xff00 + value; }
            Register::IYH => { self.data[11] = value << 8 + self.data[11] & 0xff; }
            Register::IYL => { self.data[11] = self.data[11] & 0xff00 + value; }
            _ => unreachable!(),
        }
    }

    fn read_word(self, register: Register) -> u16 {
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

pub struct CPU<'m, M> {
    pub memory: M,
    decoder: Decoder<'m, M>,
    registers: RegisterFile,
}

impl<'m, M> CPU<'m, M>
where M: memory::Read + memory::Write
{
    pub fn new(memory: M, initial_pc: u16) -> CPU<'m, M> {
        let decoder = Decoder::new(&memory);
        let registers = RegisterFile::new();

        let mut cpu = CPU {
            memory,
            decoder,
            registers,
        };
        
        cpu
    }

    pub fn fetch_and_execute(&mut self) -> bool {
        let (instruction, next_address) = self.decoder.decode_at(self.registers.read_word(Register::PC) as usize);

        match instruction {
            Instruction::Halt => false,
            _ => unimplemented!(),
        }
    }
}

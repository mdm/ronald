use crate::memory;
use memory::{ Read, Write };
use crate::cpu;

pub trait System {
    fn emulate(&mut self, time_limit: Option<u64>);
}


pub struct ZexHarness {
    memory: memory::RAM,
    cpu: cpu::CPU,
}

impl ZexHarness {
    pub fn new(rom_path: &str) -> ZexHarness {
        let mut memory = memory::RAM::from_file(0x10000, rom_path, 0x100);
        memory.write_byte(0x0005, 0xc9); // patch with RET instruction
        memory.write_word(0x0006, 0xe400); // patch with initial SP

        ZexHarness {
            memory: memory,
            cpu: cpu::CPU::new(0x100),
        }
    }
}

impl System for ZexHarness {
    fn emulate(&mut self, _time_limit: Option<u64>) {
        loop {
            match self.cpu.registers.read_word(&cpu::Register16::PC) {
                0x0000 => break,
                0x0005 => {
                    match self.cpu.registers.read_byte(&cpu::Register8::C) {
                        2 => print!("{}", self.cpu.registers.read_byte(&cpu::Register8::E) as char),
                        9 => {
                            let mut address = self.cpu.registers.read_word(&cpu::Register16::DE) as usize;
                            loop {
                                let character = self.memory.read_byte(address) as char;
                                if character == '$' {
                                    break;
                                }
                                else {
                                    print!("{}", character);
                                }
                                address += 1;
                            }
                        }
                        _ => unreachable!(),
                    }
                    self.cpu.fetch_and_execute(&mut self.memory);
                }
                _ => self.cpu.fetch_and_execute(&mut self.memory),
            }
        }
    }
}


// struct CPC464 {

// }

// impl system for CPC464 {

// }

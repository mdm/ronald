use crate::bus;
use crate::cpu;
use crate::crtc;
use crate::fdc;
use crate::gate_array;
use crate::memory;
use memory::{ Read, Write };
use crate::pio;
use crate::screen;


pub struct ZexHarness {
    cpu: cpu::CPU,
    memory: memory::RAM,
}

impl ZexHarness {
    pub fn new(rom_path: &str) -> ZexHarness {
        let mut memory = memory::RAM::from_file(0x10000, rom_path, 0x100);
        memory.write_byte(0x0005, 0xc9); // patch with RET instruction
        memory.write_word(0x0006, 0xe400); // patch with initial SP

        ZexHarness {
            cpu: cpu::CPU::new(0x100),
            memory: memory,
        }
    }

    pub fn emulate(&mut self) {
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
        println!();
    }
}

pub trait System {
    fn emulate(&mut self, time_limit: Option<u64>);
    fn get_frame_buffer(&self) -> &Vec<u32>;
}

pub struct CPC464 {
    cpu: cpu::CPU,
    memory: memory::Memory,
    bus: bus::Bus,
    screen: screen::Screen,
}

impl CPC464 {
    pub fn new() -> CPC464 {
        CPC464 {
            cpu: cpu::CPU::new(0),
            memory: memory::Memory::new(),
            bus: bus::Bus::new(
                crtc::CRTController::new(),
                fdc::FloppyDiskController::new(),
                gate_array::GateArray::new(),
                pio::PeripheralInterface::new()),
            screen: screen::Screen::new(),
        }
    }
}

impl System for CPC464 {
    fn emulate(&mut self, _time_limit: Option<u64>) {
        let cycles = self.cpu.fetch_and_execute(&mut self.memory);
        
        self.bus.step(&mut self.memory); // TODO: step multiple times depending on "cycles" and handle interrupts
    }

    fn get_frame_buffer(&self) -> &Vec<u32> {
        self.screen.get_frame_buffer()
    }
}

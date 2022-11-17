use crate::bus::{DummyBus, StandardBus};
use crate::cpu::{Cpu, Register8, Register16};
use crate::debugger::Debugger;
use crate::keyboard::Keyboard;
use crate::memory::{Ram, Memory, Read, Write};
use crate::screen::Screen;

pub struct ZexHarness {
    cpu: Cpu,
    memory: Ram,
    bus: DummyBus,
}

impl ZexHarness {
    pub fn new(rom_path: &str) -> ZexHarness {
        let mut memory = Ram::from_file(0x10000, rom_path, 0x100);
        memory.write_byte(0x0005, 0xc9); // patch with RET instruction
        memory.write_word(0x0006, 0xe400); // patch with initial SP

        ZexHarness {
            cpu: Cpu::new(0x100),
            memory,
            bus: DummyBus::new(),
        }
    }

    pub fn emulate(&mut self) {
        loop {
            match self.cpu.registers.read_word(&Register16::PC) {
                0x0000 => break,
                0x0005 => {
                    match self.cpu.registers.read_byte(&Register8::C) {
                        2 => print!(
                            "{}",
                            self.cpu.registers.read_byte(&Register8::E) as char
                        ),
                        9 => {
                            let mut address =
                                self.cpu.registers.read_word(&Register16::DE) as usize;
                            loop {
                                let character = self.memory.read_byte(address) as char;
                                if character == '$' {
                                    break;
                                } else {
                                    print!("{}", character);
                                }
                                address += 1;
                            }
                        }
                        _ => unreachable!(),
                    }
                    self.cpu.fetch_and_execute(&mut self.memory, &mut self.bus);
                }
                _ => {
                    self.cpu.fetch_and_execute(&mut self.memory, &mut self.bus);
                }
            }
        }
        println!();
    }
}

pub trait System {
    fn emulate(&mut self) -> u8;
    fn get_screen(&self) -> &Screen;
    fn get_keyboard(&mut self) -> &mut Keyboard;
    fn activate_debugger(&mut self);
    fn load_disk(&mut self, drive: usize, filename: &str);
}

pub struct CPC464 {
    cpu: Cpu,
    memory: Memory,
    bus: StandardBus,
    debugger: Debugger,
}

impl CPC464 {
    pub fn new() -> CPC464 {
        let cpu = Cpu::new(0);
        let memory = Memory::new();
        let bus = StandardBus::new();
        let debugger = Debugger::new();

        CPC464 {
            cpu,
            memory,
            bus,
            debugger,
        }
    }
}

impl System for CPC464 {
    fn emulate(&mut self) -> u8 {
        if self.debugger.is_active(&self.cpu) {
            self.debugger.run_command_shell(&mut self.cpu, &self.memory);
        }

        let (cycles, interrupt_acknowledged) = self.cpu.fetch_and_execute(&mut self.memory, &mut self.bus);

        for _ in 0..cycles {
            let interrupt = self.bus.step(&mut self.memory);
            if interrupt {
                self.cpu.request_interrupt();
            }
        }

        if interrupt_acknowledged {
            // TODO: communicate with gate array directly?
            // What about external hardware triggering (non-maskable) interrupts?
            self.bus.acknowledge_interrupt();
        }

        cycles
    }

    fn get_screen(&self) -> &Screen {
        // TODO: should the GUI own the frame buffer?
        self.bus.get_screen()
    }

    fn get_keyboard(&mut self) -> &mut Keyboard {
        self.bus.get_keyboard()
    }

    fn activate_debugger(&mut self) {
        self.debugger.activate();
    }

    fn load_disk(&mut self, drive: usize, filename: &str) {
        self.bus.load_disk(drive, filename);
    }
}

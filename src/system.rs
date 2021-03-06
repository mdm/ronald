use crate::bus;
use crate::cpu;
use crate::crtc;
use crate::debugger;
use crate::fdc;
use crate::gate_array;
use crate::keyboard;
use crate::memory;
use crate::ppi;
use crate::psg;
use crate::screen;
use crate::tape;
use memory::{Read, Write};

use std::cell::RefCell;
use std::rc::Rc;

pub struct ZexHarness {
    cpu: cpu::CPUShared<memory::RAM, bus::DummyBus>,
    memory: Rc<RefCell<memory::RAM>>,
}

impl ZexHarness {
    pub fn new(rom_path: &str) -> ZexHarness {
        let mut memory = memory::RAM::from_file(0x10000, rom_path, 0x100);
        memory.write_byte(0x0005, 0xc9); // patch with RET instruction
        memory.write_word(0x0006, 0xe400); // patch with initial SP

        let memory_shared = Rc::new(RefCell::new(memory));
        let bus_shared = Rc::new(RefCell::new(bus::DummyBus::new()));

        ZexHarness {
            cpu: cpu::CPU::new_shared(memory_shared.clone(), bus_shared, 0x100),
            memory: memory_shared,
        }
    }

    pub fn emulate(&mut self) {
        loop {
            match self.cpu.borrow().registers.read_word(&cpu::Register16::PC) {
                0x0000 => break,
                0x0005 => {
                    match self.cpu.borrow().registers.read_byte(&cpu::Register8::C) {
                        2 => print!(
                            "{}",
                            self.cpu.borrow().registers.read_byte(&cpu::Register8::E) as char
                        ),
                        9 => {
                            let mut address =
                                self.cpu.borrow().registers.read_word(&cpu::Register16::DE) as usize;
                            loop {
                                let character = self.memory.borrow().read_byte(address) as char;
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
                    self.cpu.borrow_mut().fetch_and_execute();
                }
                _ => {
                    self.cpu.borrow_mut().fetch_and_execute();
                }
            }
        }
        println!();
    }
}

pub trait System {
    fn emulate(&mut self) -> u8;
    fn get_screen(&self) -> screen::ScreenShared;
    fn get_keyboard(&self) -> keyboard::KeyboardShared;
    fn activate_debugger(&mut self);
}

pub struct CPC464 {
    cpu: cpu::CPUShared<memory::Memory, bus::StandardBus>,
    bus: bus::StandardBusShared,
    screen: screen::ScreenShared,
    keyboard: keyboard::KeyboardShared,
    debugger: debugger::Debugger<memory::Memory, bus::StandardBus>,
}

impl CPC464 {
    pub fn new() -> CPC464 {
        // TODO: receive shared screen here
        let memory = memory::Memory::new_shared();
        let crtc = crtc::CRTController::new_shared();
        let keyboard = keyboard::Keyboard::new_shared();
        let psg = psg::SoundGenerator::new_shared(keyboard.clone());
        let screen = screen::Screen::new_shared();
        let tape = tape::TapeController::new_shared();
        let bus = bus::StandardBus::new_shared(
            crtc.clone(),
            fdc::FloppyDiskController::new_shared(),
            gate_array::GateArray::new_shared(memory.clone(), crtc.clone(), screen.clone()),
            memory.clone(),
            ppi::PeripheralInterface::new_shared(crtc, keyboard.clone(), psg, tape),
        );
        let cpu = cpu::CPU::new_shared(memory, bus.clone(), 0);
        let debugger = debugger::Debugger::new_shared(cpu.clone());

        CPC464 {
            cpu,
            bus,
            screen,
            keyboard,
            debugger,
        }
    }
}

impl System for CPC464 {
    fn emulate(&mut self) -> u8 {
        if self.debugger.is_active() {
            self.debugger.run_command_shell();
        }

        let (cycles, interrupt_acknowledged) = self.cpu.borrow_mut().fetch_and_execute();

        for _ in 0..cycles {
            let interrupt = self.bus.borrow_mut().step();
            if interrupt {
                self.cpu.borrow_mut().request_interrupt();
            }
        }

        if interrupt_acknowledged {
            // TODO: communicate with gate array directly?
            // What about external hardware triggering (non-maskable) interrupts?
            self.bus.borrow_mut().acknowledge_interrupt();
        }

        cycles
    }

    fn get_screen(&self) -> screen::ScreenShared {
        // TODO: should the GUI own the frame buffer?
        self.screen.clone()
    }

    fn get_keyboard(&self) -> keyboard::KeyboardShared {
        self.keyboard.clone()
    }

    fn activate_debugger(&mut self) {
        self.debugger.activate();
    }
}

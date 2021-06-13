use crate::bus;
use crate::cpu;
use crate::crtc;
use crate::fdc;
use crate::gate_array;
use crate::keyboard;
use crate::memory;
use memory::{ Read, Write };
use crate::ppi;
use crate::screen;

use std::cell::RefCell;
use std::rc::Rc;


pub struct ZexHarness {
    cpu: cpu::CPU<memory::RAM, bus::DummyBus>,
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
            cpu: cpu::CPU::new(memory_shared.clone(), bus_shared.clone(), 0x100),
            memory: memory_shared.clone(),
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
                                let character = self.memory.borrow().read_byte(address) as char;
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
                    self.cpu.fetch_and_execute();
                }
                _ => {
                    self.cpu.fetch_and_execute();
                }
            }
        }
        println!();
    }
}

pub trait System {
    fn emulate(&mut self, time_limit: Option<u64>);
    fn get_frame_buffer(&self) -> &Vec<u32>;
    fn get_keyboard(&self) -> Rc<RefCell<keyboard::Keyboard>>;
}

pub struct CPC464 {
    cpu: cpu::CPU<memory::Memory, bus::StandardBus>,
    bus: bus::StandardBusShared,
    screen: screen::Screen,
    keyboard: keyboard::KeyboardShared,
}

impl CPC464 {
    pub fn new() -> CPC464 { // TODO: receive shared screen here
        let memory = memory::Memory::new_shared();
        let crtc = crtc::CRTController::new_shared();
        let bus = bus::StandardBus::new_shared(
            crtc.clone(),
            fdc::FloppyDiskController::new_shared(),
            gate_array::GateArray::new_shared(memory.clone(), crtc.clone()),
            ppi::PeripheralInterface::new_shared()
        );

        CPC464 {
            cpu: cpu::CPU::new(memory.clone(), bus.clone(), 0),
            bus: bus.clone(),
            screen: screen::Screen::new(),
            keyboard: keyboard::Keyboard::new_shared(),
        }
    }
}

impl System for CPC464 {
    fn emulate(&mut self, time_limit: Option<u64>) {
        let (cycles, interrupt_acknowledged) = self.cpu.fetch_and_execute();

        for _ in 0..cycles {
            let interrupt  = self.bus.borrow_mut().step();
            if interrupt {
                // TODO: generate interrupt
            }
        }

        if interrupt_acknowledged {
            // TODO: communicate with gate array directly?
            // What about external hardware triggering (non-maskable) interrupts?
            self.bus.borrow_mut().acknowledge_interrupt();
        }
    }

    fn get_frame_buffer(&self) -> &Vec<u32> { // TODO: should the GUI own the buffer?
        self.screen.get_frame_buffer()
    }

    fn get_keyboard(&self) -> keyboard::KeyboardShared {
        self.keyboard.clone()
    }
}

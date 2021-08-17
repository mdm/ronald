use std::cell::RefCell;
use std::rc::Rc;

use crate::keyboard;

pub type SoundGeneratorShared = Rc<RefCell<SoundGenerator>>;

pub struct SoundGenerator {
    keyboard: keyboard::KeyboardShared,
    buffer: u8,
    selected_register: u8,
}

impl SoundGenerator {
    pub fn new_shared(keyboard: keyboard::KeyboardShared) -> SoundGeneratorShared {
        let psg = SoundGenerator {
            keyboard,
            buffer: 0,
            selected_register: 0,
        };

        Rc::new(RefCell::new(psg))
    }

    pub fn perform_function(&mut self, function: u8) {
        match function {
            0 => (), // inactive
            1 => {
                match self.selected_register {
                    0x0e => {
                        self.buffer = self.keyboard.borrow().scan_active_line();
                    }
                    _ => unimplemented!(),
                }
            },
            2 => {
                log::trace!("Writing to PSG register {:#04x}: {:#04x}", self.selected_register, self.buffer); // TODO: process data
            },
            3 => {
                self.selected_register = self.buffer;
            }
            _ => unimplemented!(),
        }
    }

    pub fn read_byte(&self) -> u8 {
        self.buffer
    }

    pub fn write_byte(&mut self, value: u8) {
        self.buffer = value;
    }
}

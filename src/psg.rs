use std::cell::RefCell;
use std::rc::Rc;

pub type SoundGeneratorShared = Rc<RefCell<SoundGenerator>>;

pub struct SoundGenerator {
    buffer: u8,
    selected_register: u8,
}

impl SoundGenerator {
    pub fn new_shared() -> SoundGeneratorShared {
        let psg = SoundGenerator {
            buffer: 0,
            selected_register: 0,
        };

        Rc::new(RefCell::new(psg))
    }

    pub fn perform_function(&mut self, function: u8) {
        match function {
            0 => (), // inactive
            1 => unimplemented!(),
            2 => {
                // println!("PSG {:#04x}: {:#04x}", self.selected_register, self.buffer); // TODO: process data
            },
            3 => {
                self.selected_register = self.buffer;
            }
            _ => unimplemented!(),
        }
    }

    pub fn read_byte(&self) -> u8 {
        unimplemented!()
    }

    pub fn write_byte(&mut self, value: u8) {
        self.buffer = value;
    }
}

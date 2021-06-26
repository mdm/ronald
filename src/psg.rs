use std::cell::RefCell;
use std::rc::Rc;

pub type SoundGeneratorShared = Rc<RefCell<SoundGenerator>>;

pub struct SoundGenerator {
    buffer: u8,
}

impl SoundGenerator {
    pub fn new_shared() -> SoundGeneratorShared {
        let psg = SoundGenerator {
            buffer: 0,
        };

        Rc::new(RefCell::new(psg))
    }

    pub fn perform_function(&mut self, function: u8) {
        match function {
            0 => (), // inactive
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

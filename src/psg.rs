use std::cell::RefCell;
use std::rc::Rc;

pub type SoundGeneratorShared = Rc<RefCell<SoundGenerator>>;

pub struct SoundGenerator {}

impl SoundGenerator {
    pub fn new_shared() -> SoundGeneratorShared {
        let psg = SoundGenerator {};

        Rc::new(RefCell::new(psg))
    }

    pub fn perform_function(&mut self, function: u8) {
        unimplemented!()
    }

    pub fn read_byte(&self) -> u8 {
        unimplemented!()
    }

    pub fn write_byte(&mut self, value: u8) {
        unimplemented!()
    }
}

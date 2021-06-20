use std::cell::RefCell;
use std::rc::Rc;

pub type FloppyDiskControllerShared = Rc<RefCell<FloppyDiskController>>;

pub struct FloppyDiskController {}

impl FloppyDiskController {
    pub fn new_shared() -> FloppyDiskControllerShared {
        let fdc = FloppyDiskController {};

        Rc::new(RefCell::new(fdc))
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        unimplemented!()
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        unimplemented!()
    }
}

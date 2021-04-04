use std::cell::RefCell;
use std::rc::Rc;

pub type PeripheralInterfaceShared = Rc<RefCell<PeripheralInterface>>;

pub struct PeripheralInterface { // Peripheral input/output

}

impl PeripheralInterface {
    pub fn new_shared() -> PeripheralInterfaceShared {
        let pio = PeripheralInterface {};

        Rc::new(RefCell::new(pio))
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        unimplemented!()
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        unimplemented!()
    }
}
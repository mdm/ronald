use std::cell::RefCell;
use std::rc::Rc;

pub type PeripheralInterfaceShared = Rc<RefCell<PeripheralInterface>>;

pub struct PeripheralInterface { // Peripheral input/output

}

impl PeripheralInterface {
    pub fn new_shared() -> PeripheralInterfaceShared {
        let ppi = PeripheralInterface {};

        Rc::new(RefCell::new(ppi))
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        println!("PPI (read) {:#06x}", port);
        unimplemented!()
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        println!("PPI (write) {:#06x} {:#10b}", port, value);
        unimplemented!();        
    }
}
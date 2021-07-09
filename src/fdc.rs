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
        match port {
            // 0xfb7e => 0x80, // TODO: read main status register
            0xfb7e => 0x00,
            _ => {
                println!("FDC read {:#06x}", port);
                unimplemented!()
            }
        }
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        match port {
            0xfa7e => {
                match value {
                    0 => (), // TODO: switch motor off
                    _ => {
                        println!("FDC write {:#06x} {:#010b}", port, value);
                        unimplemented!()
                    }
                }
            }
            _ => {
                println!("FDC write {:#06x} {:#010b}", port, value);
                unimplemented!()
            }
        }
    }
}

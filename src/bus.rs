pub trait Device {
    fn read_byte(&self, port: u16) -> Option<u8>;
    fn write_byte(&mut self, port: u16, value: u8);
}

pub struct Bus {
    devices: Vec<Box<dyn Device>>
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            devices: vec!()
        }
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        0
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {

    }
}

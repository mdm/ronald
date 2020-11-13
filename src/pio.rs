pub struct PeripheralInterface { // Peripheral input/output

}

impl PeripheralInterface {
    pub fn new() -> PeripheralInterface {
        PeripheralInterface {}
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        unimplemented!()
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        unimplemented!()
    }
}
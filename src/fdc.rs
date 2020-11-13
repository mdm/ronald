pub struct FloppyDiskController {

}

impl FloppyDiskController {
    pub fn new() -> FloppyDiskController {
        FloppyDiskController {}
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        unimplemented!()
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        unimplemented!()
    }
}
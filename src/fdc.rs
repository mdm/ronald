use std::cell::RefCell;
use std::rc::Rc;

use crate::dsk_file::{self, Disk};

pub type FloppyDiskControllerShared = Rc<RefCell<FloppyDiskController>>;

pub struct FloppyDiskController {
    disk: Option<Disk>,
    buffer: Vec<u8>,
    phase: Phase,
    expected_bytes: Option<u8>,
    motor_on: bool,
}

impl FloppyDiskController {
    pub fn new_shared() -> FloppyDiskControllerShared {
        let fdc = FloppyDiskController {
            disk: None,
            buffer: Vec::new(),
            phase: Phase::Command,
            expected_bytes: None,
            motor_on: false,
        };

        Rc::new(RefCell::new(fdc))
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        match port {
            0xfb7e => 0x80, // always report ready -> TODO: read main status register
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
                    0 => {
                        self.motor_on = false;
                    }
                    1 => {
                        self.motor_on = true;
                    }
                    _ => unreachable!(),
                }
            }
            0xfb7f => {
                match self.phase {
                    Phase::Command => {
                        match self.expected_bytes {
                            Some(expected_bytes) => {
                                self.buffer.push(value);

                                if expected_bytes > 1 {
                                    self.expected_bytes = Some(expected_bytes - 1);
                                } else {
                                    self.execute_command();
                                }
                            }
                            None => {
                                self.buffer.push(value);

                                let expected_bytes = match value & 0x1f {
                                    0x02 => 8,
                                    0x03 => 2,
                                    0x04 => 1,
                                    0x05 => 8,
                                    0x06 => 8,
                                    0x07 => 1,
                                    0x08 => 0,
                                    0x09 => 8,
                                    0x0a => 1,
                                    0x0c => 8,
                                    0x0d => 5,
                                    0x0f => 2,
                                    0x11 => 8,
                                    0x19 => 8,
                                    0x1d => 8,
                                    _ => unreachable!(),
                                };

                                if expected_bytes == 0 {
                                    self.execute_command();
                                } else {
                                    self.expected_bytes = Some(expected_bytes);
                                }
                            }
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            _ => {
                println!("FDC write {:#06x} {:#010b}", port, value);
                unimplemented!()
            }
        }
    }

    pub fn load_disk(&mut self, filename: &str) {
        self.disk = match dsk_file::Disk::load(filename) {
            Ok(disk) => {
                println!("LOAD OK"); // TODO: use logger
                Some(disk)
            }
            Err(error) => {
                println!("LOAD ERROR: {}", error); // TODO: use logger
                None
            }
        }
    }

    fn execute_command(&mut self) {
        dbg!(&self.buffer);
        self.expected_bytes = None;
        // self.phase = Phase::Execution;

        self.buffer.clear();
        // unimplemented!();
    }
}

enum Phase {
    Command,
    Execution,
    Result,
}
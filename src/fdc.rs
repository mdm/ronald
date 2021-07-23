use std::{cell::RefCell, collections::VecDeque};
use std::rc::Rc;

use crate::dsk_file::{self, Disk};

pub type FloppyDiskControllerShared = Rc<RefCell<FloppyDiskController>>;

struct Drive {
    track: u8,
    disk: Option<Disk>,
}

enum Command {
    ReadTrack,
    Specify,
    SenseDriveState,
    WriteSector,
    ReadSector,
    Recalibrate,
    SenseInterruptState,
    WriteDeletedSector,
    ReadSectorId,
    ReadDeletedSector,
    FormatTrack,
    Seek,
    ScanEqual,
    ScanLowOrEqual,
    ScanHighOrEqual,
}

impl Command {
    fn from_byte(byte: u8) -> Command {
        match byte & 0x1f {
            0x02 => Command::ReadTrack,
            0x03 => Command::Specify,
            0x04 => Command::SenseDriveState,
            0x05 => Command::WriteSector,
            0x06 => Command::ReadSector,
            0x07 => Command::Recalibrate,
            0x08 => Command::SenseInterruptState,
            0x09 => Command::WriteDeletedSector,
            0x0a => Command::ReadSectorId,
            0x0c => Command::ReadDeletedSector,
            0x0d => Command::FormatTrack,
            0x0f => Command::Seek,
            0x11 => Command::ScanEqual,
            0x19 => Command::ScanLowOrEqual,
            0x1d => Command::ScanHighOrEqual,
            _ => unreachable!(),
        }
    }

    fn expected_parameter_bytes(&self) -> usize {
        match self {
            Command::ReadTrack => 8,
            Command::Specify => 2,
            Command::SenseDriveState => 1,
            Command::WriteSector => 8,
            Command::ReadSector => 8,
            Command::Recalibrate => 1,
            Command::SenseInterruptState => 0,
            Command::WriteDeletedSector => 8,
            Command::ReadSectorId => 1,
            Command::ReadDeletedSector => 8,
            Command::FormatTrack => 5,
            Command::Seek => 2,
            Command::ScanEqual => 8,
            Command::ScanLowOrEqual => 8,
            Command::ScanHighOrEqual => 8,
        }
    }
}

pub struct FloppyDiskController {
    drives: [Drive; 2],
    phase: Phase,
    command: Option<Command>,
    parameters_buffer: Vec<u8>,
    data_buffer: VecDeque<u8>,
    result_buffer: VecDeque<u8>,
    motors_on: bool,
    request_for_master: bool,
    data_input_output: bool,
    execution_mode: bool,
    floppy_controller_busy: bool,
    floppy_drive_busy: [bool; 2],
}

impl FloppyDiskController {
    pub fn new_shared() -> FloppyDiskControllerShared {
        let fdc = FloppyDiskController {
            drives: [
                Drive {
                    track: 0,
                    disk: None,
                },
                Drive {
                    track: 0,
                    disk: None,
                },
            ],
            phase: Phase::Command,
            command: None,
            parameters_buffer: Vec::new(),
            data_buffer: VecDeque::new(),
            result_buffer: VecDeque::new(),
            motors_on: false,
            request_for_master: true,
            data_input_output: false,
            execution_mode: false,
            floppy_controller_busy: false,
            floppy_drive_busy: [false; 2],
        };

        Rc::new(RefCell::new(fdc))
    }

    pub fn read_byte(&mut self, port: u16) -> u8 {
        match port {
            0xfb7e => self.report_main_status_register(),
            0xfb7f => {
                match self.phase {
                    Phase::Result => {
                        let result = if let Some(result) = self.result_buffer.pop_front() {
                            result
                        } else {
                            unreachable!()
                        };

                        if self.result_buffer.is_empty() {
                            self.floppy_controller_busy = false;
                        }

                        result
                    }
                    _ => {
                        println!("FDC read {:#06x}", port);
                        unimplemented!()
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        match port {
            0xfa7e => {
                match value {
                    0 => {
                        self.motors_on = false;
                    }
                    1 => {
                        self.motors_on = true;
                    }
                    _ => unreachable!(),
                }
            }
            0xfb7f => {
                match self.phase {
                    Phase::Command => {
                        match &self.command {
                            Some(command) => {
                                if self.parameters_buffer.len() < command.expected_parameter_bytes() {
                                    self.parameters_buffer.push(value);
                                }

                                if self.parameters_buffer.len() == command.expected_parameter_bytes() {
                                    self.execute_command();
                                }
                            }
                            None => {
                                self.command = Some(Command::from_byte(value));
                                self.floppy_controller_busy = true;

                                if self.parameters_buffer.len() == self.command.as_ref().unwrap().expected_parameter_bytes() {
                                    self.execute_command();
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

    pub fn load_disk(&mut self, drive: usize, filename: &str) {
        self.drives[drive].disk = match dsk_file::Disk::load(filename) {
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
        dbg!(&self.parameters_buffer);
        self.phase = Phase::Execution;

        if let Some(command) = &self.command {
            match command {
                Command::ReadTrack => {},
                Command::Specify => {},
                Command::SenseDriveState => {},
                Command::WriteSector => {},
                Command::ReadSector => {},
                Command::Recalibrate => {
                    let drive = self.parameters_buffer[0] as usize;
                    self.drives[drive].track = if self.drives[drive].track > 77 {
                        self.drives[drive].track - 77
                    } else {
                        0
                    }
                },
                Command::SenseInterruptState => {},
                Command::WriteDeletedSector => {},
                Command::ReadSectorId => {},
                Command::ReadDeletedSector => {},
                Command::FormatTrack => {},
                Command::Seek => {},
                Command::ScanEqual => {},
                Command::ScanLowOrEqual => {},
                Command::ScanHighOrEqual => {},
            }
        }
    }

    fn report_main_status_register(&self) -> u8 {
        let mut value = 0;

        if self.request_for_master {
            value |= 1 << 7;
        }

        if self.data_input_output {
            value |= 1 << 6;
        }

        if self.execution_mode {
            value |= 1 << 5;
        }

        if self.floppy_controller_busy {
            value |= 1 << 4;
        }

        if self.floppy_drive_busy[1] {
            value |= 1 << 1;
        }

        if self.floppy_drive_busy[0] {
            value |= 1 << 0;
        }

        value
    }
}

enum Phase {
    Command,
    Execution,
    Result,
}
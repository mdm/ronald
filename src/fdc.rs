use std::rc::Rc;
use std::{cell::RefCell, collections::VecDeque};

use crate::dsk_file::{self, Disk};

pub type FloppyDiskControllerShared = Rc<RefCell<FloppyDiskController>>;

struct Drive {
    track: usize,
    sector: usize,
    disk: Option<Disk>,
}

#[derive(Debug)]
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
    data_input_output: bool, // false: CPU -> FDC, true: FDC -> CPU
    execution_mode: bool,
    floppy_controller_busy: bool,
    floppy_drive_busy: [bool; 2], // TODO: do we need this? commands finish immediately in our implementation
    seek_end: bool,
    drive_not_ready: bool,
    selected_drive: usize,
    end_of_track: bool,
    status1: u8,
    status2: u8,
}

impl FloppyDiskController {
    pub fn new_shared() -> FloppyDiskControllerShared {
        let fdc = FloppyDiskController {
            drives: [
                Drive {
                    track: 0,
                    sector: 0,
                    disk: None,
                },
                Drive {
                    track: 0,
                    sector: 0,
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
            seek_end: false,
            drive_not_ready: false,
            selected_drive: 0,
            end_of_track: false,
            status1: 0,
            status2: 0,
        };

        Rc::new(RefCell::new(fdc))
    }

    pub fn read_byte(&mut self, port: u16) -> u8 {
        match port {
            0xfb7e => self.report_main_status_register(),
            0xfb7f => {
                dbg!(&self.result_buffer);
                match self.phase {
                    Phase::Execution => {
                        let data = if let Some(data) = self.data_buffer.pop_front() {
                            data
                        } else {
                            unreachable!()
                        };

                        if self.data_buffer.is_empty() {
                            self.execution_mode = false;
                            self.phase = Phase::Result;
                        }

                        data
                    }
                    Phase::Result => {
                        let result = if let Some(result) = self.result_buffer.pop_front() {
                            result
                        } else {
                            unreachable!()
                        };

                        if self.result_buffer.is_empty() {
                            self.data_input_output = false;
                            self.floppy_controller_busy = false;
                            self.phase = Phase::Command;
                        }

                        result
                    }
                    Phase::Command => {
                        println!("Unexpected FDC read in command phase");
                        unimplemented!()
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        match port {
            0xfa7e => match value {
                0 => {
                    self.motors_on = false;
                }
                1 => {
                    self.motors_on = true;
                }
                _ => unreachable!(),
            },
            0xfb7f => match self.phase {
                Phase::Command => match &self.command {
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
                        self.end_of_track = false;
                        self.seek_end = false;
                        self.drive_not_ready = false;
                        // TODO: do we need to reset anything else?
                        self.floppy_controller_busy = true;

                        if self.parameters_buffer.len()
                            == self.command.as_ref().unwrap().expected_parameter_bytes()
                        {
                            self.execute_command();
                        }
                    }
                },
                _ => unimplemented!(),
            },
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
                println!("tracks have {} bytes", disk.track_size);
                println!("{} sectors in track 0", disk.tracks[0].num_sectors);
                for (i, track) in disk.tracks.iter().enumerate() {
                    println!("sector {} has {} bytes", i, 0x80 << track.sector_size);
                }
                Some(disk)
            }
            Err(error) => {
                println!("LOAD ERROR: {}", error); // TODO: use logger
                None
            }
        }
    }

    fn execute_command(&mut self) {
        dbg!(&self.command);
        dbg!(&self.parameters_buffer);
        self.phase = Phase::Execution;

        if let Some(command) = self.command.take() {
            match command {
                Command::ReadTrack => {
                    unimplemented!();
                }
                Command::Specify => {
                    // assume CPC defaults
                    // TODO: handle other settings?
                    self.phase = Phase::Command;
                }
                Command::SenseDriveState => {
                    unimplemented!();
                }
                Command::WriteSector => {
                    unimplemented!();
                }
                Command::ReadSector => {
                    self.selected_drive = self.parameters_buffer[0] as usize;
                    match &self.drives[self.selected_drive].disk {
                        Some(disk) => {
                            match disk.find_track_index(
                                self.parameters_buffer[1],
                                self.parameters_buffer[2],
                            ) {
                                Some(track) => {
                                    self.drives[self.selected_drive].track = track;
                                }
                                None => unimplemented!(), // TODO: handle error
                            }

                            match disk.tracks[self.drives[self.selected_drive].track]
                                .find_sector(self.parameters_buffer[3])
                            {
                                Some(sector) => {
                                    self.drives[self.selected_drive].sector = sector;
                                }
                                None => unimplemented!(), // TODO: handle error
                            }

                            let sector_info = &disk.tracks[self.drives[self.selected_drive].track]
                                .sector_infos[self.drives[self.selected_drive].sector];
                            let sector_data = &disk.tracks[self.drives[self.selected_drive].track]
                                .sectors[self.drives[self.selected_drive].sector];

                            // TODO: verify actual sector length matches input parameters
                            self.data_buffer.extend(sector_data);
                            println!("FDC STATUS 1: {:#010b}", sector_info.fdc_status1);
                            println!("FDC STATUS 2: {:#010b}", sector_info.fdc_status2);
                            self.status1 = sector_info.fdc_status1;
                            self.status2 = sector_info.fdc_status2;
                            self.end_of_track = true;
                            self.execution_mode = true;
                            self.data_input_output = true;
                            self.write_standard_result();

                            self.phase = Phase::Execution;
                        }
                        None => {
                            self.drive_not_ready = true;
                            self.phase = Phase::Result;
                        }
                    }
                }
                Command::Recalibrate => {
                    self.selected_drive = self.parameters_buffer[0] as usize;
                    match &self.drives[self.selected_drive].disk {
                        Some(_) => {
                            self.drives[self.selected_drive].track =
                                if self.drives[self.selected_drive].track > 77 {
                                    self.drives[self.selected_drive].track - 77
                                } else {
                                    0
                                };
                            self.seek_end = true;
                        }
                        None => {
                            self.drive_not_ready = true;
                        }
                    }
                    self.phase = Phase::Command;
                }
                Command::SenseInterruptState => {
                    self.result_buffer
                        .push_back(self.report_status_register_0());
                    self.result_buffer
                        .push_back(self.drives[self.selected_drive].track as u8);
                    self.data_input_output = true;
                    self.phase = Phase::Result;
                }
                Command::WriteDeletedSector => {
                    unimplemented!();
                }
                Command::ReadSectorId => {
                    self.selected_drive = self.parameters_buffer[0] as usize;
                    match &self.drives[self.selected_drive].disk {
                        Some(_) => {
                            self.write_standard_result();
                        }
                        None => {
                            self.drive_not_ready = true;
                        }
                    }
                    self.data_input_output = true;
                    self.phase = Phase::Result;
                }
                Command::ReadDeletedSector => {
                    unimplemented!();
                }
                Command::FormatTrack => {
                    unimplemented!();
                }
                Command::Seek => {
                    self.selected_drive = self.parameters_buffer[0] as usize;
                    let track = self.parameters_buffer[1] as usize;
                    match &self.drives[self.selected_drive].disk {
                        Some(_) => {
                            self.drives[self.selected_drive].track = track;
                            self.seek_end = true;
                        }
                        None => {
                            self.drive_not_ready = true;
                        }
                    }
                    self.phase = Phase::Command;
                }
                Command::ScanEqual => {
                    unimplemented!();
                }
                Command::ScanLowOrEqual => {
                    unimplemented!();
                }
                Command::ScanHighOrEqual => {
                    unimplemented!();
                }
            }
        }

        self.parameters_buffer.clear();
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

        println!("FDC MAIN = {:#010b}", value);

        value
    }

    fn report_status_register_0(&self) -> u8 {
        let mut value = 0;

        if self.end_of_track {
            // assume no other errors occur
            value |= 1 << 6;
        }

        if self.seek_end {
            value |= 1 << 5;
        }

        if self.drive_not_ready {
            value |= 1 << 3;
        }

        value |= (self.selected_drive as u8) << 0;

        // TODO: reset status field here?
        // no - better in separate reset method
        // some flags are shared between status registers

        value
    }

    fn report_status_register_1(&self) -> u8 {
        let mut value = self.status1;

        if self.end_of_track {
            value |= 1 << 7;
        }

        value
    }

    fn report_status_register_2(&self) -> u8 {
        let mut value = self.status2;

        // TODO: handle bits 4, 3 and 2

        value
    }

    fn report_status_register_3(&self) -> u8 {
        let mut value = 0;

        if self.drives[self.selected_drive].track == 0 {
            value |= 1 << 4;
        }

        value |= (self.selected_drive as u8) << 0;

        value
    }

    fn write_standard_result(&mut self) {
        self.result_buffer
            .push_back(self.report_status_register_0());
        self.result_buffer
            .push_back(self.report_status_register_1());
        self.result_buffer
            .push_back(self.report_status_register_2());

        match &self.drives[self.selected_drive].disk {
            Some(disk) => {
                let track = self.drives[self.selected_drive].track;
                let sector = self.drives[self.selected_drive].sector;
                let sector_info = &disk.tracks[track].sector_infos[sector];
                self.result_buffer.push_back(sector_info.track);
                self.result_buffer.push_back(sector_info.side);
                self.result_buffer.push_back(sector_info.sector_id);
                self.result_buffer.push_back(sector_info.sector_size);
            }
            None => unreachable!(),
        }
    }
}

enum Phase {
    Command,
    Execution,
    Result,
}

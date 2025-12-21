use std::collections::VecDeque;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

mod dsk_file;

use dsk_file::Disk;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Drive {
    busy: bool,
    track: usize,
    sector: usize,
    disk: Option<Disk>,
}

#[derive(Debug, Serialize, Deserialize)]
enum LegacyCommand {
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

impl LegacyCommand {
    fn from_byte(byte: u8) -> LegacyCommand {
        match byte & 0x1f {
            0x02 => LegacyCommand::ReadTrack,
            0x03 => LegacyCommand::Specify,
            0x04 => LegacyCommand::SenseDriveState,
            0x05 => LegacyCommand::WriteSector,
            0x06 => LegacyCommand::ReadSector,
            0x07 => LegacyCommand::Recalibrate,
            0x08 => LegacyCommand::SenseInterruptState,
            0x09 => LegacyCommand::WriteDeletedSector,
            0x0a => LegacyCommand::ReadSectorId,
            0x0c => LegacyCommand::ReadDeletedSector,
            0x0d => LegacyCommand::FormatTrack,
            0x0f => LegacyCommand::Seek,
            0x11 => LegacyCommand::ScanEqual,
            0x19 => LegacyCommand::ScanLowOrEqual,
            0x1d => LegacyCommand::ScanHighOrEqual,
            _ => unreachable!(),
        }
    }

    fn expected_parameter_bytes(&self) -> usize {
        match self {
            LegacyCommand::ReadTrack => 8,
            LegacyCommand::Specify => 2,
            LegacyCommand::SenseDriveState => 1,
            LegacyCommand::WriteSector => 8,
            LegacyCommand::ReadSector => 8,
            LegacyCommand::Recalibrate => 1,
            LegacyCommand::SenseInterruptState => 0,
            LegacyCommand::WriteDeletedSector => 8,
            LegacyCommand::ReadSectorId => 1,
            LegacyCommand::ReadDeletedSector => 8,
            LegacyCommand::FormatTrack => 5,
            LegacyCommand::Seek => 2,
            LegacyCommand::ScanEqual => 8,
            LegacyCommand::ScanLowOrEqual => 8,
            LegacyCommand::ScanHighOrEqual => 8,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Mode {
    FrequencyModulation,
    ModifiedFrequencyModulation,
}

#[derive(Debug, Serialize, Deserialize)]
enum CommandType {
    ReadData,
    ReadDeletedData,
    WriteData,
    WriteDeletedData,
    ReadTrack,
    ReadId,
    FormatTrack,
    ScanEqual,
    ScanLowOrEqual,
    ScanHighOrEqual,
    Recalibrate,
    SenseInterruptStatus,
    Specify,
    SenseDriveStatus,
    Seek,
    Invalid,
}

impl CommandType {
    fn expected_len(&self) -> usize {
        match self {
            Self::ReadData => 9,
            Self::ReadDeletedData => 9,
            Self::WriteData => 9,
            Self::WriteDeletedData => 9,
            Self::ReadTrack => 9,
            Self::ReadId => 2,
            Self::FormatTrack => 6,
            Self::ScanEqual => 9,
            Self::ScanLowOrEqual => 9,
            Self::ScanHighOrEqual => 9,
            Self::Recalibrate => 2,
            Self::SenseInterruptStatus => 1,
            Self::Specify => 3,
            Self::SenseDriveStatus => 2,
            Self::Seek => 3,
            Self::Invalid => 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Chrn {
    cylinder_number: u8,
    head_address: u8,
    record: u8,
    number: u8,
}

impl From<u8> for CommandType {
    fn from(code: u8) -> Self {
        #[allow(clippy::identity_op)]
        match code {
            code if code & 0b0001_1111 == 0b0000_0110 => Self::ReadData,
            code if code & 0b0001_1111 == 0b0000_1100 => Self::ReadDeletedData,
            code if code & 0b0011_1111 == 0b0000_0101 => Self::WriteData,
            code if code & 0b0011_1111 == 0b0000_1001 => Self::WriteDeletedData,
            code if code & 0b1001_1111 == 0b0000_0010 => Self::ReadTrack,
            code if code & 0b1011_1111 == 0b0000_1010 => Self::ReadId,
            code if code & 0b1011_1111 == 0b0000_1101 => Self::FormatTrack,
            code if code & 0b0001_1111 == 0b0001_0001 => Self::ScanEqual,
            code if code & 0b0001_1111 == 0b0001_1001 => Self::ScanLowOrEqual,
            code if code & 0b0001_1111 == 0b0001_1101 => Self::ScanHighOrEqual,
            code if code & 0b1111_1111 == 0b0000_0111 => Self::Recalibrate,
            code if code & 0b1111_1111 == 0b0000_1000 => Self::SenseInterruptStatus,
            code if code & 0b1111_1111 == 0b0000_0011 => Self::Specify,
            code if code & 0b1111_1111 == 0b0000_0100 => Self::SenseDriveStatus,
            code if code & 0b1111_1111 == 0b0000_1111 => Self::Seek,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Command {
    ReadData {
        multi_track: bool,
        mode: Mode,
        skip: bool,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        gap_length: u8,
        data_length: u8,
    },
    ReadDeletedData {
        multi_track: bool,
        mode: Mode,
        skip: bool,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        gap_length: u8,
        data_length: u8,
    },
    WriteData {
        multi_track: bool,
        mode: Mode,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        gap_length: u8,
        data_length: u8,
    },
    WriteDeletedData {
        multi_track: bool,
        mode: Mode,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        gap_length: u8,
        data_length: u8,
    },
    ReadTrack {
        mode: Mode,
        skip: bool,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        gap_length: u8,
        data_length: u8,
    },
    ReadId {
        mode: Mode,
        head: u8,
        unit_select: u8,
    },
    FormatTrack {
        mode: Mode,
        head: u8,
        unit_select: u8,
        number: u8,
        sector: u8,
        gap_length: u8,
        data: u8,
    },
    ScanEqual {
        multi_track: bool,
        mode: Mode,
        skip: bool,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        gap_length: u8,
        scan_type: u8,
    },
    ScanLowOrEqual {
        multi_track: bool,
        mode: Mode,
        skip: bool,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        gap_length: u8,
        scan_type: u8,
    },
    ScanHighOrEqual {
        multi_track: bool,
        mode: Mode,
        skip: bool,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        gap_length: u8,
        scan_type: u8,
    },
    Recalibrate {
        unit_select: u8,
    },
    SenseInterruptStatus,
    Specify {
        step_rate_time: u8,
        head_unload_time: u8,
        head_load_time: u8,
        non_dma_mode: bool,
    },
    SenseDriveStatus {
        head: u8,
        unit_select: u8,
    },
    Seek {
        head: u8,
        unit_select: u8,
        new_cylinder_number: u8,
    },
    Invalid,
}

const BITMASK_MULTI_TRACK: u8 = 0b1000_0000;
const BITMASK_MODE: u8 = 0b0100_0000;
const BITMASK_SKIP: u8 = 0b0010_0000;
const BITMASK_HEAD: u8 = 0b0000_0100;
const BITMASK_UNIT_SELECT: u8 = 0b0000_0011;

impl From<&[u8]> for Command {
    fn from(bytes: &[u8]) -> Self {
        match bytes[0].into() {
            CommandType::ReadData => Command::ReadData {
                multi_track: (bytes[0] & BITMASK_MULTI_TRACK) != 0,
                mode: if (bytes[0] & BITMASK_MODE) != 0 {
                    Mode::ModifiedFrequencyModulation
                } else {
                    Mode::FrequencyModulation
                },
                skip: (bytes[0] & BITMASK_SKIP) != 0,
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
                chrn: Chrn {
                    cylinder_number: bytes[2],
                    head_address: bytes[3],
                    record: bytes[4],
                    number: bytes[5],
                },
                end_of_track: bytes[6],
                gap_length: bytes[7],
                data_length: bytes[8],
            },
            CommandType::ReadDeletedData => Command::ReadDeletedData {
                multi_track: (bytes[0] & BITMASK_MULTI_TRACK) != 0,
                mode: if (bytes[0] & BITMASK_MODE) != 0 {
                    Mode::ModifiedFrequencyModulation
                } else {
                    Mode::FrequencyModulation
                },
                skip: (bytes[0] & BITMASK_SKIP) != 0,
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
                chrn: Chrn {
                    cylinder_number: bytes[2],
                    head_address: bytes[3],
                    record: bytes[4],
                    number: bytes[5],
                },
                end_of_track: bytes[6],
                gap_length: bytes[7],
                data_length: bytes[8],
            },
            CommandType::WriteData => Command::WriteData {
                multi_track: (bytes[0] & BITMASK_MULTI_TRACK) != 0,
                mode: if (bytes[0] & BITMASK_MODE) != 0 {
                    Mode::ModifiedFrequencyModulation
                } else {
                    Mode::FrequencyModulation
                },
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
                chrn: Chrn {
                    cylinder_number: bytes[2],
                    head_address: bytes[3],
                    record: bytes[4],
                    number: bytes[5],
                },
                end_of_track: bytes[6],
                gap_length: bytes[7],
                data_length: bytes[8],
            },
            CommandType::WriteDeletedData => Command::WriteDeletedData {
                multi_track: (bytes[0] & BITMASK_MULTI_TRACK) != 0,
                mode: if (bytes[0] & BITMASK_MODE) != 0 {
                    Mode::ModifiedFrequencyModulation
                } else {
                    Mode::FrequencyModulation
                },
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
                chrn: Chrn {
                    cylinder_number: bytes[2],
                    head_address: bytes[3],
                    record: bytes[4],
                    number: bytes[5],
                },
                end_of_track: bytes[6],
                gap_length: bytes[7],
                data_length: bytes[8],
            },
            CommandType::ReadTrack => Command::ReadTrack {
                mode: if (bytes[0] & BITMASK_MODE) != 0 {
                    Mode::ModifiedFrequencyModulation
                } else {
                    Mode::FrequencyModulation
                },
                skip: (bytes[0] & BITMASK_SKIP) != 0,
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
                chrn: Chrn {
                    cylinder_number: bytes[2],
                    head_address: bytes[3],
                    record: bytes[4],
                    number: bytes[5],
                },
                end_of_track: bytes[6],
                gap_length: bytes[7],
                data_length: bytes[8],
            },
            CommandType::ReadId => Command::ReadId {
                mode: if (bytes[0] & BITMASK_MODE) != 0 {
                    Mode::ModifiedFrequencyModulation
                } else {
                    Mode::FrequencyModulation
                },
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
            },
            CommandType::FormatTrack => Command::FormatTrack {
                mode: if (bytes[0] & BITMASK_MODE) != 0 {
                    Mode::ModifiedFrequencyModulation
                } else {
                    Mode::FrequencyModulation
                },
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
                number: bytes[2],
                sector: bytes[3],
                gap_length: bytes[4],
                data: bytes[5],
            },
            CommandType::ScanEqual => Command::ScanEqual {
                multi_track: (bytes[0] & BITMASK_MULTI_TRACK) != 0,
                mode: if (bytes[0] & BITMASK_MODE) != 0 {
                    Mode::ModifiedFrequencyModulation
                } else {
                    Mode::FrequencyModulation
                },
                skip: (bytes[0] & BITMASK_SKIP) != 0,
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
                chrn: Chrn {
                    cylinder_number: bytes[2],
                    head_address: bytes[3],
                    record: bytes[4],
                    number: bytes[5],
                },
                end_of_track: bytes[6],
                gap_length: bytes[7],
                scan_type: bytes[8],
            },
            CommandType::ScanLowOrEqual => Command::ScanLowOrEqual {
                multi_track: (bytes[0] & BITMASK_MULTI_TRACK) != 0,
                mode: if (bytes[0] & BITMASK_MODE) != 0 {
                    Mode::ModifiedFrequencyModulation
                } else {
                    Mode::FrequencyModulation
                },
                skip: (bytes[0] & BITMASK_SKIP) != 0,
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
                chrn: Chrn {
                    cylinder_number: bytes[2],
                    head_address: bytes[3],
                    record: bytes[4],
                    number: bytes[5],
                },
                end_of_track: bytes[6],
                gap_length: bytes[7],
                scan_type: bytes[8],
            },
            CommandType::ScanHighOrEqual => Command::ScanHighOrEqual {
                multi_track: (bytes[0] & BITMASK_MULTI_TRACK) != 0,
                mode: if (bytes[0] & BITMASK_MODE) != 0 {
                    Mode::ModifiedFrequencyModulation
                } else {
                    Mode::FrequencyModulation
                },
                skip: (bytes[0] & BITMASK_SKIP) != 0,
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
                chrn: Chrn {
                    cylinder_number: bytes[2],
                    head_address: bytes[3],
                    record: bytes[4],
                    number: bytes[5],
                },
                end_of_track: bytes[6],
                gap_length: bytes[7],
                scan_type: bytes[8],
            },
            CommandType::Recalibrate => Command::Recalibrate {
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
            },
            CommandType::SenseInterruptStatus => Command::SenseInterruptStatus,
            CommandType::Specify => Command::Specify {
                step_rate_time: (bytes[1] & 0b1111_0000) >> 4,
                head_unload_time: bytes[1] & 0b0000_1111,
                head_load_time: (bytes[2] & 0b1111_1110) >> 1,
                non_dma_mode: (bytes[2] & 0b0000_0001) != 0,
            },
            CommandType::SenseDriveStatus => Command::SenseDriveStatus {
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
            },
            CommandType::Seek => Command::Seek {
                head: (bytes[1] & BITMASK_HEAD) >> 2,
                unit_select: bytes[1] & BITMASK_UNIT_SELECT,
                new_cylinder_number: bytes[2],
            },
            CommandType::Invalid => Command::Invalid,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum InterruptCode {
    #[default]
    NormalTermination,
    AbnormalTermination,
    InvalidCommand,
    ReadyChanged,
}

#[derive(Debug, Default)]
struct StatusRegister0 {
    interrupt_code: InterruptCode,
    seek_end: bool,
    equipment_check: bool,
    not_ready: bool,
    head_address: u8,
    unit_select: u8,
}

impl From<StatusRegister0> for u8 {
    fn from(value: StatusRegister0) -> Self {
        let mut byte = 0;

        match value.interrupt_code {
            InterruptCode::NormalTermination => byte |= 0b00 << 6,
            InterruptCode::AbnormalTermination => byte |= 0b01 << 6,
            InterruptCode::InvalidCommand => byte |= 0b10 << 6,
            InterruptCode::ReadyChanged => byte |= 0b11 << 6,
        }

        if value.seek_end {
            byte |= 1 << 5;
        }

        if value.equipment_check {
            byte |= 1 << 4;
        }

        if value.not_ready {
            byte |= 1 << 3;
        }

        byte |= (value.head_address & 1) << 2;

        byte |= value.unit_select & 0b11;

        byte
    }
}

#[derive(Debug, Default)]
struct StatusRegister1 {
    end_of_cylinder: bool,
    data_error: bool,
    over_run: bool,
    no_data: bool,
    not_writeable: bool,
    missing_address_mark: bool,
}

impl From<StatusRegister1> for u8 {
    fn from(value: StatusRegister1) -> Self {
        let mut byte = 0;

        if value.end_of_cylinder {
            byte |= 1 << 7;
        }

        if value.data_error {
            byte |= 1 << 5;
        }

        if value.over_run {
            byte |= 1 << 4;
        }

        if value.no_data {
            byte |= 1 << 2;
        }

        if value.not_writeable {
            byte |= 1 << 1;
        }

        if value.missing_address_mark {
            byte |= 1;
        }

        byte
    }
}

#[derive(Debug, Default)]
struct StatusRegister2 {
    control_mark: bool,
    data_error_in_data_field: bool,
    wrong_cylinder: bool,
    scan_equal_hit: bool,
    scan_not_satisfied: bool,
    bad_cylinder: bool,
    missing_address_mark_in_data_field: bool,
}

impl From<StatusRegister2> for u8 {
    fn from(value: StatusRegister2) -> Self {
        let mut byte = 0;

        if value.control_mark {
            byte |= 1 << 6;
        }

        if value.data_error_in_data_field {
            byte |= 1 << 5;
        }

        if value.wrong_cylinder {
            byte |= 1 << 4;
        }

        if value.scan_equal_hit {
            byte |= 1 << 3;
        }

        if value.scan_not_satisfied {
            byte |= 1 << 2;
        }

        if value.bad_cylinder {
            byte |= 1 << 1;
        }

        if value.missing_address_mark_in_data_field {
            byte |= 1;
        }

        byte
    }
}

struct StatusRegister3 {
    fault: bool,
    write_protected: bool,
    ready: bool,
    track_zero: bool,
    two_side: bool,
    head_address: u8,
    unit_select: u8,
}

impl From<StatusRegister3> for u8 {
    fn from(value: StatusRegister3) -> Self {
        let mut byte = 0;

        if value.fault {
            byte |= 1 << 7;
        }

        if value.write_protected {
            byte |= 1 << 6;
        }

        if value.ready {
            byte |= 1 << 5;
        }

        if value.track_zero {
            byte |= 1 << 4;
        }

        if value.two_side {
            byte |= 1 << 3;
        }

        byte |= (value.head_address & 1) << 2;

        byte |= value.unit_select & 0b11;

        byte
    }
}

struct StandardResult {
    st0: StatusRegister0,
    st1: StatusRegister1,
    st2: StatusRegister2,
    chrn: Chrn,
}

enum CommandResult {
    ReadData(StandardResult),
    ReadDeletedData(StandardResult),
    WriteData(StandardResult),
    WriteDeletedData(StandardResult),
    ReadTrack(StandardResult),
    ReadId(StandardResult),
    FormatTrack(StandardResult),
    ScanEqual(StandardResult),
    ScanLowOrEqual(StandardResult),
    ScanHighOrEqual(StandardResult),
    Recalibrate,
    SenseInterruptStatus { st0: StatusRegister0, pcn: u8 },
    Specify,
    SenseDriveStatus { st3: StatusRegister3 },
    Seek,
    Invalid { st0: StatusRegister0 },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloppyDiskController {
    drives: Vec<Drive>,
    phase: Phase,
    command_buffer: Vec<u8>,
    data_buffer: VecDeque<u8>,
    result_buffer: VecDeque<u8>,

    step_rate_time: u8,
    head_unload_time: u8,
    head_load_time: u8,
    non_dma_mode: bool,

    motors_on: bool,
    seek_end: bool,
    drive_not_ready: bool,
    selected_drive: usize,
    end_of_track: bool,
    status1: u8,
    status2: u8,
}

impl Default for FloppyDiskController {
    fn default() -> Self {
        Self::new(1)
    }
}

impl FloppyDiskController {
    pub fn new(num_drives: usize) -> Self {
        let mut drives = vec![
            Drive {
                busy: false,
                track: 0,
                sector: 0,
                disk: None,
            },
            Drive {
                busy: false,
                track: 0,
                sector: 0,
                disk: None,
            },
        ];
        drives.truncate(num_drives);

        Self {
            drives,
            phase: Phase::Command,
            command_buffer: Vec::new(),
            data_buffer: VecDeque::new(),
            result_buffer: VecDeque::new(),

            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: false,

            motors_on: false,
            seek_end: false,
            drive_not_ready: false,
            selected_drive: 0,
            end_of_track: false,
            status1: 0,
            status2: 0,
        }
    }

    pub fn read_byte(&mut self, port: u16) -> u8 {
        match port {
            0xfb7e => self.report_main_status_register(),
            0xfb7f => {
                match self.phase {
                    Phase::Execution => {
                        // TODO: handle over run here (modify result if last poll more than 26us ago)

                        let data = if let Some(data) = self.data_buffer.pop_front() {
                            log::trace!("Reading data from FDC: {data:#04X}");
                            data
                        } else {
                            log::error!("Unexpected FDC read in execution phase");
                            todo!("return dummy value instead?");
                        };

                        if self.data_buffer.is_empty() {
                            self.phase = Phase::Result;
                        }

                        data
                    }
                    Phase::Result => {
                        let result = if let Some(result) = self.result_buffer.pop_front() {
                            log::debug!("Reading result from FDC: {result:#04X}");
                            result
                        } else {
                            // TODO: we hit this if no disk is loaded and CAT is executed
                            log::error!("Unexpected FDC read in result phase");
                            todo!("return dummy value instead?");
                        };

                        if self.result_buffer.is_empty() {
                            self.phase = Phase::Command;
                        }

                        result
                    }
                    Phase::Command => {
                        log::error!("Unexpected FDC read in command phase");
                        todo!("return dummy value instead?");
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
                Phase::Command => {
                    if self.command_buffer.is_empty()
                        || self.command_buffer.len()
                            < CommandType::from(self.command_buffer[0]).expected_len()
                    {
                        self.command_buffer.push(value);
                    }

                    if self.command_buffer.len()
                        == CommandType::from(self.command_buffer[0]).expected_len()
                    {
                        self.execute_command();
                    }
                }
                _ => {
                    log::error!(
                        "FDC write outside command phase using port {port:#06X}: {value:#010b}"
                    );
                }
            },
            _ => {
                log::error!("Unexpected FDC write using port {port:#06X}: {value:#010b}");
            }
        }
    }

    pub fn load_disk(&mut self, drive: usize, rom: Vec<u8>, path: PathBuf) {
        self.drives[drive].disk = match dsk_file::Disk::load(rom, path) {
            Ok(disk) => {
                log::info!("Disk loaded successfully");
                Some(disk)
            }
            Err(error) => {
                log::warn!("Disk could not be loaded: {error}");
                None
            }
        }
    }

    fn execute_command(&mut self) -> CommandResult {
        self.phase = Phase::Execution;

        let command = self.command_buffer.as_slice().into();
        self.command_buffer.clear();
        log::debug!("Executing FDC command: {:?}", &command);
        match command {
            Command::ReadData {
                multi_track,
                mode,
                skip,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                data_length,
            } => {
                if multi_track {
                    log::error!("Unsupported multi-track flag set");
                }
                if let Mode::FrequencyModulation = mode {
                    log::error!("Unsupported frequency modulation mode");
                }
                if skip {
                    log::error!("Unsupported skip flag set");
                }
                if head != 0 {
                    log::error!("Unsupported head number");
                    todo!("Return NOT READY in ST0")
                }
                let head_address = 0;

                self.data_buffer.clear();
                self.result_buffer.clear();

                match self.drives.get(unit_select as usize) {
                    Some(drive) => {
                        let Some(disk) = drive.disk else {
                            self.phase = Phase::Result;

                            // ST0
                            let interrupt_code = InterruptCode::AbnormalTermination;

                            // ST1
                            let no_data = true;

                            // ST2
                            let missing_address_mark_in_data_field = true;

                            return CommandResult::ReadData(StandardResult {
                                st0: StatusRegister0 {
                                    interrupt_code,
                                    ..Default::default()
                                },
                                st1: StatusRegister1 {
                                    no_data,
                                    ..Default::default()
                                },
                                st2: StatusRegister2 {
                                    missing_address_mark_in_data_field,
                                    ..Default::default()
                                },
                                chrn,
                            });
                        };

                        let track = drive.track;

                        let data_length = if chrn.number == 0 {
                            data_length as usize
                        } else {
                            128 << chrn.number
                        };

                        // ST0
                        let interrupt_code = InterruptCode::NormalTermination;

                        // ST1
                        let end_of_cylinder = false;
                        let data_error = false;
                        let no_data = false;
                        let missing_address_mark = false;

                        // ST2
                        let control_mark = false;
                        let data_error_in_data_field = false;
                        let wrong_cylinder = false;
                        let bad_cylinder = false;
                        let missing_address_mark_in_data_field = false;

                        loop {
                            let sector = match disk.tracks[track].find_sector(chrn) {
                                Some(sector) => sector,
                                None => {
                                    no_data = true;
                                    interrupt_code = InterruptCode::AbnormalTermination;
                                    break;
                                }
                            };

                            let sector_info = disk.tracks[track].sector_infos[sector];

                            if sector_info.fdc_status1 & 0b0010_0000 != 0 {
                                data_error = true;
                                interrupt_code = InterruptCode::AbnormalTermination;
                            }

                            // Contrary to https://www.cpcwiki.eu/index.php/Format:DSK_disk_image_file_format
                            // we determine the NO DATA condition above instead of from fdc_status1 bit 2

                            if sector_info.fdc_status1 & 0b0000_0001 != 0 {
                                missing_address_mark = true;
                                interrupt_code = InterruptCode::AbnormalTermination;
                            }

                            if sector_info.fdc_status2 & 0b0100_0000 != 0 {
                                control_mark = true;
                                interrupt_code = InterruptCode::AbnormalTermination;
                            }

                            if sector_info.fdc_status2 & 0b0010_0000 != 0 {
                                data_error_in_data_field = true;
                                interrupt_code = InterruptCode::AbnormalTermination;
                            }

                            if sector_info.chrn.cylinder_number != chrn.cylinder_number {
                                no_data = true;
                                wrong_cylinder = true;
                                if sector_info.chrn.cylinder_number == 0xff {
                                    bad_cylinder = true;
                                }
                                interrupt_code = InterruptCode::AbnormalTermination;
                            }

                            if sector_info.fdc_status2 & 0b0000_0001 != 0 {
                                missing_address_mark_in_data_field = true;
                                interrupt_code = InterruptCode::AbnormalTermination;
                            }

                            if !control_mark && interrupt_code == InterruptCode::AbnormalTermination
                            {
                                break;
                            }

                            let sector_data = disk.tracks[track].sectors[sector];

                            if data_length > sector_data.len() {
                                log::error!("Specified data length exceeds physical sector size");
                            }

                            self.data_buffer
                                .extend(sector_data.iter().take(data_length));

                            if !control_mark && chrn.cylinder_number < end_of_track {
                                chrn.record += 1;
                            } else {
                                end_of_cylinder = true;
                                chrn.cylinder_number += 1;
                                chrn.record = 1;
                                self.phase = Phase::Execution;
                                break;
                            }
                        }

                        CommandResult::ReadData(StandardResult {
                            st0: StatusRegister0 {
                                interrupt_code,
                                head_address,
                                unit_select,
                                ..Default::default()
                            },
                            st1: StatusRegister1 {
                                end_of_cylinder,
                                data_error,
                                no_data,
                                missing_address_mark,
                                ..Default::default()
                            },
                            st2: StatusRegister2 {
                                control_mark,
                                data_error_in_data_field,
                                wrong_cylinder,
                                bad_cylinder,
                                missing_address_mark_in_data_field,
                                ..Default::default()
                            },
                            chrn,
                        })
                    }
                    None => {
                        self.phase = Phase::Result;

                        // ST0
                        let interrupt_code = InterruptCode::AbnormalTermination;
                        let equipment_check = true;

                        // ST1
                        let no_data = true;
                        let end_of_cylinder = true;

                        // ST2
                        let missing_address_mark_in_data_field = true;

                        CommandResult::ReadData(StandardResult {
                            st0: StatusRegister0 {
                                interrupt_code,
                                equipment_check,
                                ..Default::default()
                            },
                            st1: StatusRegister1 {
                                no_data,
                                end_of_cylinder,
                                ..Default::default()
                            },
                            st2: StatusRegister2 {
                                missing_address_mark_in_data_field,
                                ..Default::default()
                            },
                            chrn,
                        })
                    }
                }
            }
            Command::ReadDeletedData { .. } => {}
            Command::WriteData { .. } => {}
            Command::WriteDeletedData { .. } => {}
            Command::ReadTrack { .. } => {}
            Command::ReadId { .. } => {}
            Command::FormatTrack { .. } => {}
            Command::ScanEqual { .. } => {}
            Command::ScanLowOrEqual { .. } => {}
            Command::ScanHighOrEqual { .. } => {}
            Command::Recalibrate { .. } => {}
            Command::SenseInterruptStatus { .. } => {
                //TODO: return InterruptCode::InvalidCommand if SEEK or RECALIBRATE still in
                //progress
            }
            Command::Specify {
                step_rate_time,
                head_unload_time,
                head_load_time,
                non_dma_mode,
            } => {
                if !non_dma_mode {
                    unimplemented!();
                }

                self.step_rate_time = step_rate_time;
                self.head_unload_time = head_unload_time;
                self.head_load_time = head_load_time;
                self.non_dma_mode = non_dma_mode;

                self.phase = Phase::Command;
            }
            Command::SenseDriveStatus { .. } => {}
            Command::Seek { .. } => {}
            Command::Invalid { .. } => {}

            LegacyCommand::ReadTrack => {
                unimplemented!();
            }
            LegacyCommand::Specify => {}
            LegacyCommand::SenseDriveState => {
                unimplemented!();
            }
            LegacyCommand::WriteSector => {
                unimplemented!();
            }
            LegacyCommand::ReadSector => {}
            LegacyCommand::Recalibrate => {
                self.selected_drive = self.command_buffer[0] as usize;
                match &self.drives[self.selected_drive].disk {
                    Some(_) => {
                        self.drives[self.selected_drive].track =
                            self.drives[self.selected_drive].track.saturating_sub(77);
                        self.seek_end = true;
                    }
                    None => {
                        self.drive_not_ready = true;
                    }
                }
                self.phase = Phase::Command;
            }
            LegacyCommand::SenseInterruptState => {
                self.result_buffer
                    .push_back(self.report_status_register_0());
                self.result_buffer
                    .push_back(self.drives[self.selected_drive].track as u8);
                self.phase = Phase::Result;
            }
            LegacyCommand::WriteDeletedSector => {
                unimplemented!();
            }
            LegacyCommand::ReadSectorId => {
                self.selected_drive = self.command_buffer[0] as usize;
                match &self.drives[self.selected_drive].disk {
                    Some(_) => {
                        self.write_standard_result();
                    }
                    None => {
                        self.drive_not_ready = true;
                    }
                }
                self.phase = Phase::Result;
            }
            LegacyCommand::ReadDeletedSector => {
                unimplemented!();
            }
            LegacyCommand::FormatTrack => {
                unimplemented!();
            }
            LegacyCommand::Seek => {
                self.selected_drive = self.command_buffer[0] as usize;
                let track = self.command_buffer[1] as usize;
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
            LegacyCommand::ScanEqual => {
                unimplemented!();
            }
            LegacyCommand::ScanLowOrEqual => {
                unimplemented!();
            }
            LegacyCommand::ScanHighOrEqual => {
                unimplemented!();
            }
        }
    }

    fn report_main_status_register(&self) -> u8 {
        let mut value = 0;

        // Request for master
        // TODO: Can we ever observe RQM cleared? We execute commands instantly.
        value |= 1 << 7;

        // Data input/output
        if matches!(self.phase, Phase::Execution | Phase::Result) {
            value |= 1 << 6;
        }

        // Execution mode
        if self.non_dma_mode
            && let Phase::Execution = self.phase
        {
            value |= 1 << 5;
        }

        // FDC busy
        // TODO: Do we ever need to set this? We execute commands instantly.
        // value |= 1 << 4;

        // Drive busy flags
        // TODO: Can we ever observe busy drives? We execute commands instantly.
        self.drives
            .iter()
            .enumerate()
            .fold(
                value,
                |value, (i, drive)| {
                    if drive.busy { value | (1 << i) } else { value }
                },
            );

        log::trace!("Reporting FDC main status register: {value:#010b}");

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

        value |= self.selected_drive as u8;

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
        // let value = self.status2;

        // // TODO: handle bits 4, 3 and 2

        // value
        self.status2
    }

    fn report_status_register_3(&self) -> u8 {
        let mut value = 0;

        if self.drives[self.selected_drive].track == 0 {
            value |= 1 << 4;
        }

        value |= self.selected_drive as u8;

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

#[derive(Serialize, Deserialize)]
enum Phase {
    Command,
    Execution,
    Result,
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;
}

use std::path::PathBuf;
use std::{collections::VecDeque, fmt::Display};

use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

mod dsk_file;

use dsk_file::Disk;

use crate::debug::event::FdcDebugEvent;
use crate::debug::view::FdcDebugView;
use crate::debug::{DebugSource, Debuggable, Snapshottable};
use crate::system::clock::MasterClockTick;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Drive {
    busy: bool,
    track: usize,
    disk: Option<Disk>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Command,
    Execution,
    Result,
}

impl Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Command => write!(f, "Command (idle)"),
            Self::Execution => write!(f, "Execution"),
            Self::Result => write!(f, "Result"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Mode {
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
    fn command_len(&self) -> usize {
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

impl From<CommandType> for u8 {
    fn from(value: CommandType) -> Self {
        match value {
            CommandType::ReadData => 0b0000_0110,
            CommandType::ReadDeletedData => 0b0000_1100,
            CommandType::WriteData => 0b0000_0101,
            CommandType::WriteDeletedData => 0b0000_1001,
            CommandType::ReadTrack => 0b0000_0010,
            CommandType::ReadId => 0b0000_1010,
            CommandType::FormatTrack => 0b0000_1101,
            CommandType::ScanEqual => 0b0001_0001,
            CommandType::ScanLowOrEqual => 0b0001_1001,
            CommandType::ScanHighOrEqual => 0b0001_1101,
            CommandType::Recalibrate => 0b0000_0111,
            CommandType::SenseInterruptStatus => 0b0000_1000,
            CommandType::Specify => 0b0000_0011,
            CommandType::SenseDriveStatus => 0b0000_0100,
            CommandType::Seek => 0b0000_1111,
            CommandType::Invalid => 0b0000_0000,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Chrn {
    pub cylinder_number: u8,
    pub head_address: u8,
    pub record: u8,
    pub number: u8,
}

impl Display for Chrn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}",
            self.cylinder_number, self.head_address, self.record, self.number
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Command {
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

impl Command {
    fn write_len(&self) -> usize {
        match self {
            Self::WriteData {
                chrn, data_length, ..
            }
            | Self::WriteDeletedData {
                chrn, data_length, ..
            } => {
                if chrn.number == 0 {
                    *data_length as usize
                } else {
                    128 << chrn.number
                }
            }
            Self::FormatTrack { sector, .. } => *sector as usize * 4,
            Self::ScanEqual { chrn, .. }
            | Self::ScanLowOrEqual { chrn, .. }
            | Self::ScanHighOrEqual { chrn, .. } => 128 << chrn.number,
            _ => 0,
        }
    }
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

impl From<&Command> for Vec<u8> {
    fn from(value: &Command) -> Self {
        let mut bytes = Vec::new();

        match *value {
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
                let mut header = CommandType::ReadData.into();

                if multi_track {
                    header |= BITMASK_MULTI_TRACK;
                }

                if let Mode::ModifiedFrequencyModulation = mode {
                    header |= BITMASK_MODE;
                }

                if skip {
                    header |= BITMASK_SKIP;
                }

                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);

                bytes.push(chrn.cylinder_number);
                bytes.push(chrn.head_address);
                bytes.push(chrn.record);
                bytes.push(chrn.number);

                bytes.push(end_of_track);
                bytes.push(gap_length);
                bytes.push(data_length);
            }
            Command::ReadDeletedData {
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
                let mut header = CommandType::ReadDeletedData.into();

                if multi_track {
                    header |= BITMASK_MULTI_TRACK;
                }

                if let Mode::ModifiedFrequencyModulation = mode {
                    header |= BITMASK_MODE;
                }

                if skip {
                    header |= BITMASK_SKIP;
                }

                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);

                bytes.push(chrn.cylinder_number);
                bytes.push(chrn.head_address);
                bytes.push(chrn.record);
                bytes.push(chrn.number);

                bytes.push(end_of_track);
                bytes.push(gap_length);
                bytes.push(data_length);
            }
            Command::WriteData {
                multi_track,
                mode,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                data_length,
            } => {
                let mut header = CommandType::WriteData.into();

                if multi_track {
                    header |= BITMASK_MULTI_TRACK;
                }

                if let Mode::ModifiedFrequencyModulation = mode {
                    header |= BITMASK_MODE;
                }

                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);

                bytes.push(chrn.cylinder_number);
                bytes.push(chrn.head_address);
                bytes.push(chrn.record);
                bytes.push(chrn.number);

                bytes.push(end_of_track);
                bytes.push(gap_length);
                bytes.push(data_length);
            }
            Command::WriteDeletedData {
                multi_track,
                mode,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                data_length,
            } => {
                let mut header = CommandType::WriteDeletedData.into();

                if multi_track {
                    header |= BITMASK_MULTI_TRACK;
                }

                if let Mode::ModifiedFrequencyModulation = mode {
                    header |= BITMASK_MODE;
                }

                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);

                bytes.push(chrn.cylinder_number);
                bytes.push(chrn.head_address);
                bytes.push(chrn.record);
                bytes.push(chrn.number);

                bytes.push(end_of_track);
                bytes.push(gap_length);
                bytes.push(data_length);
            }
            Command::ReadTrack {
                mode,
                skip,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                data_length,
            } => {
                let mut header = CommandType::ReadTrack.into();

                if let Mode::ModifiedFrequencyModulation = mode {
                    header |= BITMASK_MODE;
                }

                if skip {
                    header |= BITMASK_SKIP;
                }

                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);

                bytes.push(chrn.cylinder_number);
                bytes.push(chrn.head_address);
                bytes.push(chrn.record);
                bytes.push(chrn.number);

                bytes.push(end_of_track);
                bytes.push(gap_length);
                bytes.push(data_length);
            }
            Command::ReadId {
                mode,
                head,
                unit_select,
            } => {
                let mut header = CommandType::ReadId.into();

                if let Mode::ModifiedFrequencyModulation = mode {
                    header |= BITMASK_MODE;
                }

                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);
            }
            Command::FormatTrack {
                mode,
                head,
                unit_select,
                number,
                sector,
                gap_length,
                data,
            } => {
                let mut header = CommandType::FormatTrack.into();

                if let Mode::ModifiedFrequencyModulation = mode {
                    header |= BITMASK_MODE;
                }

                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);

                bytes.push(number);
                bytes.push(sector);

                bytes.push(gap_length);
                bytes.push(data);
            }
            Command::ScanEqual {
                multi_track,
                mode,
                skip,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                scan_type,
            } => {
                let mut header = CommandType::ScanEqual.into();

                if multi_track {
                    header |= BITMASK_MULTI_TRACK;
                }

                if let Mode::ModifiedFrequencyModulation = mode {
                    header |= BITMASK_MODE;
                }

                if skip {
                    header |= BITMASK_SKIP;
                }

                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);

                bytes.push(chrn.cylinder_number);
                bytes.push(chrn.head_address);
                bytes.push(chrn.record);
                bytes.push(chrn.number);

                bytes.push(end_of_track);
                bytes.push(gap_length);
                bytes.push(scan_type);
            }
            Command::ScanLowOrEqual {
                multi_track,
                mode,
                skip,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                scan_type,
            } => {
                let mut header = CommandType::ScanLowOrEqual.into();

                if multi_track {
                    header |= BITMASK_MULTI_TRACK;
                }

                if let Mode::ModifiedFrequencyModulation = mode {
                    header |= BITMASK_MODE;
                }

                if skip {
                    header |= BITMASK_SKIP;
                }

                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);

                bytes.push(chrn.cylinder_number);
                bytes.push(chrn.head_address);
                bytes.push(chrn.record);
                bytes.push(chrn.number);

                bytes.push(end_of_track);
                bytes.push(gap_length);
                bytes.push(scan_type);
            }
            Command::ScanHighOrEqual {
                multi_track,
                mode,
                skip,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                scan_type,
            } => {
                let mut header = CommandType::ScanHighOrEqual.into();

                if multi_track {
                    header |= BITMASK_MULTI_TRACK;
                }

                if let Mode::ModifiedFrequencyModulation = mode {
                    header |= BITMASK_MODE;
                }

                if skip {
                    header |= BITMASK_SKIP;
                }

                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);

                bytes.push(chrn.cylinder_number);
                bytes.push(chrn.head_address);
                bytes.push(chrn.record);
                bytes.push(chrn.number);

                bytes.push(end_of_track);
                bytes.push(gap_length);
                bytes.push(scan_type);
            }
            Command::Recalibrate { unit_select } => {
                let header = CommandType::Recalibrate.into();
                bytes.push(header);

                let subheader = unit_select & BITMASK_UNIT_SELECT;
                bytes.push(subheader);
            }
            Command::SenseInterruptStatus => {
                let header = CommandType::SenseInterruptStatus.into();
                bytes.push(header);
            }
            Command::Specify {
                step_rate_time,
                head_unload_time,
                head_load_time,
                non_dma_mode,
            } => {
                let header = CommandType::Specify.into();
                bytes.push(header);

                let params1 = (step_rate_time << 4) | (head_unload_time & 0b0000_1111);
                bytes.push(params1);

                let mut params2 = head_load_time << 1;

                if non_dma_mode {
                    params2 |= 0b0000_0001;
                }

                bytes.push(params2);
            }
            Command::SenseDriveStatus { head, unit_select } => {
                let header = CommandType::SenseDriveStatus.into();
                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);
            }
            Command::Seek {
                head,
                unit_select,
                new_cylinder_number,
            } => {
                let header = CommandType::Seek.into();
                bytes.push(header);

                let mut subheader = unit_select & BITMASK_UNIT_SELECT;

                if head != 0 {
                    subheader |= BITMASK_HEAD;
                }

                bytes.push(subheader);

                bytes.push(new_cylinder_number);
            }
            Command::Invalid => {
                let header = CommandType::Invalid.into();
                bytes.push(header);
            }
        }

        bytes
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterruptCode {
    #[default]
    NormalTermination,
    AbnormalTermination,
    InvalidCommand,
    ReadyChanged,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusRegister0 {
    pub interrupt_code: InterruptCode,
    pub seek_end: bool,
    pub equipment_check: bool,
    pub not_ready: bool,
    pub head_address: u8,
    pub unit_select: u8,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusRegister1 {
    pub end_of_cylinder: bool,
    pub data_error: bool,
    pub over_run: bool,
    pub no_data: bool,
    pub not_writeable: bool,
    pub missing_address_mark: bool,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusRegister2 {
    pub control_mark: bool,
    pub data_error_in_data_field: bool,
    pub wrong_cylinder: bool,
    pub scan_equal_hit: bool,
    pub scan_not_satisfied: bool,
    pub bad_cylinder: bool,
    pub missing_address_mark_in_data_field: bool,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusRegister3 {
    pub fault: bool,
    pub write_protected: bool,
    pub ready: bool,
    pub track_zero: bool,
    pub two_side: bool,
    pub head_address: u8,
    pub unit_select: u8,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardResult {
    pub st0: StatusRegister0,
    pub st1: StatusRegister1,
    pub st2: StatusRegister2,
    pub chrn: Chrn,
}

impl StandardResult {
    fn not_ready(chrn: Chrn) -> Self {
        let interrupt_code = InterruptCode::AbnormalTermination;
        let not_ready = true;

        Self {
            st0: StatusRegister0 {
                interrupt_code,
                not_ready,
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn,
        }
    }
}

impl IntoIterator for StandardResult {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.st0.into(),
            self.st1.into(),
            self.st2.into(),
            self.chrn.cylinder_number,
            self.chrn.head_address,
            self.chrn.record,
            self.chrn.number,
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandResult {
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

impl IntoIterator for CommandResult {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::ReadData(result)
            | Self::ReadDeletedData(result)
            | Self::WriteData(result)
            | Self::WriteDeletedData(result)
            | Self::ReadTrack(result)
            | Self::ReadId(result)
            | Self::FormatTrack(result)
            | Self::ScanEqual(result)
            | Self::ScanLowOrEqual(result)
            | Self::ScanHighOrEqual(result) => result.into_iter(),
            Self::Recalibrate | Self::Seek | Self::Specify => vec![].into_iter(),
            Self::SenseInterruptStatus { st0, pcn } => vec![st0.into(), pcn].into_iter(),
            Self::SenseDriveStatus { st3 } => vec![st3.into()].into_iter(),
            Self::Invalid { st0 } => vec![st0.into()].into_iter(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum Register {
    MainStatus = 0xfb7e,
    Data = 0xfb7f,
    MotorControl = 0xfa7e,
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MainStatus => write!(f, "Main Status"),
            Self::Data => write!(f, "Data"),
            Self::MotorControl => write!(f, "Motor Control"),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FloppyDiskController {
    master_clock: MasterClockTick,
    drives: Vec<Drive>,
    motors_on: bool,
    busy: bool,
    phase: Phase,
    command_buffer: Vec<u8>,
    data_buffer: VecDeque<u8>,
    result_buffer: VecDeque<u8>,
    current_command: Option<Command>,
    current_result: Option<CommandResult>,
    interrupt_status: Option<StatusRegister0>,

    step_rate_time: u8,
    head_unload_time: u8,
    head_load_time: u8,
    non_dma_mode: bool,
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
                disk: None,
            },
            Drive {
                busy: false,
                track: 0,
                disk: None,
            },
        ];
        drives.truncate(num_drives);

        Self {
            master_clock: MasterClockTick::default(),
            drives,
            motors_on: false,
            busy: false,
            phase: Phase::Command,
            command_buffer: Vec::new(),
            data_buffer: VecDeque::new(),
            result_buffer: VecDeque::new(),
            current_command: None,
            current_result: None,
            interrupt_status: None,

            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: false,
        }
    }

    pub fn read_byte(&mut self, port: u16) -> u8 {
        match port.try_into() {
            Ok(Register::MainStatus) => {
                let value = self.report_main_status_register();
                self.emit_debug_event(
                    FdcDebugEvent::RegisterRead {
                        register: Register::MainStatus,
                        value,
                    },
                    self.master_clock,
                );
                value
            }
            Ok(Register::Data) => {
                let value = match self.phase {
                    Phase::Execution => {
                        // Design decision: Never report Over Run, since we don't have the constraints of the original hardware.

                        let data = if let Some(data) = self.data_buffer.pop_front() {
                            log::trace!("Reading data from FDC: {data:#04X}");
                            data
                        } else {
                            log::error!("Unexpected FDC read in execution phase");
                            todo!("return dummy value instead?");
                        };

                        if self.data_buffer.is_empty() {
                            self.enter_phase(Phase::Result);
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
                            self.interrupt_status = None;
                            self.busy = false;
                            self.enter_phase(Phase::Command);
                        }

                        result
                    }
                    Phase::Command => {
                        log::error!("Unexpected FDC read in command phase");
                        todo!("return dummy value instead?");
                    }
                };
                self.emit_debug_event(
                    FdcDebugEvent::RegisterRead {
                        register: Register::Data,
                        value,
                    },
                    self.master_clock,
                );
                value
            }
            _ => {
                log::error!("Unexpected FDC read using port {port:#06X}");
                todo!("return dummy value instead?");
            }
        }
    }

    pub fn write_byte(&mut self, port: u16, value: u8) {
        match port.try_into() {
            Ok(Register::MotorControl) => {
                match value {
                    0 => {
                        self.motors_on = false;
                    }
                    1 => {
                        self.motors_on = true;
                    }
                    _ => unreachable!(),
                }
                self.emit_debug_event(
                    FdcDebugEvent::RegisterWritten {
                        register: Register::MotorControl,
                        value,
                    },
                    self.master_clock,
                );
            }
            Ok(Register::Data) => {
                match &self.phase {
                    Phase::Command => {
                        self.busy = true;

                        if self.command_buffer.is_empty()
                            || self.command_buffer.len()
                                < CommandType::from(self.command_buffer[0]).command_len()
                        {
                            self.command_buffer.push(value);
                        }

                        if self.command_buffer.len()
                            == CommandType::from(self.command_buffer[0]).command_len()
                        {
                            let command = self.command_buffer.as_slice().into();
                            self.command_buffer.clear();
                            self.data_buffer.clear();
                            self.result_buffer.clear();
                            self.current_command = Some(command);
                            self.enter_phase(Phase::Execution);
                        }
                    }
                    Phase::Execution => {
                        // Design decision: Never report Over Run, since we don't have the constraints of the original hardware.

                        if let Some(command) = &self.current_command
                            && self.data_buffer.len() < command.write_len()
                        {
                            self.data_buffer.push_back(value);
                        } else {
                            log::error!(
                                "Unexpected FDC write in execution phase using port {port:#06X}: {value:#010b}"
                            );
                        }
                    }
                    Phase::Result => {
                        log::error!(
                            "Unexpected FDC write in result phase using port {port:#06X}: {value:#010b}"
                        );
                    }
                }
                self.emit_debug_event(
                    FdcDebugEvent::RegisterWritten {
                        register: Register::Data,
                        value,
                    },
                    self.master_clock,
                );
            }
            _ => {
                log::error!("Unexpected FDC write using port {port:#06X}: {value:#010b}");
            }
        }
    }

    pub fn step(&mut self, master_clock: MasterClockTick) {
        self.master_clock = master_clock;

        let Phase::Execution = self.phase else {
            return;
        };

        let Some(command) = &self.current_command else {
            return;
        };

        if self.data_buffer.len() < command.write_len() {
            // Can't execute command yet, waiting for more data
            return;
        }

        if command.write_len() == 0 && !self.data_buffer.is_empty() {
            // `data_buffer` contains data from read command, wait until host reads it
            return;
        }

        log::debug!("Executing FDC command: {:?}", command);

        let result = match *command {
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
            }
            | Command::ReadDeletedData {
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
                // Data not yet read by host. Stay in execution phase.
                self.command_read_data(
                    multi_track,
                    mode,
                    skip,
                    head,
                    unit_select,
                    chrn,
                    end_of_track,
                    gap_length,
                    data_length,
                    matches!(command, Command::ReadDeletedData { .. }),
                )
            }
            Command::WriteData {
                multi_track,
                mode,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                data_length,
            }
            | Command::WriteDeletedData {
                multi_track,
                mode,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                data_length,
            } => {
                let deleted = matches!(command, Command::WriteDeletedData { .. });
                self.enter_phase(Phase::Result);
                self.command_write_data(
                    multi_track,
                    mode,
                    head,
                    unit_select,
                    chrn,
                    end_of_track,
                    gap_length,
                    data_length,
                    deleted,
                )
            }
            Command::ReadTrack {
                mode,
                skip,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                data_length,
            } => {
                // Data not yet read by host. Stay in execution phase.
                self.command_read_track(
                    mode,
                    skip,
                    head,
                    unit_select,
                    chrn,
                    end_of_track,
                    gap_length,
                    data_length,
                )
            }
            Command::ReadId {
                mode,
                head,
                unit_select,
            } => {
                self.enter_phase(Phase::Result);
                self.command_read_id(mode, head, unit_select)
            }
            Command::FormatTrack {
                mode,
                head,
                unit_select,
                number,
                sector,
                gap_length,
                data,
                ..
            } => {
                self.enter_phase(Phase::Result);
                self.command_format_track(mode, head, unit_select, number, sector, gap_length, data)
            }
            Command::ScanEqual {
                multi_track,
                mode,
                skip,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                scan_type,
            }
            | Command::ScanLowOrEqual {
                multi_track,
                mode,
                skip,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                scan_type,
            }
            | Command::ScanHighOrEqual {
                multi_track,
                mode,
                skip,
                head,
                unit_select,
                chrn,
                end_of_track,
                gap_length,
                scan_type,
            } => {
                let low_or_equal = matches!(command, Command::ScanLowOrEqual { .. });
                let high_or_equal = matches!(command, Command::ScanHighOrEqual { .. });
                self.enter_phase(Phase::Result);
                self.command_scan(
                    multi_track,
                    mode,
                    skip,
                    head,
                    unit_select,
                    chrn,
                    end_of_track,
                    gap_length,
                    scan_type,
                    low_or_equal,
                    high_or_equal,
                )
            }
            Command::Recalibrate { unit_select } => {
                self.busy = false;
                self.enter_phase(Phase::Command);
                self.command_recalibrate(unit_select)
            }
            Command::SenseInterruptStatus => {
                self.enter_phase(Phase::Result);
                self.command_sense_interrupt_status()
            }
            Command::Specify {
                step_rate_time,
                head_unload_time,
                head_load_time,
                non_dma_mode,
            } => {
                self.busy = false;
                self.enter_phase(Phase::Command);
                self.command_specify(
                    step_rate_time,
                    head_unload_time,
                    head_load_time,
                    non_dma_mode,
                )
            }
            Command::SenseDriveStatus { head, unit_select } => {
                self.enter_phase(Phase::Result);
                self.command_sense_drive_status(head, unit_select)
            }
            Command::Seek {
                head,
                unit_select,
                new_cylinder_number,
            } => {
                self.busy = false;
                self.enter_phase(Phase::Command);
                self.command_seek(head, unit_select, new_cylinder_number)
            }
            Command::Invalid => {
                self.enter_phase(Phase::Result);
                self.command_invalid()
            }
        };

        log::debug!("FDC command result: {:?}", result);

        self.current_result = Some(result.clone());
        self.result_buffer.extend(result);
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

    fn enter_phase(&mut self, phase: Phase) {
        self.emit_debug_event(
            FdcDebugEvent::PhaseChanged {
                is: phase,
                was: self.phase,
            },
            self.master_clock,
        );
        self.phase = phase;
    }

    #[allow(clippy::too_many_arguments)]
    fn command_read_data(
        &mut self,
        multi_track: bool,
        mode: Mode,
        skip: bool,
        head: u8,
        unit_select: u8,
        mut chrn: Chrn,
        end_of_track: u8,
        _gap_length: u8,
        data_length: u8,
        deleted: bool,
    ) -> CommandResult {
        if multi_track {
            log::error!("Unsupported multi-track flag set");
        }
        if let Mode::FrequencyModulation = mode {
            log::error!("Unsupported frequency modulation mode");
        }
        if head != 0 {
            log::error!("Unsupported head number");
            todo!("Return NOT READY in ST0")
        }
        let head_address = 0;

        match self.drives.get(unit_select as usize) {
            Some(drive) => {
                let Some(disk) = &drive.disk else {
                    log::debug!("No disk in drive {}", unit_select);
                    self.enter_phase(Phase::Result);

                    let result = StandardResult::not_ready(chrn);
                    if deleted {
                        return CommandResult::ReadDeletedData(result);
                    } else {
                        return CommandResult::ReadData(result);
                    }
                };

                let track = drive.track;

                let data_length = if chrn.number == 0 {
                    data_length as usize
                } else {
                    128 << chrn.number
                };

                // ST0
                let mut interrupt_code = InterruptCode::NormalTermination;

                // ST1
                let mut end_of_cylinder = false;
                let mut data_error = false;
                let mut no_data = false;
                let mut missing_address_mark = false;

                // ST2
                let mut control_mark = false;
                let mut data_error_in_data_field = false;
                let mut wrong_cylinder = false;
                let mut bad_cylinder = false;
                let mut missing_address_mark_in_data_field = false;

                loop {
                    let sector = match disk.tracks[track].find_sector(chrn, false) {
                        Some(sector) => sector,
                        None => {
                            self.enter_phase(Phase::Result);
                            no_data = true;
                            interrupt_code = InterruptCode::AbnormalTermination;
                            log::debug!("Sector ID {} not found", chrn);
                            break;
                        }
                    };

                    let sector_info = &disk.tracks[track].sector_infos[sector];

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

                    let deleted_data_address_mark = sector_info.fdc_status2 & 0b0100_0000 != 0;

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

                    log::debug!(
                        "Reading sector ID {}:{}:{} -> ST1={:08b} ST2={:08b}",
                        chrn.cylinder_number,
                        chrn.head_address,
                        chrn.record,
                        sector_info.fdc_status1,
                        sector_info.fdc_status2,
                    );

                    if interrupt_code == InterruptCode::AbnormalTermination {
                        log::debug!(
                            "Abnormal termination on sector ID {}:{}:{}",
                            chrn.cylinder_number,
                            chrn.head_address,
                            chrn.record
                        );
                        self.enter_phase(Phase::Result);
                        break;
                    }

                    let sector_data = &disk.tracks[track].sectors[sector];

                    if data_length > sector_data.len() {
                        log::error!("Specified data length exceeds physical sector size");
                    }

                    if deleted_data_address_mark == deleted || !skip {
                        self.data_buffer
                            .extend(sector_data.iter().take(data_length));

                        if deleted_data_address_mark != deleted {
                            control_mark = true;
                        }
                    }

                    if chrn.record < end_of_track {
                        chrn.record += 1;
                    } else {
                        end_of_cylinder = true;
                        interrupt_code = InterruptCode::AbnormalTermination;
                        chrn.cylinder_number += 1;
                        chrn.record = 1;
                    }

                    if end_of_cylinder || control_mark {
                        log::debug!("Read {} bytes from disk", self.data_buffer.len());
                        break;
                    }
                }

                let result = StandardResult {
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
                };
                if deleted {
                    CommandResult::ReadDeletedData(result)
                } else {
                    CommandResult::ReadData(result)
                }
            }
            None => {
                log::debug!("Drive {} not connected", unit_select);
                self.enter_phase(Phase::Result);

                let result = StandardResult::not_ready(chrn);
                if deleted {
                    CommandResult::ReadDeletedData(result)
                } else {
                    CommandResult::ReadData(result)
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments, unused_variables)]
    fn command_write_data(
        &mut self,
        multi_track: bool,
        mode: Mode,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        gap_length: u8,
        data_length: u8,
        deleted: bool,
    ) -> CommandResult {
        todo!("support write commands") // TODO: remove allow(unused_variables) when implemented
    }

    #[allow(clippy::too_many_arguments)]
    fn command_read_track(
        &mut self,
        mode: Mode,
        skip: bool,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        _gap_length: u8,
        data_length: u8,
    ) -> CommandResult {
        if let Mode::FrequencyModulation = mode {
            log::error!("Unsupported frequency modulation mode");
        }
        if skip {
            log::warn!("ReadTrack FDC command ignores skip flag");
        }
        if head != 0 {
            log::error!("Unsupported head number");
            todo!("Return NOT READY in ST0")
        }
        let head_address = 0;

        match self.drives.get(unit_select as usize) {
            Some(drive) => {
                let Some(disk) = &drive.disk else {
                    self.enter_phase(Phase::Result);

                    let result = StandardResult::not_ready(chrn);
                    return CommandResult::ReadTrack(result);
                };

                let track = drive.track;

                let data_length = if chrn.number == 0 {
                    data_length as usize
                } else {
                    128 << chrn.number
                };

                // ST0
                let mut interrupt_code = InterruptCode::NormalTermination;

                // ST1
                let mut no_data = true; // reset below if specified sector ID found
                let mut missing_address_mark = true; // reset below if any sector ID found

                for sector in 0..end_of_track as usize {
                    if sector >= disk.tracks[track].sectors.len() {
                        log::error!("Specified end of track exceeds physical track length");
                        break;
                    }

                    if missing_address_mark {
                        // first valid sector found
                        missing_address_mark = false;
                    }

                    let sector_info = &disk.tracks[track].sector_infos[sector];

                    if sector_info.chrn == chrn {
                        // specified sector ID found
                        no_data = false;
                    }

                    let sector_data = &disk.tracks[track].sectors[sector];

                    if data_length > sector_data.len() {
                        log::error!("Specified data length exceeds physical sector size");
                    }

                    self.data_buffer
                        .extend(sector_data.iter().take(data_length));
                }

                if missing_address_mark || no_data {
                    interrupt_code = InterruptCode::AbnormalTermination;
                }

                if self.data_buffer.is_empty() {
                    self.enter_phase(Phase::Result);
                }

                let result = StandardResult {
                    st0: StatusRegister0 {
                        interrupt_code,
                        head_address,
                        unit_select,
                        ..Default::default()
                    },
                    st1: StatusRegister1 {
                        no_data,
                        missing_address_mark,
                        ..Default::default()
                    },
                    st2: StatusRegister2 {
                        ..Default::default()
                    },
                    chrn,
                };
                CommandResult::ReadTrack(result)
            }
            None => {
                self.enter_phase(Phase::Result);

                let result = StandardResult::not_ready(chrn);
                CommandResult::ReadTrack(result)
            }
        }
    }

    fn command_read_id(&mut self, mode: Mode, head: u8, unit_select: u8) -> CommandResult {
        if let Mode::FrequencyModulation = mode {
            log::error!("Unsupported frequency modulation mode");
        }
        if head != 0 {
            log::error!("Unsupported head number");
            todo!("Return NOT READY in ST0")
        }
        let head_address = 0;

        match self.drives.get(unit_select as usize) {
            Some(drive) => {
                let Some(disk) = &drive.disk else {
                    let chrn = Chrn {
                        cylinder_number: 0,
                        head_address: 0,
                        record: 0,
                        number: 0,
                    };

                    let result = StandardResult::not_ready(chrn);
                    return CommandResult::ReadId(result);
                };

                let track = drive.track;

                // ST0
                let mut interrupt_code = InterruptCode::NormalTermination;

                // ST1
                let mut data_error = false;
                let mut no_data = false;
                let mut missing_address_mark = false;

                let chrn = if disk.tracks[track].sector_infos.is_empty() {
                    no_data = true;
                    interrupt_code = InterruptCode::AbnormalTermination;
                    Chrn {
                        cylinder_number: 0,
                        head_address: 0,
                        record: 0,
                        number: 0,
                    }
                } else {
                    let sector_info = &disk.tracks[track].sector_infos[0];

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

                    if interrupt_code == InterruptCode::NormalTermination {
                        sector_info.chrn
                    } else {
                        Chrn {
                            cylinder_number: 0,
                            head_address: 0,
                            record: 0,
                            number: 0,
                        }
                    }
                };

                let result = StandardResult {
                    st0: StatusRegister0 {
                        interrupt_code,
                        head_address,
                        unit_select,
                        ..Default::default()
                    },
                    st1: StatusRegister1 {
                        data_error,
                        no_data,
                        missing_address_mark,
                        ..Default::default()
                    },
                    st2: StatusRegister2 {
                        ..Default::default()
                    },
                    chrn,
                };
                CommandResult::ReadId(result)
            }
            None => {
                let chrn = Chrn {
                    cylinder_number: 0,
                    head_address: 0,
                    record: 0,
                    number: 0,
                };

                let result = StandardResult::not_ready(chrn);
                CommandResult::ReadId(result)
            }
        }
    }

    #[allow(clippy::too_many_arguments, unused_variables)]
    fn command_format_track(
        &mut self,
        mode: Mode,
        head: u8,
        unit_select: u8,
        number: u8,
        sector: u8,
        gap_length: u8,
        data: u8,
    ) -> CommandResult {
        todo!("support write commands") // TODO: remove allow(unused_variables) when implemented
    }

    #[allow(clippy::too_many_arguments, unused_variables)]
    fn command_scan(
        &mut self,
        multi_track: bool,
        mode: Mode,
        skip: bool,
        head: u8,
        unit_select: u8,
        chrn: Chrn,
        end_of_track: u8,
        gap_length: u8,
        scan_type: u8,
        low_or_equal: bool,
        high_or_equal: bool,
    ) -> CommandResult {
        todo!("support scan commands") // TODO: remove allow(unused_variables) when implemented
    }

    fn command_recalibrate(&mut self, unit_select: u8) -> CommandResult {
        let head_address = 0;

        match self.drives.get_mut(unit_select as usize) {
            Some(drive) => {
                let Some(_disk) = &drive.disk else {
                    let interrupt_code = InterruptCode::AbnormalTermination;
                    let not_ready = true;

                    self.interrupt_status = Some(StatusRegister0 {
                        interrupt_code,
                        not_ready,
                        head_address,
                        unit_select,
                        ..Default::default()
                    });

                    return CommandResult::Recalibrate;
                };

                drive.busy = true;
                drive.track = drive.track.saturating_sub(77);

                let mut interrupt_code = InterruptCode::NormalTermination;
                let seek_end = true;
                let mut equipment_check = false;

                if drive.track != 0 {
                    interrupt_code = InterruptCode::AbnormalTermination;
                    equipment_check = true;
                }

                self.interrupt_status = Some(StatusRegister0 {
                    interrupt_code,
                    seek_end,
                    equipment_check,
                    head_address,
                    unit_select,
                    ..Default::default()
                });

                CommandResult::Recalibrate
            }
            None => {
                let interrupt_code = InterruptCode::AbnormalTermination;
                let not_ready = true;

                self.interrupt_status = Some(StatusRegister0 {
                    interrupt_code,
                    not_ready,
                    head_address,
                    unit_select,
                    ..Default::default()
                });

                CommandResult::Recalibrate
            }
        }
    }

    fn command_sense_interrupt_status(&mut self) -> CommandResult {
        match self.interrupt_status.take() {
            Some(st0) => {
                let pcn = self
                    .drives
                    .get(st0.unit_select as usize)
                    .map_or(0, |drive| drive.track as u8);
                CommandResult::SenseInterruptStatus { st0, pcn }
            }
            None => {
                log::warn!("Sense Interrupt Status command called without pending interrupt");
                let interrupt_code = InterruptCode::InvalidCommand;

                CommandResult::Invalid {
                    st0: StatusRegister0 {
                        interrupt_code,
                        ..Default::default()
                    },
                }
            }
        }
    }

    fn command_specify(
        &mut self,
        step_rate_time: u8,
        head_unload_time: u8,
        head_load_time: u8,
        non_dma_mode: bool,
    ) -> CommandResult {
        if !non_dma_mode {
            log::error!("DMA mode is not supported");
        }

        self.step_rate_time = step_rate_time;
        self.head_unload_time = head_unload_time;
        self.head_load_time = head_load_time;
        self.non_dma_mode = non_dma_mode;

        CommandResult::Specify
    }

    fn command_sense_drive_status(&mut self, head: u8, unit_select: u8) -> CommandResult {
        if head != 0 {
            log::error!("Unsupported head number");
        }
        let head_address = 0;

        match self.drives.get_mut(unit_select as usize) {
            Some(drive) => {
                let Some(_disk) = &drive.disk else {
                    let st3 = StatusRegister3 {
                        head_address,
                        unit_select,
                        ..Default::default()
                    };
                    return CommandResult::SenseDriveStatus { st3 };
                };

                let ready = true;
                let track_zero = drive.track == 0;

                let st3 = StatusRegister3 {
                    ready,
                    track_zero,
                    head_address,
                    unit_select,
                    ..Default::default()
                };
                CommandResult::SenseDriveStatus { st3 }
            }
            None => {
                let st3 = StatusRegister3 {
                    head_address,
                    unit_select,
                    ..Default::default()
                };
                CommandResult::SenseDriveStatus { st3 }
            }
        }
    }

    fn command_seek(
        &mut self,
        head: u8,
        unit_select: u8,
        new_cylinder_number: u8,
    ) -> CommandResult {
        if head != 0 {
            log::error!("Unsupported head number");
            todo!("Return NOT READY in ST0")
        }
        let head_address = 0;

        match self.drives.get_mut(unit_select as usize) {
            Some(drive) => {
                let Some(_disk) = &drive.disk else {
                    let interrupt_code = InterruptCode::AbnormalTermination;
                    let not_ready = true;

                    self.interrupt_status = Some(StatusRegister0 {
                        interrupt_code,
                        not_ready,
                        head_address,
                        unit_select,
                        ..Default::default()
                    });

                    return CommandResult::Seek;
                };

                drive.busy = true;
                drive.track = new_cylinder_number as usize;

                let interrupt_code = InterruptCode::NormalTermination;
                let seek_end = true;
                self.interrupt_status = Some(StatusRegister0 {
                    interrupt_code,
                    seek_end,
                    head_address,
                    unit_select,
                    ..Default::default()
                });

                CommandResult::Seek
            }
            None => {
                let interrupt_code = InterruptCode::AbnormalTermination;
                let not_ready = true;

                self.interrupt_status = Some(StatusRegister0 {
                    interrupt_code,
                    not_ready,
                    head_address,
                    unit_select,
                    ..Default::default()
                });

                CommandResult::Seek
            }
        }
    }

    fn command_invalid(&mut self) -> CommandResult {
        log::error!("Invalid FDC command received");

        let interrupt_code = InterruptCode::InvalidCommand;

        CommandResult::Invalid {
            st0: StatusRegister0 {
                interrupt_code,
                ..Default::default()
            },
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
        if self.busy {
            value |= 1 << 4;
        }

        // Drive busy flags
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
}

impl Snapshottable for FloppyDiskController {
    type View = FdcDebugView;

    fn debug_view(&self) -> Self::View {
        let main_status_register = self.report_main_status_register();
        let phase = self.phase;
        let command_buffer = self.command_buffer.clone();
        let data_buffer = self.data_buffer.iter().copied().collect();
        let result_buffer = self.result_buffer.iter().copied().collect();
        let current_command = self.current_command.clone();
        let current_result = self.current_result.clone();
        let motors_on = self.motors_on;
        let drive_a_track = self.drives.first().map(|d| d.track);
        let drive_b_track = self.drives.get(1).map(|d| d.track);

        Self::View {
            main_status_register,
            phase,
            command_buffer,
            data_buffer,
            result_buffer,
            current_command,
            current_result,
            motors_on,
            drive_a_track,
            drive_b_track,
        }
    }
}

impl Debuggable for FloppyDiskController {
    const SOURCE: DebugSource = DebugSource::Fdc;
    type Event = FdcDebugEvent;
}

#[cfg(test)]
mod tests {
    use crate::system::{bus::fdc::dsk_file::DiskBuilder, clock::MasterClock};

    use super::*;

    const MSR_RQM: u8 = 0b1000_0000;
    const MSR_DIO: u8 = 0b0100_0000;
    const MSR_EXM: u8 = 0b0010_0000;

    #[derive(Default)]
    struct FdcHost {
        clock: MasterClock,
        fdc: FloppyDiskController,
    }

    impl FdcHost {
        fn wait_for_rqm(&mut self) -> u8 {
            for _ in 0..1_000 {
                self.clock.step(1);
                self.fdc.step(self.clock.current());

                let msr = self.fdc.read_byte(Register::MainStatus as u16);
                if msr & MSR_RQM != 0 {
                    return msr;
                }
            }

            panic!("FDC did not become ready in time");
        }

        fn write_command(&mut self, command: &Command) {
            let bytes = Vec::from(command);

            for byte in bytes {
                let msr = self.wait_for_rqm();

                if msr & MSR_DIO == 0 {
                    self.fdc.write_byte(Register::Data as u16, byte);
                } else {
                    panic!("Trying to write when FDC expects read")
                }
            }
        }

        fn read_data(&mut self, expected_len: usize) -> Vec<u8> {
            let mut data = Vec::with_capacity(expected_len);

            for _ in 0..(expected_len + 1) {
                let msr = self.wait_for_rqm();

                if msr & MSR_DIO != 0 {
                    if msr & MSR_EXM == 0 {
                        return data;
                    }

                    let byte = self.fdc.read_byte(Register::Data as u16);
                    data.push(byte);
                } else {
                    if !data.is_empty() {
                        return data;
                    }

                    panic!("Trying to read when FDC expects write")
                }
            }

            panic!("FDC has more data from execution phase than expected")
        }

        fn read_result(&mut self, expected_len: usize) -> Vec<u8> {
            let mut data = Vec::with_capacity(expected_len);

            for _ in 0..(expected_len + 1) {
                let msr = self.wait_for_rqm();

                if (msr & MSR_DIO != 0) && (msr & MSR_EXM == 0) {
                    let byte = self.fdc.read_byte(Register::Data as u16);
                    data.push(byte);
                } else {
                    return data;
                }
            }

            panic!("FDC has more data from result phase than expected")
        }
    }

    #[test]
    fn test_command_read_data_encode_decode() {
        let command = Command::ReadData {
            multi_track: true,
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 1,
            unit_select: 1,
            chrn: Chrn {
                cylinder_number: 42,
                head_address: 1,
                record: 43,
                number: 44,
            },
            end_of_track: 42,
            gap_length: 45,
            data_length: 46,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_read_deleted_data_encode_decode() {
        let command = Command::ReadDeletedData {
            multi_track: true,
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 1,
            unit_select: 1,
            chrn: Chrn {
                cylinder_number: 42,
                head_address: 1,
                record: 43,
                number: 44,
            },
            end_of_track: 42,
            gap_length: 45,
            data_length: 46,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_write_data_encode_decode() {
        let command = Command::WriteData {
            multi_track: true,
            mode: Mode::ModifiedFrequencyModulation,
            head: 1,
            unit_select: 1,
            chrn: Chrn {
                cylinder_number: 42,
                head_address: 1,
                record: 43,
                number: 44,
            },
            end_of_track: 42,
            gap_length: 45,
            data_length: 46,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_write_deleted_data_encode_decode() {
        let command = Command::WriteDeletedData {
            multi_track: true,
            mode: Mode::ModifiedFrequencyModulation,
            head: 1,
            unit_select: 1,
            chrn: Chrn {
                cylinder_number: 42,
                head_address: 1,
                record: 43,
                number: 44,
            },
            end_of_track: 42,
            gap_length: 45,
            data_length: 46,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_read_track_encode_decode() {
        let command = Command::ReadTrack {
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 1,
            unit_select: 1,
            chrn: Chrn {
                cylinder_number: 42,
                head_address: 1,
                record: 43,
                number: 44,
            },
            end_of_track: 42,
            gap_length: 45,
            data_length: 46,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_read_id_encode_decode() {
        let command = Command::ReadId {
            mode: Mode::ModifiedFrequencyModulation,
            head: 1,
            unit_select: 1,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_format_track_encode_decode() {
        let command = Command::FormatTrack {
            mode: Mode::ModifiedFrequencyModulation,
            head: 1,
            unit_select: 1,
            number: 42,
            sector: 43,
            gap_length: 45,
            data: 46,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_scan_equal_encode_decode() {
        let command = Command::ScanEqual {
            multi_track: true,
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 1,
            unit_select: 1,
            chrn: Chrn {
                cylinder_number: 42,
                head_address: 1,
                record: 43,
                number: 44,
            },
            end_of_track: 42,
            gap_length: 45,
            scan_type: 46,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_scan_low_or_equal_encode_decode() {
        let command = Command::ScanLowOrEqual {
            multi_track: true,
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 1,
            unit_select: 1,
            chrn: Chrn {
                cylinder_number: 42,
                head_address: 1,
                record: 43,
                number: 44,
            },
            end_of_track: 42,
            gap_length: 45,
            scan_type: 46,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_scan_high_or_equal_encode_decode() {
        let command = Command::ScanHighOrEqual {
            multi_track: true,
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 1,
            unit_select: 1,
            chrn: Chrn {
                cylinder_number: 42,
                head_address: 1,
                record: 43,
                number: 44,
            },
            end_of_track: 42,
            gap_length: 45,
            scan_type: 46,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_recalibrate_encode_decode() {
        let command = Command::Recalibrate { unit_select: 1 };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_sense_interrupt_status_encode_decode() {
        let command = Command::SenseInterruptStatus;

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_specify_encode_decode() {
        let command = Command::Specify {
            step_rate_time: 8,
            head_unload_time: 9,
            head_load_time: 44,
            non_dma_mode: true,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_sense_drive_status_encode_decode() {
        let command = Command::SenseDriveStatus {
            head: 1,
            unit_select: 1,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_seek_encode_decode() {
        let command = Command::Seek {
            head: 1,
            unit_select: 1,
            new_cylinder_number: 42,
        };

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_invalid_encode_decode() {
        let command = Command::Invalid;

        let encoded = Vec::from(&command);
        let decoded = Command::from(encoded.as_slice());

        assert_eq!(command, decoded);
    }

    #[test]
    fn test_command_recalibrate_without_disk_fails() {
        let mut host = FdcHost::default();
        host.fdc.drives[0].track = 42;

        let command = Command::Recalibrate { unit_select: 0 };
        host.write_command(&command);

        let command = Command::SenseInterruptStatus;
        host.write_command(&command);
        let result = host.read_result(2);

        let expected_result = CommandResult::SenseInterruptStatus {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                seek_end: false,
                equipment_check: false,
                not_ready: true,
                head_address: 0,
                unit_select: 0,
            },
            pcn: 42,
        }
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_recalibrate_with_disk_succeeds() {
        let mut host = FdcHost::default();
        host.fdc.drives[0].disk = Some(DiskBuilder::new().build());
        host.fdc.drives[0].track = 42;

        let command = Command::Recalibrate { unit_select: 0 };
        host.write_command(&command);

        let command = Command::SenseInterruptStatus;
        host.write_command(&command);
        let result = host.read_result(2);

        let expected_result = CommandResult::SenseInterruptStatus {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::NormalTermination,
                seek_end: true,
                equipment_check: false,
                not_ready: false,
                head_address: 0,
                unit_select: 0,
            },
            pcn: 0,
        }
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_without_disk_fails() {
        let mut host = FdcHost::default();

        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn: Chrn {
                cylinder_number: 0,
                head_address: 0,
                record: 0,
                number: 0,
            },
            end_of_track: 0,
            gap_length: 0,
            data_length: 0,
        };
        host.write_command(&command);
        let data = host.read_data(0);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                not_ready: true,
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn: Chrn {
                cylinder_number: 0,
                head_address: 0,
                record: 0,
                number: 0,
            },
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert!(data.is_empty(), "Expected no data to be read");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_on_sector_not_found_fails() {
        let mut host = FdcHost::default();
        host.fdc.drives[0].disk = Some(DiskBuilder::new().add_track(0).build());

        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn: Chrn {
                cylinder_number: 0,
                head_address: 0,
                record: 0,
                number: 0,
            },
            end_of_track: 0,
            gap_length: 0,
            data_length: 0,
        };
        host.write_command(&command);
        let data = host.read_data(0);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                no_data: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn: Chrn {
                cylinder_number: 0,
                head_address: 0,
                record: 0,
                number: 0,
            },
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert!(data.is_empty(), "Expected no data to be read");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_if_sector_has_data_error_fails() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 0,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, Vec::new(), 0b0010_0000, 0b0000_0000)
                .build(),
        );

        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 0,
            gap_length: 0,
            data_length: 0,
        };
        host.write_command(&command);
        let data = host.read_data(0);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                data_error: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert!(data.is_empty(), "Expected no data to be read");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_if_sector_misses_address_mark_fails() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 0,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, Vec::new(), 0b0000_0001, 0b0000_0000)
                .build(),
        );

        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 0,
            gap_length: 0,
            data_length: 0,
        };
        host.write_command(&command);
        let data = host.read_data(0);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                missing_address_mark: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert!(data.is_empty(), "Expected no data to be read");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_if_sector_has_data_error_in_data_field_fails() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 0,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, Vec::new(), 0b0000_0000, 0b0010_0000)
                .build(),
        );

        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 0,
            gap_length: 0,
            data_length: 0,
        };
        host.write_command(&command);
        let data = host.read_data(0);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                data_error_in_data_field: true,
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert!(data.is_empty(), "Expected no data to be read");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_if_sector_misses_address_mark_in_data_field_fails() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 0,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, Vec::new(), 0b0000_0000, 0b0000_0001)
                .build(),
        );

        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 0,
            gap_length: 0,
            data_length: 0,
        };
        host.write_command(&command);
        let data = host.read_data(0);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                missing_address_mark_in_data_field: true,
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert!(data.is_empty(), "Expected no data to be read");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_on_wrong_cylinder_fails() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 0,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(
                    Chrn {
                        cylinder_number: 0xff,
                        ..chrn
                    },
                    Vec::new(),
                    0b0000_0000,
                    0b0000_0000,
                )
                .build(),
        );

        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 0,
            gap_length: 0,
            data_length: 0,
        };
        host.write_command(&command);
        let data = host.read_data(0);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                no_data: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                wrong_cylinder: true,
                bad_cylinder: true,
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert!(data.is_empty(), "Expected no data to be read");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_reads_normal_sector() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0000_0000)
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 2,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(3);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                end_of_cylinder: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn: Chrn {
                cylinder_number: 1,
                head_address: 0,
                record: 1,
                number: 0,
            },
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_reads_deleted_sector_and_terminates() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0100_0000)
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 3,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(3);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                control_mark: true,
                ..Default::default()
            },
            chrn: Chrn {
                cylinder_number: 0,
                head_address: 0,
                record: 3,
                number: 0,
            },
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_reads_multiple_sectors() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0000_0000)
                .with_sector(
                    Chrn { record: 3, ..chrn },
                    vec![0xd, 0xe, 0xf],
                    0b0000_0000,
                    0b0000_0000,
                )
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 3,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(6);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                end_of_cylinder: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn: Chrn {
                cylinder_number: 1,
                head_address: 0,
                record: 1,
                number: 0,
            },
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc, 0xd, 0xe, 0xf]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_data_skips_deleted_sector() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0100_0000)
                .with_sector(
                    Chrn { record: 3, ..chrn },
                    vec![0xd, 0xe, 0xf],
                    0b0000_0000,
                    0b0000_0000,
                )
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 3,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(3);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                end_of_cylinder: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn: Chrn {
                cylinder_number: 1,
                head_address: 0,
                record: 1,
                number: 0,
            },
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xd, 0xe, 0xf]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_deleted_data_reads_deleted_sector() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0100_0000)
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadDeletedData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 2,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(3);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadDeletedData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                end_of_cylinder: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn: Chrn {
                cylinder_number: 1,
                head_address: 0,
                record: 1,
                number: 0,
            },
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_deleted_data_reads_normal_sector_and_terminates() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0000_0000)
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadDeletedData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 3,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(3);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadDeletedData(StandardResult {
            st0: StatusRegister0 {
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                control_mark: true,
                ..Default::default()
            },
            chrn: Chrn {
                cylinder_number: 0,
                head_address: 0,
                record: 3,
                number: 0,
            },
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_deleted_data_reads_multiple_deleted_sectors() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0100_0000)
                .with_sector(
                    Chrn { record: 3, ..chrn },
                    vec![0xd, 0xe, 0xf],
                    0b0000_0000,
                    0b0100_0000,
                )
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadDeletedData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 3,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(6);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadDeletedData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                end_of_cylinder: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn: Chrn {
                cylinder_number: 1,
                head_address: 0,
                record: 1,
                number: 0,
            },
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc, 0xd, 0xe, 0xf]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_deleted_data_skips_normal_sector() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0000_0000)
                .with_sector(
                    Chrn { record: 3, ..chrn },
                    vec![0xd, 0xe, 0xf],
                    0b0000_0000,
                    0b0100_0000,
                )
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadDeletedData {
            multi_track: false,
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 3,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(3);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadDeletedData(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                end_of_cylinder: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn: Chrn {
                cylinder_number: 1,
                head_address: 0,
                record: 1,
                number: 0,
            },
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xd, 0xe, 0xf]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_track_without_disk_fails() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadTrack {
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 3,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(0);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadTrack(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                not_ready: true,
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert!(data.is_empty(), "Expected no data to be read");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_track_with_empty_track_signals_missing_address_mark() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(DiskBuilder::new().add_track(0).build());

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadTrack {
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 1,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(0);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadTrack(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                missing_address_mark: true,
                no_data: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert!(data.is_empty(), "Expected no data to be read");
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_track_with_missing_sector_signals_no_data() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(
                    Chrn { record: 3, ..chrn },
                    vec![0xa, 0xb, 0xc],
                    0b0000_0000,
                    0b0000_0000,
                )
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadTrack {
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 1,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(3);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadTrack(StandardResult {
            st0: StatusRegister0 {
                interrupt_code: InterruptCode::AbnormalTermination,
                ..Default::default()
            },
            st1: StatusRegister1 {
                no_data: true,
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_track_with_ordered_sectors_succeeds() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0000_0000)
                .with_sector(
                    Chrn { record: 3, ..chrn },
                    vec![0xd, 0xe, 0xf],
                    0b0000_0000,
                    0b0000_0000,
                )
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadTrack {
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 2,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(6);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadTrack(StandardResult {
            st0: StatusRegister0 {
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc, 0xd, 0xe, 0xf]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_track_with_unordered_sectors_succeeds() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0000_0000)
                .with_sector(
                    Chrn { record: 1, ..chrn },
                    vec![0xd, 0xe, 0xf],
                    0b0000_0000,
                    0b0000_0000,
                )
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadTrack {
            mode: Mode::ModifiedFrequencyModulation,
            skip: false,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 2,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(6);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadTrack(StandardResult {
            st0: StatusRegister0 {
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc, 0xd, 0xe, 0xf]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_track_with_deleted_sectors_succeeds() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0000_0000)
                .with_sector(
                    Chrn { record: 3, ..chrn },
                    vec![0xd, 0xe, 0xf],
                    0b0000_0000,
                    0b0100_0000,
                )
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadTrack {
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 2,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(6);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadTrack(StandardResult {
            st0: StatusRegister0 {
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc, 0xd, 0xe, 0xf]);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_command_read_track_with_crc_errors_succeeds() {
        let mut host = FdcHost::default();
        let chrn = Chrn {
            cylinder_number: 0,
            head_address: 0,
            record: 2,
            number: 0,
        };
        host.fdc.drives[0].disk = Some(
            DiskBuilder::new()
                .add_track(0)
                .with_sector(chrn, vec![0xa, 0xb, 0xc], 0b0000_0000, 0b0000_0000)
                .with_sector(
                    Chrn { record: 3, ..chrn },
                    vec![0xd, 0xe, 0xf],
                    0b0010_0000,
                    0b0000_0000,
                )
                .build(),
        );

        let command = Command::Specify {
            step_rate_time: 0,
            head_unload_time: 0,
            head_load_time: 0,
            non_dma_mode: true,
        };
        host.write_command(&command);
        let command = Command::ReadTrack {
            mode: Mode::ModifiedFrequencyModulation,
            skip: true,
            head: 0,
            unit_select: 0,
            chrn,
            end_of_track: 2,
            gap_length: 0,
            data_length: 3,
        };
        host.write_command(&command);
        let data = host.read_data(6);
        let result = host.read_result(7);

        let expected_result = CommandResult::ReadTrack(StandardResult {
            st0: StatusRegister0 {
                ..Default::default()
            },
            st1: StatusRegister1 {
                ..Default::default()
            },
            st2: StatusRegister2 {
                ..Default::default()
            },
            chrn,
        })
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(data, vec![0xa, 0xb, 0xc, 0xd, 0xe, 0xf]);
        assert_eq!(result, expected_result);
    }
}

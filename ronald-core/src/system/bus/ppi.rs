use serde::{Deserialize, Serialize};

use crate::system::bus::crtc::CrtController;
use crate::system::bus::keyboard::Keyboard;
use crate::system::bus::psg::SoundGenerator;
use crate::system::bus::tape::TapeController;

#[derive(PartialEq, Serialize, Deserialize)]
enum Direction {
    Input,
    Output,
}

#[derive(Serialize, Deserialize)]
enum Mode {
    Basic,
    Strobed,
    Bidirectional,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeripheralInterface {
    // Peripheral input/output
    direction_a: Direction,
    direction_b: Direction,
    direction_c_lower: Direction,
    direction_c_upper: Direction,
    mode_a_and_c_upper: Mode,
    mode_b_and_c_lower: Mode,
}

impl PeripheralInterface {
    pub fn new() -> Self {
        PeripheralInterface {
            direction_a: Direction::Input,
            direction_b: Direction::Input,
            direction_c_lower: Direction::Input,
            direction_c_upper: Direction::Input,
            mode_a_and_c_upper: Mode::Basic,
            mode_b_and_c_lower: Mode::Basic,
        }
    }

    pub fn read_byte(
        &self,
        crtc: &CrtController,
        psg: &SoundGenerator,
        tape: &TapeController,
        port: u16,
    ) -> u8 {
        let function = (port >> 8) & 0x03;

        match function {
            0 => {
                if self.direction_a == Direction::Input {
                    psg.read_byte()
                } else {
                    0
                }
            }
            1 => {
                if self.direction_b == Direction::Input {
                    let mut value = 0x07 << 1 | 0x01 << 4; // distributor ID: Amstrad, 50 Hz monitor

                    if crtc.read_vertical_sync() {
                        value |= 0x01;
                    }

                    if tape.read_sample() {
                        value |= 0x01 << 7;
                    }

                    value
                } else {
                    0
                }
            }
            2 => 0,
            3 => 0,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(
        &mut self,
        keyboard: &mut Keyboard,
        psg: &mut SoundGenerator,
        tape: &mut TapeController,
        port: u16,
        value: u8,
    ) {
        let function = (port >> 8) & 0x03;

        match function {
            0 => {
                if self.direction_a == Direction::Output {
                    psg.write_byte(value);
                }
            }
            1 => (),
            2 => {
                if self.direction_c_lower == Direction::Output {
                    keyboard.set_active_line(value as usize & 0x0f);
                }

                if self.direction_c_upper == Direction::Output {
                    psg.perform_function(keyboard, (value >> 6) & 0x03);
                    tape.write_sample((value >> 5) & 0x01 != 0);
                    tape.switch_motor(value & 0x10 != 0);
                }
            }
            3 => {
                if value & 0x80 != 0 {
                    if value & 0x01 != 0 {
                        self.direction_c_lower = Direction::Input;
                    } else {
                        self.direction_c_lower = Direction::Output;
                    }

                    if value & 0x02 != 0 {
                        self.direction_b = Direction::Input;
                    } else {
                        self.direction_b = Direction::Output;
                    }

                    if value & 0x04 != 0 {
                        self.mode_b_and_c_lower = Mode::Strobed;
                    } else {
                        self.mode_b_and_c_lower = Mode::Basic;
                    }

                    if value & 0x08 != 0 {
                        self.direction_c_upper = Direction::Input;
                    } else {
                        self.direction_c_upper = Direction::Output;
                    }

                    if value & 0x10 != 0 {
                        self.direction_a = Direction::Input;
                    } else {
                        self.direction_a = Direction::Output;
                    }

                    self.mode_a_and_c_upper = match (value & 0x60) >> 5 {
                        0 => Mode::Basic,
                        1 => Mode::Strobed,
                        _ => Mode::Bidirectional,
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

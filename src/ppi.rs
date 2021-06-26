use std::cell::RefCell;
use std::rc::Rc;

use crate::crtc;
use crate::keyboard;
use crate::psg;
use crate::tape;

pub type PeripheralInterfaceShared = Rc<RefCell<PeripheralInterface>>;

#[derive(PartialEq)]
enum Direction {
    Input,
    Output,
}

enum Mode {
    Basic,
    Strobed,
    Bidirectional,
}

pub struct PeripheralInterface {
    // Peripheral input/output
    direction_a: Direction,
    direction_b: Direction,
    direction_c_lower: Direction,
    direction_c_upper: Direction,
    mode_a_and_c_upper: Mode,
    mode_b_and_c_lower: Mode,
    crtc: crtc::CRTControllerShared,
    keyboard: keyboard::KeyboardShared,
    psg: psg::SoundGeneratorShared,
    tape: tape::TapeControllerShared,
}

impl PeripheralInterface {
    pub fn new_shared(
        crtc: crtc::CRTControllerShared,
        keyboard: keyboard::KeyboardShared,
        psg: psg::SoundGeneratorShared,
        tape: tape::TapeControllerShared,
    ) -> PeripheralInterfaceShared {
        let ppi = PeripheralInterface {
            direction_a: Direction::Input,
            direction_b: Direction::Input,
            direction_c_lower: Direction::Input,
            direction_c_upper: Direction::Input,
            mode_a_and_c_upper: Mode::Basic,
            mode_b_and_c_lower: Mode::Basic,
            crtc,
            keyboard,
            psg,
            tape,
        };

        Rc::new(RefCell::new(ppi))
    }

    pub fn read_byte(&self, port: u16) -> u8 {
        let function = (port >> 8) & 0x03;

        match function {
            0 => {
                if self.direction_a == Direction::Input {
                    self.psg.borrow().read_byte()
                } else {
                    0
                }
            }
            1 => {
                if self.direction_b == Direction::Input {
                    let mut value = 0x07 << 1 | 0x01 << 4; // distributor ID: Amstrad, 50 Hz monitor

                    if self.crtc.borrow().read_vertical_sync() {
                        value |= 0x01;
                    }

                    if self.tape.borrow().read_sample() {
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

    pub fn write_byte(&mut self, port: u16, value: u8) {
        let function = (port >> 8) & 0x03;

        match function {
            0 => {
                if self.direction_a == Direction::Output {
                    self.psg.borrow_mut().write_byte(value);
                }
            }
            1 => (),
            2 => {
                if self.direction_c_lower == Direction::Output {
                    self.keyboard
                        .borrow_mut()
                        .set_active_line(value as usize & 0x0f);
                }

                if self.direction_c_upper == Direction::Output {
                    self.psg.borrow_mut().perform_function((value & 0xc0) >> 3);
                    self.tape
                        .borrow_mut()
                        .write_sample((value >> 5) & 0x01 != 0);
                    self.tape.borrow_mut().switch_motor(value & 0x10 != 0);
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

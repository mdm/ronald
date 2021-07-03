use std::cell::RefCell;
use std::rc::Rc;

pub const BUFFER_WIDTH: usize = 64 * 16;
pub const BUFFER_HEIGHT: usize = 39 * 16;

#[allow(clippy::identity_op, clippy::eq_op)]
const FIRMWARE_COLORS: [u32; 27] = [
    0x00 << 16 | 0x00 << 8 | 0x00, // 0
    0x00 << 16 | 0x00 << 8 | 0x80, // 1
    0x00 << 16 | 0x00 << 8 | 0xff, // 2
    0x80 << 16 | 0x00 << 8 | 0x00, // 3
    0x80 << 16 | 0x00 << 8 | 0x80, // 4
    0x80 << 16 | 0x00 << 8 | 0xff, // 5
    0xff << 16 | 0x00 << 8 | 0x00, // 6
    0xff << 16 | 0x00 << 8 | 0x80, // 7
    0xff << 16 | 0x00 << 8 | 0xff, // 8
    0x00 << 16 | 0x80 << 8 | 0x00, // 9
    0x00 << 16 | 0x80 << 8 | 0x80, // 10
    0x00 << 16 | 0x80 << 8 | 0xff, // 11
    0x80 << 16 | 0x80 << 8 | 0x00, // 12
    0x80 << 16 | 0x80 << 8 | 0x80, // 13
    0x80 << 16 | 0x80 << 8 | 0xff, // 14
    0xff << 16 | 0x80 << 8 | 0x00, // 15
    0xff << 16 | 0x80 << 8 | 0x80, // 16
    0xff << 16 | 0x80 << 8 | 0xff, // 17
    0x00 << 16 | 0xff << 8 | 0x00, // 18
    0x00 << 16 | 0xff << 8 | 0x80, // 19
    0x00 << 16 | 0xff << 8 | 0xff, // 20
    0x80 << 16 | 0xff << 8 | 0x00, // 21
    0x80 << 16 | 0xff << 8 | 0x80, // 22
    0x80 << 16 | 0xff << 8 | 0xff, // 23
    0xff << 16 | 0xff << 8 | 0x00, // 24
    0xff << 16 | 0xff << 8 | 0x80, // 25
    0xff << 16 | 0xff << 8 | 0xff, // 26
];

const HARDWARE_TO_FIRMWARE_COLORS: [usize; 32] = [
    13, // 0 (0x40)
    13, // 1 (0x41)
    19, // 2 (0x42)
    25, // 3 (0x43)
    1,  // 4 (0x44)
    7,  // 5 (0x45)
    10, // 6 (0x46)
    16, // 7 (0x47)
    7,  // 8 (0x48)
    25, // 9 (0x49)
    24, // 10 (0x4a)
    26, // 11 (0x4b)
    6,  // 12 (0x4c)
    8,  // 13 (0x4d)
    15, // 14 (0x4e)
    17, // 15 (0x4f)
    1,  // 16 (0x50)
    19, // 17 (0x51)
    18, // 18 (0x52)
    20, // 19 (0x53)
    0,  // 20 (0x54)
    2,  // 21 (0x55)
    9,  // 22 (0x56)
    11, // 23 (0x57)
    4,  // 24 (0x58)
    22, // 25 (0x59)
    21, // 26 (0x5a)
    23, // 27 (0x5b)
    3,  // 28 (0x5c)
    5,  // 29 (0x5d)
    12, // 30 (0x5e)
    14, // 31 (0x5f)
];

pub type ScreenShared = Rc<RefCell<Screen>>;

pub struct Screen {
    buffer: Vec<u32>,
    gun_position: usize,
    waiting_for_vsync: bool,
}

impl Screen {
    pub fn new_shared() -> ScreenShared {
        let screen = Screen {
            buffer: vec![FIRMWARE_COLORS[0]; BUFFER_WIDTH * BUFFER_HEIGHT],
            gun_position: 0,
            waiting_for_vsync: true,
        };

        Rc::new(RefCell::new(screen))
    }

    pub fn get_frame_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn write(&mut self, color: usize) {
        if self.waiting_for_vsync {
            return;
        }

        self.buffer[self.gun_position] = FIRMWARE_COLORS[HARDWARE_TO_FIRMWARE_COLORS[color]];
        self.buffer[self.gun_position + BUFFER_WIDTH] = FIRMWARE_COLORS[HARDWARE_TO_FIRMWARE_COLORS[color]];

        self.gun_position += 1;

        if self.gun_position % BUFFER_WIDTH == 0 {
            self.gun_position += BUFFER_WIDTH;
        }

        if self.gun_position == self.buffer.len() {
            self.gun_position = 0;
            self.waiting_for_vsync = true;
        }
    }

    pub fn trigger_vsync(&mut self) {
        self.waiting_for_vsync = false;
    }
}

pub const BUFFER_WIDTH: usize = 64 * 16;
pub const BUFFER_HEIGHT: usize = 39 * 16;

#[allow(clippy::identity_op, clippy::eq_op)]
const COLORS: [u32; 32] = [
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
    0x00 << 16 | 0x00 << 8 | 0x00, // duplicate
    0x00 << 16 | 0x00 << 8 | 0x00, // duplicate
    0x00 << 16 | 0x00 << 8 | 0x00, // duplicate
    0x00 << 16 | 0x00 << 8 | 0x00, // duplicate
    0x00 << 16 | 0x00 << 8 | 0x00, // duplicate
];

pub struct Screen {
    buffer: Vec<u32>,
    gun_position: usize,
    waiting_for_vsync: bool,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            buffer: vec![COLORS[2]; BUFFER_WIDTH * BUFFER_HEIGHT],
            gun_position: 0,
            waiting_for_vsync: true,
        }
    }

    pub fn get_frame_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    fn write(&mut self, color: usize) {
        if self.waiting_for_vsync {
            return;
        }

        self.buffer[self.gun_position] = COLORS[color];
        self.buffer[self.gun_position + BUFFER_WIDTH] = COLORS[color];

        self.gun_position += 1;

        if self.gun_position % BUFFER_WIDTH == 0 {
            self.gun_position += BUFFER_WIDTH;
        }

        if self.gun_position == self.buffer.len() {
            self.gun_position = 0;
            self.waiting_for_vsync = true;
        }
    }

    fn trigger_vsync(&mut self) {
        self.waiting_for_vsync = false;
    }
}

pub const BUFFER_WIDTH: usize = 64 * 16;
pub const BUFFER_HEIGHT: usize = 39 * 16;

const COLORS: Vec<u32> = vec![
    u32::from_be_bytes([0x00, 0x00, 0x00, 0x00]), // 0
    u32::from_be_bytes([0x00, 0x00, 0x80, 0x00]), // 1
    u32::from_be_bytes([0x00, 0x00, 0xff, 0x00]), // 2
    u32::from_be_bytes([0x80, 0x00, 0x00, 0x00]), // 3
    u32::from_be_bytes([0x80, 0x00, 0x80, 0x00]), // 4
    u32::from_be_bytes([0x80, 0x00, 0xff, 0x00]), // 5
    u32::from_be_bytes([0xff, 0x00, 0x00, 0x00]), // 6
    u32::from_be_bytes([0xff, 0x00, 0x80, 0x00]), // 7
    u32::from_be_bytes([0xff, 0x00, 0xff, 0x00]), // 8
    u32::from_be_bytes([0x00, 0x80, 0x00, 0x00]), // 9
    u32::from_be_bytes([0x00, 0x80, 0x80, 0x00]), // 10
    u32::from_be_bytes([0x00, 0x80, 0xff, 0x00]), // 11
    u32::from_be_bytes([0x80, 0x80, 0x00, 0x00]), // 12
    u32::from_be_bytes([0x80, 0x80, 0x80, 0x00]), // 13
    u32::from_be_bytes([0x80, 0x80, 0xff, 0x00]), // 14
    u32::from_be_bytes([0xff, 0x80, 0x00, 0x00]), // 15
    u32::from_be_bytes([0xff, 0x80, 0x80, 0x00]), // 16
    u32::from_be_bytes([0xff, 0x80, 0xff, 0x00]), // 17
    u32::from_be_bytes([0x00, 0xff, 0x00, 0x00]), // 18
    u32::from_be_bytes([0x00, 0xff, 0x80, 0x00]), // 19
    u32::from_be_bytes([0x00, 0xff, 0xff, 0x00]), // 20
    u32::from_be_bytes([0x80, 0xff, 0x00, 0x00]), // 21
    u32::from_be_bytes([0x80, 0xff, 0x80, 0x00]), // 22
    u32::from_be_bytes([0x80, 0xff, 0xff, 0x00]), // 23
    u32::from_be_bytes([0xff, 0xff, 0x00, 0x00]), // 24
    u32::from_be_bytes([0xff, 0xff, 0x80, 0x00]), // 25
    u32::from_be_bytes([0xff, 0xff, 0xff, 0x00]), // 26
    u32::from_be_bytes([0x00, 0x00, 0x00, 0x00]), // duplicate
    u32::from_be_bytes([0x00, 0x00, 0x00, 0x00]), // duplicate
    u32::from_be_bytes([0x00, 0x00, 0x00, 0x00]), // duplicate
    u32::from_be_bytes([0x00, 0x00, 0x00, 0x00]), // duplicate
    u32::from_be_bytes([0x00, 0x00, 0x00, 0x00]), // duplicate
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
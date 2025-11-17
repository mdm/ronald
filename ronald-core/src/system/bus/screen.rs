use serde::{Deserialize, Serialize};

use crate::VideoSink;
use crate::constants::{
    FIRMWARE_COLORS, HARDWARE_TO_FIRMWARE_COLORS, SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH,
};

const VIRTUAL_BUFFER_WIDTH: usize = 64 * 16;
const VIRTUAL_BUFFER_HEIGHT: usize = 39 * 16;
const BORDER_WIDTH: usize = 4 * 16;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Screen {
    buffer: Vec<u8>,
    gun_position: usize,
    width_counter: usize,
    waiting_for_vsync: bool,
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            buffer: vec![0; SCREEN_BUFFER_WIDTH * SCREEN_BUFFER_HEIGHT * 4],
            gun_position: 0,
            width_counter: 0,
            waiting_for_vsync: true,
        }
    }
}

impl Screen {
    pub fn write(&mut self, color: usize) {
        if self.waiting_for_vsync {
            return;
        }

        let color = FIRMWARE_COLORS[HARDWARE_TO_FIRMWARE_COLORS[color]];
        let buffer_len = self.buffer.len(); // TODO: document why we need this
        let x = (self.gun_position + BORDER_WIDTH) % VIRTUAL_BUFFER_WIDTH;
        let y = (self.gun_position + BORDER_WIDTH) / VIRTUAL_BUFFER_WIDTH;

        if y >= 4 * 16 && x < 48 * 16 {
            let y = y - (4 * 16);
            let start = (y * SCREEN_BUFFER_WIDTH + x) * 4 % buffer_len;
            self.buffer[start..(start + 4)].copy_from_slice(&color);
            let start = (y * SCREEN_BUFFER_WIDTH + x + SCREEN_BUFFER_WIDTH) * 4 % buffer_len;
            self.buffer[start..(start + 4)].copy_from_slice(&color);
        }

        self.gun_position += 1;
        self.width_counter += 1;

        if self.width_counter == VIRTUAL_BUFFER_WIDTH {
            self.gun_position += VIRTUAL_BUFFER_WIDTH;
            self.width_counter = 0;
        }

        if self.gun_position == VIRTUAL_BUFFER_WIDTH * VIRTUAL_BUFFER_HEIGHT {
            self.gun_position = 0;
            self.waiting_for_vsync = true;
        }
    }

    pub fn trigger_vsync(&mut self, video: &mut impl VideoSink) {
        video.draw_frame(&self.buffer);
        self.waiting_for_vsync = false;
    }
}

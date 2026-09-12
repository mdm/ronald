use serde::{Deserialize, Serialize};

use crate::VideoSink;
use crate::constants::{HARDWARE_COLORS, SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

// virtual frame buffer, includes hsync and vsync
const VIRTUAL_BUFFER_WIDTH: usize = 64 * 16;
const VIRTUAL_BUFFER_HEIGHT: usize = 39 * 16;
const BORDER_WIDTH: usize = 4 * 16;
const BORDER_HEIGHT: usize = 4 * 16;
const FRAME_LENGTH: usize = VIRTUAL_BUFFER_WIDTH * VIRTUAL_BUFFER_HEIGHT;

// physical frame buffer
const ROW_LENGTH: usize = SCREEN_BUFFER_WIDTH * 4;
const VISIBLE_LENGTH: usize = ROW_LENGTH * SCREEN_BUFFER_HEIGHT;
const BUFFER_LENGTH: usize = VISIBLE_LENGTH + ROW_LENGTH; // one spare row for the last scan line's tail, which wraps to the first visible row

const PIXELS_PER_WRITE: usize = 16;

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
            buffer: vec![0; BUFFER_LENGTH],
            gun_position: 0,
            width_counter: 0,
            waiting_for_vsync: true,
        }
    }
}

impl Screen {
    #[inline]
    pub fn write(&mut self, colors: &[u8; PIXELS_PER_WRITE]) {
        if self.waiting_for_vsync {
            return;
        }

        let position = self.gun_position + BORDER_WIDTH;
        let x = position % VIRTUAL_BUFFER_WIDTH;
        let y = position / VIRTUAL_BUFFER_WIDTH;

        if y >= BORDER_HEIGHT && x < SCREEN_BUFFER_WIDTH {
            let start = ((y - BORDER_HEIGHT) * SCREEN_BUFFER_WIDTH + x) * 4;
            let end = start + PIXELS_PER_WRITE * 4;
            for (pixel, color) in self.buffer[start..end]
                .as_chunks_mut::<4>()
                .0
                .iter_mut()
                .zip(colors)
            {
                *pixel = HARDWARE_COLORS[(color & 0x1f) as usize];
            }
            self.buffer.copy_within(start..end, start + ROW_LENGTH);
        }

        self.advance();
    }

    #[inline]
    fn advance(&mut self) {
        self.gun_position += PIXELS_PER_WRITE;
        self.width_counter += PIXELS_PER_WRITE;

        if self.width_counter >= VIRTUAL_BUFFER_WIDTH {
            debug_assert_eq!(self.width_counter, VIRTUAL_BUFFER_WIDTH);
            self.gun_position += VIRTUAL_BUFFER_WIDTH;
            self.width_counter = 0;
        }

        if self.gun_position >= FRAME_LENGTH {
            debug_assert_eq!(self.gun_position, FRAME_LENGTH);
            self.gun_position = 0;
            self.waiting_for_vsync = true;
        }
    }

    pub fn trigger_vsync(&mut self, video: &mut impl VideoSink) {
        // The last scan line's second write lands in the spare row; it belongs
        // at the start of the first visible row, which nothing else writes.
        self.buffer
            .copy_within(VISIBLE_LENGTH..VISIBLE_LENGTH + BORDER_WIDTH * 4, 0);

        video.draw_frame(&self.buffer[..VISIBLE_LENGTH]);
        self.waiting_for_vsync = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NullVideo;

    impl VideoSink for NullVideo {
        fn draw_frame(&mut self, _buffer: &[u8]) {}
    }

    #[test]
    fn the_gun_stays_aligned_to_the_gate_arrays_16_pixel_batches() {
        let mut screen = Screen::default();
        screen.trigger_vsync(&mut NullVideo);

        for step in 0..(FRAME_LENGTH / PIXELS_PER_WRITE) {
            assert_eq!(
                screen.gun_position % 16,
                0,
                "gun position is unaligned at step {step}"
            );
            assert_eq!(
                screen.width_counter % 16,
                0,
                "width counter is unaligned at step {step}"
            );

            screen.write(&[0; 16]);
            if screen.waiting_for_vsync {
                screen.trigger_vsync(&mut NullVideo);
            }
        }
    }

    #[test]
    fn only_the_first_border_pixels_of_the_spare_row_are_ever_written() {
        let mut screen = Screen::default();
        screen.trigger_vsync(&mut NullVideo);
        for _ in 0..(FRAME_LENGTH / PIXELS_PER_WRITE) {
            screen.write(&[11; 16]); // white
        }

        assert!(
            screen.buffer[VISIBLE_LENGTH..(VISIBLE_LENGTH + BORDER_WIDTH * 4)]
                .as_chunks::<4>()
                .0
                .iter()
                .all(|pixel| *(pixel) == HARDWARE_COLORS[11]),
            "the wrapped tail of the last scan line is missing"
        );
        assert!(
            screen.buffer[(VISIBLE_LENGTH + BORDER_WIDTH * 4)..]
                .iter()
                .all(|&byte| byte == 0),
            "the spare row is written beyond the wrapped tail"
        );
    }
}

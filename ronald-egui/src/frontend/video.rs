use ronald_core::VideoSink;
use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

pub struct EguiWgpuVideo {
    ctx: CanvasRenderingContext2d,
}

impl EguiWgpuVideo {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self { ctx }
    }
}

impl VideoSink for EguiWgpuVideo {
    fn draw_frame(&mut self, buffer: &Vec<(u8, u8, u8)>) {
        let mut frame = vec![0u8; 4 * SCREEN_BUFFER_WIDTH * SCREEN_BUFFER_HEIGHT];
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel[0] = buffer[i].0; // R
            pixel[1] = buffer[i].1; // G
            pixel[2] = buffer[i].2; // B
            pixel[3] = 255; // A
        }

        let image_data = match ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(frame.as_mut_slice()),
            SCREEN_BUFFER_WIDTH as u32,
            SCREEN_BUFFER_HEIGHT as u32,
        ) {
            Ok(image_data) => image_data,
            Err(_) => return,
        };
        self.ctx.put_image_data(&image_data, 0.0, 0.0);
    }
}

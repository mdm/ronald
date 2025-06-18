use std::sync::Arc;

use pixels::Pixels;
use winit::window::Window;

use ronald_core::VideoSink;

pub struct PixelsVideo<'win> {
    pub pixels: Pixels<'win>,
    pub window: Arc<Window>,
}

impl<'win> VideoSink for &mut PixelsVideo<'win> {
    fn draw_frame(&mut self, buffer: &Vec<(u8, u8, u8)>) {
        for (i, pixel) in self.pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            pixel[0] = buffer[i].0; // R
            pixel[1] = buffer[i].1; // G
            pixel[2] = buffer[i].2; // B
            pixel[3] = 255; // A
        }
    }
}

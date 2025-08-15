use std::sync::Arc;

use pixels::Pixels;
use winit::window::Window;

use ronald_core::VideoSink;

pub struct PixelsVideo<'win> {
    pub pixels: Pixels<'win>,
    pub window: Arc<Window>,
}

impl<'win> VideoSink for &mut PixelsVideo<'win> {
    fn draw_frame(&mut self, buffer: &Vec<u8>) {
        self.pixels.frame_mut().copy_from_slice(buffer);
    }
}

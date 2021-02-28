use crate::{screen, system};

pub struct GUI {
    system: Box<dyn system::System>,
    window: minifb::Window,   
}

impl GUI {
    pub fn new(system: Box<dyn system::System>) -> GUI {
        let mut window = minifb::Window::new(
            "Ronald - Amstrad CPC Emulator",
            screen::BUFFER_WIDTH,
            screen::BUFFER_HEIGHT,
            minifb::WindowOptions::default(),
        ).unwrap(); // TODO: handle errors properly

        window.limit_update_rate(Some(std::time::Duration::from_micros(20_000))); // 50 fps
        // TODO: measure actual fps

        GUI {
            system,
            window,
        }
    }
    
    pub fn run(&mut self) {
        while self.window.is_open() && !self.should_quit() {
            self.system.emulate(None);
            
            self.window.update_with_buffer(
                self.system.get_frame_buffer(),
                screen::BUFFER_WIDTH,
                screen::BUFFER_HEIGHT,
            ).unwrap(); // TODO: handle errors properly
        }
    }

    fn should_quit(&self) -> bool {
        let ctrl_down = self.window.is_key_down(minifb::Key::LeftCtrl) || self.window.is_key_down(minifb::Key::RightCtrl);
        ctrl_down && self.window.is_key_down(minifb::Key::Q)
    }
}
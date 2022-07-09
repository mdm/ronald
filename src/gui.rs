use std::collections::HashMap;

use crate::{keyboard::{self, HostKey, KeyDefinition}, screen, system};

pub struct GUI {
    system: Box<dyn system::System>,
    key_map: HashMap<HostKey, Vec<KeyDefinition>>,
    joystick_enabled: bool,
}

impl GUI {
    pub fn new(system: Box<dyn system::System>) -> GUI {
        let keys = HashMap::from(keyboard::KEYS);
        let mut key_map = HashMap::new();

        GUI { system, key_map, joystick_enabled: false }
    }

    pub fn run(mut self) {
        // self.window
        //     .limit_update_rate(Some(std::time::Duration::from_micros(20_000)));

        // TODO: control frame-rate with pixels
        // see also https://docs.rs/winit/0.25.0/winit/#event-handling
        // double-buffering inside screen on vsync? + waiting after emulation??? -> need to account for draw time and subtract it

        let event_loop = winit::event_loop::EventLoop::new();
        let window = {
            let size = winit::dpi::LogicalSize::new(screen::BUFFER_WIDTH as f64, screen::BUFFER_HEIGHT as f64);
            winit::window::WindowBuilder::new()
                .with_title("Ronald - Amstrad CPC Emulator")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        // window.limit_update_rate(Some(std::time::Duration::from_micros(20_000))); // 50 fps
        //                                                                           // TODO: measure actual fps

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = pixels::SurfaceTexture::new(window_size.width, window_size.height, &window);
            pixels::Pixels::new(screen::BUFFER_WIDTH as u32, screen::BUFFER_HEIGHT as u32, surface_texture).unwrap()
        };


        event_loop.run(move |event, _, control_flow| {
            match event {
                winit::event::Event::RedrawRequested(_) => {
                    log::trace!("Drawing current frame");
                    let start = std::time::Instant::now();

                    self.draw_frame(&mut pixels);
                    pixels.render();

                    log::trace!("Frame drawn in {} microseconds", start.elapsed().as_micros());
                }
                winit::event::Event::RedrawEventsCleared => {
                    log::trace!("Starting new frame");
                    let start = std::time::Instant::now();
        
                    let mut elapsed_microseconds: u32 = 0;
                    while elapsed_microseconds < 20_000 {
                        // TODO: tie this to vsync instead of fixed value
                        elapsed_microseconds += self.system.emulate() as u32;
                    }
        
                    log::trace!("Frame emulated in {} microseconds", start.elapsed().as_micros());
                    
                    window.request_redraw();
                }
                _ => {
                    // TODO: handle input events
                }
            }

            // if self.window.is_key_down(minifb::Key::F12) {
            //     self.system.activate_debugger();
            // }

            // if self.window.is_key_down(minifb::Key::F5) {
            //     if let Ok(Some(pathbuf)) = native_dialog::FileDialog::new()
            //         .add_filter("DSK file", &["dsk"])
            //         .show_open_single_file() {
            //         if let Some(dsk_filename) = pathbuf.as_os_str().to_str() {
            //             self.system.load_disk(0, dsk_filename);
            //         }
            //     }
            // }

            // self.update_keys();
        });
    }

    // fn should_quit(&self) -> bool {
    //     let ctrl_down = self.window.is_key_down(minifb::Key::LeftCtrl);
    //     ctrl_down && self.window.is_key_down(minifb::Key::Q)
    // }

    fn draw_frame(&self, pixels: &mut pixels::Pixels) {
        for (i, pixel) in pixels.get_frame().chunks_exact_mut(4).enumerate() {
            let frame_buffer_color = self.system.get_screen().borrow().get_frame_buffer()[i]; // TODO: optimize frame buffer access
            let rgba = [
                ((frame_buffer_color >> 16) & 0xff) as u8,
                ((frame_buffer_color >> 8) & 0xff) as u8,
                ((frame_buffer_color >> 0) & 0xff) as u8,
                255,
            ];

            pixel.copy_from_slice(&rgba);
        }        
    }

    // fn update_keys(&mut self) {
    //     let keyboard = self.system.get_keyboard();
    //     keyboard.borrow_mut().reset_all();

    //     for key in self.window.get_keys() {
    //         match key {
    //             minifb::Key::Up => keyboard.borrow_mut().set_key(0, 0),
    //             minifb::Key::Right => keyboard.borrow_mut().set_key(0, 1),
    //             minifb::Key::Down => keyboard.borrow_mut().set_key(0, 2),
    //             minifb::Key::NumPad9 => keyboard.borrow_mut().set_key(0, 3),
    //             minifb::Key::NumPad6 => if self.joystick_enabled {
    //                 keyboard.borrow_mut().set_key(9, 3)
    //             } else {
    //                 keyboard.borrow_mut().set_key(0, 4)
    //             },
    //             minifb::Key::NumPad3 => keyboard.borrow_mut().set_key(0, 5),
    //             minifb::Key::NumPadEnter => keyboard.borrow_mut().set_key(0, 6),
    //             minifb::Key::NumPadDot => keyboard.borrow_mut().set_key(0, 7),
    //             minifb::Key::Left => keyboard.borrow_mut().set_key(1, 0),
    //             minifb::Key::Insert => keyboard.borrow_mut().set_key(1, 1),
    //             minifb::Key::NumPad7 => keyboard.borrow_mut().set_key(1, 2),
    //             minifb::Key::NumPad8 => if self.joystick_enabled {
    //                 keyboard.borrow_mut().set_key(9, 0)
    //             } else {
    //                 keyboard.borrow_mut().set_key(1, 3)
    //             },
    //             minifb::Key::NumPad5 => if self.joystick_enabled {
    //                 keyboard.borrow_mut().set_key(9, 4)
    //             } else {
    //                 keyboard.borrow_mut().set_key(1, 4)
    //             },
    //             minifb::Key::NumPad1 => keyboard.borrow_mut().set_key(1, 5),
    //             minifb::Key::NumPad2 => if self.joystick_enabled {
    //                 keyboard.borrow_mut().set_key(9, 1)
    //             } else {
    //                 keyboard.borrow_mut().set_key(1, 6)
    //             },
    //             minifb::Key::NumPad0 => if self.joystick_enabled {
    //                 keyboard.borrow_mut().set_key(9, 5)
    //             } else {
    //                 keyboard.borrow_mut().set_key(1, 7)
    //             },
    //             minifb::Key::Delete => keyboard.borrow_mut().set_key(2, 0),
    //             minifb::Key::LeftBracket => keyboard.borrow_mut().set_key(2, 1),
    //             minifb::Key::Enter => keyboard.borrow_mut().set_key(2, 2),
    //             minifb::Key::RightBracket => keyboard.borrow_mut().set_key(2, 3),
    //             minifb::Key::NumPad4 => if self.joystick_enabled {
    //                 keyboard.borrow_mut().set_key(9, 2)
    //             } else {
    //                 keyboard.borrow_mut().set_key(2, 4)
    //             },
    //             minifb::Key::LeftShift => keyboard.borrow_mut().set_key(2, 5),
    //             minifb::Key::RightShift => keyboard.borrow_mut().set_key(2, 5),
    //             minifb::Key::Backslash => keyboard.borrow_mut().set_key(2, 6),
    //             minifb::Key::RightCtrl => keyboard.borrow_mut().set_key(2, 7),
    //             minifb::Key::Equal => keyboard.borrow_mut().set_key(3, 0),
    //             minifb::Key::Minus => keyboard.borrow_mut().set_key(3, 1),
    //             minifb::Key::End => keyboard.borrow_mut().set_key(3, 2),
    //             minifb::Key::P => keyboard.borrow_mut().set_key(3, 3),
    //             minifb::Key::Semicolon => keyboard.borrow_mut().set_key(3, 4),
    //             minifb::Key::Apostrophe => keyboard.borrow_mut().set_key(3, 5),
    //             minifb::Key::Slash => keyboard.borrow_mut().set_key(3, 6),
    //             minifb::Key::Period => keyboard.borrow_mut().set_key(3, 7),
    //             minifb::Key::Key0 => keyboard.borrow_mut().set_key(4, 0),
    //             minifb::Key::Key9 => keyboard.borrow_mut().set_key(4, 1),
    //             minifb::Key::O => keyboard.borrow_mut().set_key(4, 2),
    //             minifb::Key::I => keyboard.borrow_mut().set_key(4, 3),
    //             minifb::Key::L => keyboard.borrow_mut().set_key(4, 4),
    //             minifb::Key::K => keyboard.borrow_mut().set_key(4, 5),
    //             minifb::Key::M => keyboard.borrow_mut().set_key(4, 6),
    //             minifb::Key::Comma => keyboard.borrow_mut().set_key(4, 7),
    //             minifb::Key::Key8 => keyboard.borrow_mut().set_key(5, 0),
    //             minifb::Key::Key7 => keyboard.borrow_mut().set_key(5, 1),
    //             minifb::Key::U => keyboard.borrow_mut().set_key(5, 2),
    //             minifb::Key::Y => keyboard.borrow_mut().set_key(5, 3),
    //             minifb::Key::H => keyboard.borrow_mut().set_key(5, 4),
    //             minifb::Key::J => keyboard.borrow_mut().set_key(5, 5),
    //             minifb::Key::N => keyboard.borrow_mut().set_key(5, 6),
    //             minifb::Key::Space => keyboard.borrow_mut().set_key(5, 7),
    //             minifb::Key::Key6 => keyboard.borrow_mut().set_key(6, 0),
    //             minifb::Key::Key5 => keyboard.borrow_mut().set_key(6, 1),
    //             minifb::Key::R => keyboard.borrow_mut().set_key(6, 2),
    //             minifb::Key::T => keyboard.borrow_mut().set_key(6, 3),
    //             minifb::Key::G => keyboard.borrow_mut().set_key(6, 4),
    //             minifb::Key::F => keyboard.borrow_mut().set_key(6, 5),
    //             minifb::Key::B => keyboard.borrow_mut().set_key(6, 6),
    //             minifb::Key::V => keyboard.borrow_mut().set_key(6, 7),
    //             minifb::Key::Key4 => keyboard.borrow_mut().set_key(7, 0),
    //             minifb::Key::Key3 => keyboard.borrow_mut().set_key(7, 1),
    //             minifb::Key::E => keyboard.borrow_mut().set_key(7, 2),
    //             minifb::Key::W => keyboard.borrow_mut().set_key(7, 3),
    //             minifb::Key::S => keyboard.borrow_mut().set_key(7, 4),
    //             minifb::Key::D => keyboard.borrow_mut().set_key(7, 5),
    //             minifb::Key::C => keyboard.borrow_mut().set_key(7, 6),
    //             minifb::Key::X => keyboard.borrow_mut().set_key(7, 7),
    //             minifb::Key::Key1 => keyboard.borrow_mut().set_key(8, 0),
    //             minifb::Key::Key2 => keyboard.borrow_mut().set_key(8, 1),
    //             minifb::Key::Escape => keyboard.borrow_mut().set_key(8, 2),
    //             minifb::Key::Q => keyboard.borrow_mut().set_key(8, 3),
    //             minifb::Key::Tab => keyboard.borrow_mut().set_key(8, 4),
    //             minifb::Key::A => keyboard.borrow_mut().set_key(8, 5),
    //             minifb::Key::CapsLock => keyboard.borrow_mut().set_key(8, 6),
    //             minifb::Key::Z => keyboard.borrow_mut().set_key(8, 7),
    //             minifb::Key::Backspace => keyboard.borrow_mut().set_key(9, 7),
    //             minifb::Key::NumLock => {
    //                 self.joystick_enabled = !self.joystick_enabled;
    //             }
    //             _ => {}
    //         }
    //     }
    // }
}

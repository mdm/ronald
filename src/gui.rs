use std::{collections::HashMap, time::Duration};
use std::fs::File;

use crate::{keyboard::{self, HostKey, KeyDefinition}, screen, system};

pub struct GUI {
    system: Box<dyn system::System>,
    key_map: HashMap<HostKey, Vec<KeyDefinition>>,
    current_modifiers: u32,
    alt_gr_modifier: u32,
    pressed_keys: HashMap<u32, HostKey>,
    joystick_enabled: bool,
}

impl GUI {
    pub fn new(system: Box<dyn system::System>) -> GUI {
        let keys = HashMap::from(keyboard::KEYS);
        let file = File::open("keyconfig.yml").unwrap();
        let key_configs: HashMap<String, keyboard::KeyConfig> = serde_yaml::from_reader(file).unwrap();
        let mut key_map = HashMap::new();

        for (key, key_config) in key_configs {
            let (_shiftable, key_definition) = keys.get(&key as &str).unwrap();
            key_map.insert(key_config.normal, vec![key_definition.clone()]);
            if let Some(key_config_shifted) = key_config.shifted {
                key_map.insert(key_config_shifted, vec![key_definition.clone(), KeyDefinition { line: 2, bit: 5 }]);
            }
        }

        GUI { 
            system,
            key_map,
            current_modifiers: winit::event::ModifiersState::empty().bits(),
            alt_gr_modifier: 0,
            pressed_keys: HashMap::new(),
            joystick_enabled: false,
        }
    }

    pub fn run(mut self) {
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

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = pixels::SurfaceTexture::new(window_size.width, window_size.height, &window);
            pixels::Pixels::new(screen::BUFFER_WIDTH as u32, screen::BUFFER_HEIGHT as u32, surface_texture).unwrap()
        };

        let mut frame_start = std::time::Instant::now();

        event_loop.run(move |event, _, control_flow| {
            match event {
                winit::event::Event::RedrawRequested(_) => {
                    log::trace!("Drawing current frame");
                    let start = std::time::Instant::now();

                    self.draw_frame(&mut pixels);
                    pixels.render().unwrap();

                    log::trace!("Frame drawn in {} microseconds", start.elapsed().as_micros());

                    if frame_start.elapsed().as_micros() < 20_000 {
                        let delay = 20_000 - frame_start.elapsed().as_micros() as u64;
                        std::thread::sleep(Duration::from_micros(delay));
                    }
                }
                winit::event::Event::RedrawEventsCleared => {
                    frame_start = std::time::Instant::now();

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
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                        self.current_modifiers = modifiers.bits();
                    }
                    winit::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                        winit::event::ElementState::Pressed => {
                            if let Some(winit::event::VirtualKeyCode::F12) = input.virtual_keycode {
                                self.system.activate_debugger();
                            }

                            if let Some(winit::event::VirtualKeyCode::F5) = input.virtual_keycode {
                                if let Ok(Some(pathbuf)) = native_dialog::FileDialog::new()
                                    .add_filter("DSK file", &["dsk"])
                                    .show_open_single_file() {
                                    if let Some(dsk_filename) = pathbuf.as_os_str().to_str() {
                                        self.system.load_disk(0, dsk_filename);
                                    }
                                }
                            }

                            if input.scancode == 100 { // TODO: make this more portable
                                self.alt_gr_modifier = 1 << 12;
                            }

                            let host_key = HostKey { scancode: input.scancode, modifiers: self.current_modifiers | self.alt_gr_modifier };
                            if let Some(keys) = self.key_map.get(&host_key) {
                                self.pressed_keys.insert(input.scancode, host_key);
                                for key in keys {
                                    self.system.get_keyboard().borrow_mut().set_key(key.line, key.bit)                                    
                                }
                            }
                        }
                        winit::event::ElementState::Released => {
                            if input.scancode == 100 { // TODO: make this more portable
                                self.alt_gr_modifier = 0;
                            }

                            if let Some(host_key) = self.pressed_keys.get(&input.scancode) {
                                if let Some(keys) = self.key_map.get(&host_key) {
                                    for key in keys {
                                        self.system.get_keyboard().borrow_mut().unset_key(key.line, key.bit)                                    
                                    }
                                }
                            }
                        }
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    _ => {}
                }
                _ => {}
            }
        });
    }

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
}

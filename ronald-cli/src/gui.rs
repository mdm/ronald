use std::fs::File;
use std::{collections::HashMap, time::Duration};

use ronald_core::{
    constants::{self, KeyDefinition},
    system, Driver, VideoSink,
};

use crate::keybindings;

mod audio;

pub struct Gui<S>
where
    S: system::System,
{
    driver: Driver<S>,
    key_map: HashMap<keybindings::HostKey, Vec<KeyDefinition>>,
    current_modifiers: u32,
    alt_gr_modifier: u32,
    pressed_keys: HashMap<u32, keybindings::HostKey>,
    joystick_enabled: bool,
}

impl<S> Gui<S>
where
    S: system::System,
{
    pub fn new(driver: Driver<S>) -> Gui<S> {
        let keys = HashMap::from(constants::KEYS);
        let file = File::open("keyconfig.yml").unwrap();
        let key_configs: HashMap<String, keybindings::KeyConfig> =
            serde_yaml::from_reader(file).unwrap();
        let mut key_map = HashMap::new();

        for (key, key_config) in key_configs {
            let (_shiftable, key_definition) = keys.get(&key as &str).unwrap();
            key_map.insert(key_config.normal, vec![key_definition.clone()]);
            if let Some(key_config_shifted) = key_config.shifted {
                key_map.insert(
                    key_config_shifted,
                    vec![key_definition.clone(), KeyDefinition { line: 2, bit: 5 }],
                );
            }
        }

        Gui {
            driver,
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
            let size = winit::dpi::LogicalSize::new(
                constants::SCREEN_BUFFER_WIDTH as f64,
                constants::SCREEN_BUFFER_HEIGHT as f64,
            );
            winit::window::WindowBuilder::new()
                .with_title("Ronald - Amstrad CPC Emulator")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                pixels::SurfaceTexture::new(window_size.width, window_size.height, &window);
            pixels::Pixels::new(
                constants::SCREEN_BUFFER_WIDTH as u32,
                constants::SCREEN_BUFFER_HEIGHT as u32,
                surface_texture,
            )
            .unwrap()
        };

        let mut frame_start = std::time::Instant::now();

        event_loop.run(move |event, _, control_flow| {
            match event {
                winit::event::Event::RedrawRequested(_) => {
                    log::trace!("Drawing current frame");
                    let start = std::time::Instant::now();

                    self.draw_frame(&mut pixels);
                    pixels.render().unwrap();

                    log::trace!(
                        "Frame drawn in {} microseconds",
                        start.elapsed().as_micros()
                    );

                    if frame_start.elapsed().as_micros() < 20_000 {
                        let delay = 20_000 - frame_start.elapsed().as_micros() as u64;
                        std::thread::sleep(Duration::from_micros(delay));
                    }
                }
                winit::event::Event::RedrawEventsCleared => {
                    frame_start = std::time::Instant::now();

                    log::trace!("Starting new frame");
                    let start = std::time::Instant::now();

                    self.driver
                        .step(20_000, &mut self, None::<&mut audio::CpalAudioThread>);

                    log::trace!(
                        "Frame emulated in {} microseconds",
                        start.elapsed().as_micros()
                    );

                    window.request_redraw();
                }
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                        self.current_modifiers = modifiers.bits();
                    }
                    winit::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                        winit::event::ElementState::Pressed => {
                            if let Some(winit::event::VirtualKeyCode::F12) = input.virtual_keycode {
                                self.driver.activate_debugger();
                            }

                            if let Some(winit::event::VirtualKeyCode::F5) = input.virtual_keycode {
                                if let Ok(Some(pathbuf)) = native_dialog::FileDialog::new()
                                    .add_filter("DSK file", &["dsk"])
                                    .show_open_single_file()
                                {
                                    if let Ok(rom) = std::fs::read(pathbuf) {
                                        self.driver.load_rom(0, rom);
                                    }
                                }
                            }

                            if input.scancode == 100 {
                                // TODO: make this more portable
                                self.alt_gr_modifier = 1 << 12;
                            }

                            let host_key = keybindings::HostKey {
                                scancode: input.scancode,
                                modifiers: self.current_modifiers | self.alt_gr_modifier,
                            };
                            if let Some(keys) = self.key_map.get(&host_key) {
                                self.pressed_keys.insert(input.scancode, host_key);
                                for key in keys {
                                    self.driver.press_key(key.line, key.bit)
                                }
                            }
                        }
                        winit::event::ElementState::Released => {
                            if input.scancode == 100 {
                                // TODO: make this more portable
                                self.alt_gr_modifier = 0;
                            }

                            if let Some(host_key) = self.pressed_keys.get(&input.scancode) {
                                if let Some(keys) = self.key_map.get(host_key) {
                                    for key in keys {
                                        self.driver.release_key(key.line, key.bit)
                                    }
                                }
                            }
                        }
                    },
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }

    fn draw_frame(&self, pixels: &mut pixels::Pixels) {
        for (i, pixel) in pixels.get_frame().chunks_exact_mut(4).enumerate() {
            let frame_buffer_color = self.system.get_screen().get_frame_buffer()[i]; // TODO: optimize frame buffer access
            let rgba = [
                ((frame_buffer_color >> 16) & 0xff) as u8,
                ((frame_buffer_color >> 8) & 0xff) as u8,
                (frame_buffer_color & 0xff) as u8,
                255,
            ];

            pixel.copy_from_slice(&rgba);
        }
    }
}

impl<S> VideoSink for Gui<S>
where
    S: system::System,
{
    fn set_pixel(&self, x: usize, y: usize, r: u8, g: u8, b: u8) {}

    fn submit_frame(&self) {}
}

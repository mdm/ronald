use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use pixels::PixelsBuilder;
use ronald_core::constants;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::platform::scancode::PhysicalKeyExtScancode;
use winit::window::WindowAttributes;

use crate::keybindings;

const MODIFIERS: [&str; 6] = [
    "ShiftLeft",
    "ShiftRight",
    "AltLeft",
    "AltRight",
    "ControlLeft",
    "ControlRight",
];

struct WindowState<'win> {
    pixels: pixels::Pixels<'win>,
    window: Arc<winit::window::Window>,
}

impl<'win> WindowState<'win> {
    fn draw_frame(&mut self) {
        let frame = self.pixels.frame_mut();
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0; // R
            pixel[1] = 0; // G
            pixel[2] = 0; // B
            pixel[3] = 255; // A
        }
    }
}

pub struct KeyboardConfigurator {
    window_state: Option<WindowState<'static>>,
    print_key: bool,
    modifiers_recorded: bool,
    current_key: usize,
    current_shifted: bool,
    current_modifiers: winit::keyboard::ModifiersState,
    last_pressed_key: Option<winit::keyboard::PhysicalKey>,
    recorded_key_normal: Option<keybindings::HostKey>,
    recorded_key_shifted: Option<keybindings::HostKey>,
    recorded_modifiers: Vec<winit::keyboard::PhysicalKey>,
    recorded_keys: HashMap<&'static str, keybindings::KeyConfig>,
}

impl ApplicationHandler for KeyboardConfigurator {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let width = 200;
        let height = 200;
        let size = LogicalSize::new(width as f64, height as f64);

        let Ok(window) = event_loop.create_window(
            WindowAttributes::default()
                .with_title("Ronald - Amstrad CPC Emulator")
                .with_inner_size(Size::Logical(size))
                .with_min_inner_size(Size::Logical(size)),
        ) else {
            log::error!("Failed to create window");
            event_loop.exit();
            return;
        };

        let window = Arc::new(window);

        let surface_texture = pixels::SurfaceTexture::new(width, height, window.clone());

        let Ok(pixels) = PixelsBuilder::new(width, height, surface_texture).build() else {
            log::error!("Failed to create Pixels framebuffer");
            event_loop.exit();
            return;
        };

        self.window_state = Some(WindowState { pixels, window });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::RedrawRequested => {
                self.window_state.as_mut().unwrap().draw_frame();
                self.window_state.as_ref().unwrap().pixels.render().unwrap();
            }
            _ => {
                if self.modifiers_recorded && self.current_key == constants::KEYS.len() {
                    let mut file = File::create("keyconfig.yml").unwrap();
                    file.write_all(
                        serde_yaml::to_string(&self.recorded_keys)
                            .unwrap()
                            .as_bytes(),
                    )
                    .unwrap();

                    event_loop.exit();
                } else if self.modifiers_recorded {
                    self.record_keys(event);
                } else {
                    self.record_modifiers(event);
                }
            }
        }
    }
}

impl KeyboardConfigurator {
    pub fn new() -> KeyboardConfigurator {
        KeyboardConfigurator {
            window_state: None,
            print_key: true,
            modifiers_recorded: false,
            current_key: 0,
            current_shifted: false,
            current_modifiers: winit::keyboard::ModifiersState::empty(),
            last_pressed_key: None,
            recorded_key_normal: None,
            recorded_key_shifted: None,
            recorded_modifiers: Vec::new(),
            recorded_keys: HashMap::new(),
        }
    }

    fn record_modifiers(&mut self, event: winit::event::WindowEvent) {
        if self.print_key {
            println!("Press modifier key \"{}\"", MODIFIERS[self.current_key]);
            self.print_key = false;
        }

        if let winit::event::WindowEvent::KeyboardInput { event, .. } = event {
            if let winit::event::ElementState::Pressed = event.state {
                self.recorded_modifiers.push(event.physical_key);

                self.current_key += 1;
                self.print_key = true;

                if self.current_key == MODIFIERS.len() {
                    self.current_key = 0;
                    self.modifiers_recorded = true;
                }
            }
        }
    }

    fn record_keys(&mut self, event: winit::event::WindowEvent) {
        if self.print_key {
            if self.current_shifted {
                println!(
                    "Press key to assign for \"{}\" (shifted)",
                    constants::KEYS[self.current_key].0
                );
            } else {
                println!(
                    "Press key to assign for \"{}\" (normal)",
                    constants::KEYS[self.current_key].0
                );
            }

            self.print_key = false;
        }

        match event {
            winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                self.current_modifiers = modifiers.state();
            }
            winit::event::WindowEvent::KeyboardInput { event, .. } => match event.state {
                winit::event::ElementState::Pressed => {
                    self.last_pressed_key = Some(event.physical_key);
                    if !self.recorded_modifiers.contains(&event.physical_key) {
                        let scancode = event.physical_key.to_scancode().unwrap();
                        if self.current_shifted {
                            self.recorded_key_shifted = Some(keybindings::HostKey {
                                scancode,
                                modifiers: self.current_modifiers.bits(),
                            });
                        } else {
                            self.recorded_key_normal = Some(keybindings::HostKey {
                                scancode,
                                modifiers: self.current_modifiers.bits(),
                            });
                        }

                        if constants::KEYS[self.current_key].1.shiftable && !self.current_shifted {
                            self.current_shifted = true;
                        } else {
                            self.recorded_keys.insert(
                                constants::KEYS[self.current_key].0,
                                keybindings::KeyConfig {
                                    normal: self.recorded_key_normal.take().unwrap(),
                                    shifted: self.recorded_key_shifted.take(),
                                },
                            );

                            self.current_shifted = false;
                            self.current_key += 1;
                        }

                        self.print_key = true;
                    }
                }
                winit::event::ElementState::Released => {
                    if self.recorded_modifiers.contains(&event.physical_key)
                        && self.last_pressed_key == Some(event.physical_key)
                    {
                        self.recorded_key_normal = Some(keybindings::HostKey {
                            scancode: event.physical_key.to_scancode().unwrap(),
                            modifiers: self.current_modifiers.bits(),
                        });

                        self.recorded_keys.insert(
                            constants::KEYS[self.current_key].0,
                            keybindings::KeyConfig {
                                normal: self.recorded_key_normal.take().unwrap(),
                                shifted: None,
                            },
                        );
                        self.recorded_key_shifted = None;

                        self.current_shifted = false;
                        self.current_key += 1;

                        self.print_key = true;
                    }
                }
            },
            _ => {}
        }
    }
}

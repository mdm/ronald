use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use crate::keyboard;

const MODIFIERS: [&str; 6] = [
    "ShiftLeft",
    "ShiftRight",
    "AltLeft",
    "AltRight",
    "ControlLeft",
    "ControlRight",
];

pub struct KeyboardConfigurator {
    print_key: bool,
    modifiers_recorded: bool,
    current_key: usize,
    current_shifted: bool,
    current_modifiers: winit::event::ModifiersState,
    last_pressed_key: winit::event::ScanCode,
    recorded_key_normal: Option<keyboard::ScancodeAndModifiers>,
    recorded_key_shifted: Option<keyboard::ScancodeAndModifiers>,
    recorded_modifiers: Vec<winit::event::ScanCode>,
    recorded_keys: HashMap<&'static str, keyboard::KeyConfig>,
}

impl KeyboardConfigurator {
    pub fn new() -> KeyboardConfigurator {
        KeyboardConfigurator {
            print_key: true,
            modifiers_recorded: false,
            current_key: 0,
            current_shifted: false,
            current_modifiers: winit::event::ModifiersState::empty(),
            last_pressed_key: 0,
            recorded_key_normal: None,
            recorded_key_shifted: None,
            recorded_modifiers: Vec::new(),
            recorded_keys: HashMap::new(),
        }
    }

    pub fn run(mut self) {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = {
            let size = winit::dpi::LogicalSize::new(200, 200);
            winit::window::WindowBuilder::new()
                .with_title("Ronald - Amstrad CPC Emulator")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        event_loop.run(move |event, _, control_flow| {
            if self.modifiers_recorded {
                self.record_keys(event);
            } else {
                self.record_modifiers(event);
            }
            
            if self.modifiers_recorded && self.current_key == keyboard::KEYS.len() {
                let mut file = File::create("keyconfig.yml").unwrap();
                file.write_all(serde_yaml::to_string(&self.recorded_keys).unwrap().as_bytes()).unwrap();

                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
        });
    }

    fn record_modifiers(&mut self, event: winit::event::Event<()>) {
        if self.print_key {
            println!("Press modifier key \"{}\"", MODIFIERS[self.current_key]);
            self.print_key = false;
        }

        if let winit::event::Event::WindowEvent { event, .. } = event {
            match event {
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let winit::event::ElementState::Pressed = input.state {
                        self.recorded_modifiers.push(input.scancode);

                        self.current_key += 1;
                        self.print_key = true;

                        if self.current_key == MODIFIERS.len() {
                            self.current_key = 0;
                            self.modifiers_recorded = true;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn record_keys(&mut self, event: winit::event::Event<()>) {
        if self.print_key {
            if self.current_shifted {
                println!("Press key to assign for \"{}\" (shifted)", keyboard::KEYS[self.current_key].0);
            } else {
                println!("Press key to assign for \"{}\" (normal)", keyboard::KEYS[self.current_key].0);
            }

            self.print_key = false;
        }

        if let winit::event::Event::WindowEvent { event, .. } = event {
            match event {
                winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                    self.current_modifiers = modifiers;
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    match input.state {
                        winit::event::ElementState::Pressed => {
                            self.last_pressed_key = input.scancode;
                            if !self.recorded_modifiers.contains(&input.scancode) {
                                if self.current_shifted {
                                    self.recorded_key_shifted = Some(keyboard::ScancodeAndModifiers { scancode: input.scancode, modifiers: self.current_modifiers.bits() });
                                } else {
                                    self.recorded_key_normal = Some(keyboard::ScancodeAndModifiers { scancode: input.scancode, modifiers: self.current_modifiers.bits() });
                                }

                                if keyboard::KEYS[self.current_key].1 && !self.current_shifted {
                                    self.current_shifted = true;
                                } else {
                                    self.recorded_keys.insert(keyboard::KEYS[self.current_key].0, keyboard::KeyConfig { normal: self.recorded_key_normal.take().unwrap(), shifted: self.recorded_key_shifted.take() });

                                    self.current_shifted = false;
                                    self.current_key += 1;
                                }

                                self.print_key = true;
                            }
                        }
                        winit::event::ElementState::Released => {
                            if self.recorded_modifiers.contains(&input.scancode) && self.last_pressed_key == input.scancode {
                                self.recorded_key_normal = Some(keyboard::ScancodeAndModifiers { scancode: input.scancode, modifiers: self.current_modifiers.bits() });

                                self.recorded_keys.insert(keyboard::KEYS[self.current_key].0, keyboard::KeyConfig { normal: self.recorded_key_normal.take().unwrap(), shifted: None });
                                self.recorded_key_shifted = None;                                    

                                self.current_shifted = false;
                                self.current_key += 1;

                                self.print_key = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

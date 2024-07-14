use std::fs::File;
use std::{collections::HashMap, time::Duration};

use pixels::Pixels;
use winit::event_loop::EventLoop;
use winit::window::Window;

use ronald_core::{
    constants::{self, KeyDefinition},
    system, Driver, VideoSink,
};

use crate::keybindings;

mod audio;

struct PixelsWindow {
    pixels: Pixels,
    window: Window,
}

impl PixelsWindow {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let window = {
            let size = winit::dpi::LogicalSize::new(
                constants::SCREEN_BUFFER_WIDTH as f64,
                constants::SCREEN_BUFFER_HEIGHT as f64,
            );
            winit::window::WindowBuilder::new()
                .with_title("Ronald - Amstrad CPC Emulator")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(event_loop)
                .unwrap()
        };

        let pixels = {
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

        Self { pixels, window }
    }
}

impl VideoSink for PixelsWindow {
    fn draw_frame(&mut self, buffer: &Vec<(u8, u8, u8)>) {
        for (i, pixel) in self.pixels.get_frame().chunks_exact_mut(4).enumerate() {
            pixel[0] = buffer[i].0; // R
            pixel[1] = buffer[i].1; // G
            pixel[2] = buffer[i].2; // B
            pixel[3] = 255; // A
        }

        self.window.request_redraw();
    }
}

pub fn run<S>(mut driver: Driver<S>)
where
    S: system::System<'static> + 'static,
{
    let file = File::open("keyconfig.yml").unwrap();
    let key_configs: HashMap<String, keybindings::KeyConfig> =
        serde_yaml::from_reader(file).unwrap();
    let mut key_map = HashMap::new();

    for (key, key_config) in key_configs {
        key_map.insert(key_config.normal, vec![key.clone()]);
        if let Some(key_config_shifted) = key_config.shifted {
            key_map.insert(key_config_shifted, vec![key.clone(), "Shift".into()]);
        }
    }

    let event_loop = winit::event_loop::EventLoop::new();
    let mut pixels_window = PixelsWindow::new(&event_loop);

    let mut audio = audio::CpalAudio::new();

    let mut current_modifiers = winit::event::ModifiersState::empty().bits();
    let mut alt_gr_modifier = 0;
    let mut pressed_keys = HashMap::new();
    let joystick_enabled = false;

    let mut frame_start = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            winit::event::Event::RedrawRequested(_) => {
                log::trace!("Drawing current frame");
                let start = std::time::Instant::now();

                pixels_window.pixels.render().unwrap();

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

                driver.step(20_000, &mut pixels_window, &mut audio);

                log::trace!(
                    "Frame emulated in {} microseconds",
                    start.elapsed().as_micros()
                );
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                    current_modifiers = modifiers.bits();
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                    winit::event::ElementState::Pressed => {
                        if let Some(winit::event::VirtualKeyCode::F12) = input.virtual_keycode {
                            driver.activate_debugger();
                        }

                        if let Some(winit::event::VirtualKeyCode::F5) = input.virtual_keycode {
                            if let Ok(Some(pathbuf)) = native_dialog::FileDialog::new()
                                .add_filter("DSK file", &["dsk"])
                                .show_open_single_file()
                            {
                                if let Ok(rom) = std::fs::read(pathbuf.clone()) {
                                    driver.load_disk(0, rom, pathbuf);
                                }
                            }
                        }

                        if input.scancode == 100 {
                            // TODO: make this more portable
                            alt_gr_modifier = 1 << 12;
                        }

                        let host_key = keybindings::HostKey {
                            scancode: input.scancode,
                            modifiers: current_modifiers | alt_gr_modifier,
                        };
                        if let Some(keys) = key_map.get(&host_key) {
                            pressed_keys.insert(input.scancode, host_key);
                            for key in keys {
                                driver.press_key(key)
                            }
                        }
                    }
                    winit::event::ElementState::Released => {
                        if input.scancode == 100 {
                            // TODO: make this more portable
                            alt_gr_modifier = 0;
                        }

                        if let Some(host_key) = pressed_keys.get(&input.scancode) {
                            if let Some(keys) = key_map.get(host_key) {
                                for key in keys {
                                    driver.release_key(key)
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

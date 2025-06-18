use std::fs::File;
use std::sync::Arc;
use std::time::Instant;
use std::{collections::HashMap, time::Duration};

use pixels::PixelsBuilder;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, Size};
use winit::platform::scancode::PhysicalKeyExtScancode;
use winit::window::WindowAttributes;

use ronald_core::{constants, system::System, Driver};

use crate::gui::audio::CpalAudio;
use crate::gui::video::PixelsVideo;
use crate::keybindings::{HostKey, KeyConfig};

mod audio;
mod video;

pub struct RonaldApp<'win, S>
where
    S: System<'static> + 'static,
{
    driver: Driver<S>,
    video: Option<PixelsVideo<'win>>,
    audio: CpalAudio,
    key_map: HashMap<HostKey, Vec<String>>,
    current_modifiers: u32,
    alt_gr_modifier: u32,
    pressed_keys: HashMap<u32, HostKey>,
    joystick_enabled: bool,
    frame_start: Instant,
}

impl<'win, S> RonaldApp<'win, S>
where
    S: System<'static> + 'static,
{
    pub fn new(driver: Driver<S>) -> Self {
        let audio = CpalAudio::new();

        let file = File::open("keyconfig.yml").unwrap();
        let key_configs: HashMap<String, KeyConfig> = serde_yaml::from_reader(file).unwrap();
        let mut key_map = HashMap::new();

        for (key, key_config) in key_configs {
            key_map.insert(key_config.normal, vec![key.clone()]);
            if let Some(key_config_shifted) = key_config.shifted {
                key_map.insert(key_config_shifted, vec![key.clone(), "Shift".into()]);
            }
        }

        let current_modifiers = winit::keyboard::ModifiersState::empty().bits();
        let alt_gr_modifier = 0;
        let pressed_keys = HashMap::new();
        let joystick_enabled = false;

        let frame_start = Instant::now();

        Self {
            driver,
            video: None,
            audio,
            key_map,
            current_modifiers,
            alt_gr_modifier,
            pressed_keys,
            joystick_enabled,
            frame_start,
        }
    }
}

impl<'win, S> ApplicationHandler for RonaldApp<'win, S>
where
    S: System<'static> + 'static,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let size = LogicalSize::new(
            constants::SCREEN_BUFFER_WIDTH as f64,
            constants::SCREEN_BUFFER_HEIGHT as f64,
        );

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

        let surface_texture = pixels::SurfaceTexture::new(
            constants::SCREEN_BUFFER_WIDTH as u32,
            constants::SCREEN_BUFFER_HEIGHT as u32,
            window.clone(),
        );

        let Ok(pixels) = PixelsBuilder::new(
            constants::SCREEN_BUFFER_WIDTH as u32,
            constants::SCREEN_BUFFER_HEIGHT as u32,
            surface_texture,
        )
        .build() else {
            log::error!("Failed to create Pixels framebuffer");
            event_loop.exit();
            return;
        };

        self.video = Some(PixelsVideo { pixels, window });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            winit::event::WindowEvent::RedrawRequested => {
                log::trace!("Drawing current frame");
                let start = std::time::Instant::now();

                self.video.as_ref().unwrap().pixels.render().unwrap();

                log::trace!(
                    "Frame drawn in {} microseconds",
                    start.elapsed().as_micros()
                );

                if self.frame_start.elapsed().as_micros() < 20_000 {
                    let delay = 20_000 - self.frame_start.elapsed().as_micros() as u64;
                    std::thread::sleep(Duration::from_micros(delay));
                }

                self.frame_start = std::time::Instant::now();

                log::trace!("Starting new frame");
                let start = std::time::Instant::now();

                self.driver
                    .step(20_000, &mut self.video.as_mut().unwrap(), &mut self.audio);

                log::trace!(
                    "Frame emulated in {} microseconds",
                    start.elapsed().as_micros()
                );

                self.video.as_ref().unwrap().window.request_redraw();
            }
            winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                self.current_modifiers = modifiers.state().bits();
            }
            winit::event::WindowEvent::KeyboardInput { event, .. } => match event.state {
                winit::event::ElementState::Pressed => {
                    if let winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F12) =
                        event.physical_key
                    {
                        self.driver.activate_debugger();
                    }

                    if let winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F5) =
                        event.physical_key
                    {
                        if let Ok(Some(pathbuf)) = native_dialog::DialogBuilder::file()
                            .add_filter("DSK file", &["dsk"])
                            .open_single_file()
                            .show()
                        {
                            if let Ok(rom) = std::fs::read(pathbuf.clone()) {
                                self.driver.load_disk(0, rom, pathbuf);
                            }
                        }
                    }

                    if event.repeat {
                        return;
                    }

                    let scancode = event.physical_key.to_scancode().unwrap();
                    if scancode == 100 {
                        // TODO: make this more portable
                        self.alt_gr_modifier = 1 << 12;
                    }

                    let host_key = HostKey {
                        scancode,
                        modifiers: self.current_modifiers | self.alt_gr_modifier,
                    };
                    if let Some(keys) = self.key_map.get(&host_key) {
                        self.pressed_keys.insert(scancode, host_key);
                        for key in keys {
                            self.driver.press_key(key)
                        }
                    }
                }
                winit::event::ElementState::Released => {
                    let scancode = event.physical_key.to_scancode().unwrap();
                    if scancode == 100 {
                        // TODO: make this more portable
                        self.alt_gr_modifier = 0;
                    }

                    if let Some(host_key) = self.pressed_keys.get(&scancode) {
                        if let Some(keys) = self.key_map.get(host_key) {
                            for key in keys {
                                self.driver.release_key(key)
                            }
                        }
                    }
                }
            },
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

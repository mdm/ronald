use std::{path::PathBuf, time::Instant};

use eframe::{egui, egui_wgpu};
use pollster::FutureExt as _;

use ronald_core::{
    AudioSink, Driver,
    constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH},
    system::System,
};

use crate::frontend::{audio::CpalAudio, video::EguiWgpuVideo};

mod audio;
mod video;

pub struct Frontend<S>
where
    S: System<'static> + 'static,
{
    driver: Driver<S>,
    audio: CpalAudio,
    video: EguiWgpuVideo,
    frame_start: Instant,
    time_available: usize,
    input_test: String,
    can_interact: bool,
}

impl<S> Frontend<S>
where
    S: System<'static> + 'static,
{
    pub fn new(render_state: &egui_wgpu::RenderState) -> Self {
        let driver = Driver::new();
        let audio = CpalAudio::new();
        let video = EguiWgpuVideo::new(render_state);

        Self {
            driver,
            audio,
            video,
            frame_start: Instant::now(),
            time_available: 0,
            input_test: String::new(),
            can_interact: true,
        }
    }

    pub fn load_disk_image_drive_a(&mut self) {
        let saved_frame_start = self.frame_start;
        self.audio.pause_audio();
        if let Some(file) = rfd::AsyncFileDialog::new()
            .set_title("Load DSK into Drive A:")
            .add_filter("DSK Disk Image", &["dsk"])
            .pick_file()
            .block_on()
        {
            self.driver
                .load_disk(0, file.read().block_on(), file.path().to_path_buf());
        }
        self.frame_start = saved_frame_start; // prevent audio delay
        self.audio.play_audio();
    }

    pub fn ui(
        &mut self,
        ctx: &egui::Context,
        ui: Option<&mut egui::Ui>,
        key_mapper: &mut impl KeyMapper,
    ) {
        match ui {
            Some(ui) => {
                let central_panel_size = ui.max_rect().size();
                let size = if central_panel_size.x
                    * (SCREEN_BUFFER_HEIGHT as f32 / SCREEN_BUFFER_WIDTH as f32)
                    < central_panel_size.y
                {
                    egui::Vec2::new(
                        central_panel_size.x,
                        central_panel_size.x
                            * (SCREEN_BUFFER_HEIGHT as f32 / SCREEN_BUFFER_WIDTH as f32),
                    )
                } else {
                    egui::Vec2::new(
                        central_panel_size.y
                            * (SCREEN_BUFFER_WIDTH as f32 / SCREEN_BUFFER_HEIGHT as f32),
                        central_panel_size.y,
                    )
                };
                if let Some(pos) = ctx.pointer_interact_pos() {
                    self.can_interact = ctx.layer_id_at(pos) == Some(egui::LayerId::background());
                }

                if !ctx.wants_keyboard_input() && self.can_interact {
                    ui.input(|input| self.handle_input(input, key_mapper));
                }

                ui.input(|input| {
                    self.handle_dropped_files(input);
                });

                self.step_emulation(); // TODO: step only when accepting input (stop audio?)
                self.draw_framebuffer(ctx, ui, size);
            }
            None => {
                let size = egui::Vec2::new(SCREEN_BUFFER_WIDTH as f32, SCREEN_BUFFER_HEIGHT as f32);

                egui::Window::new("Screen")
                    .collapsible(false)
                    .resizable(false)
                    .default_size(size)
                    .show(ctx, |ui| {
                        if let Some(pos) = ctx.pointer_interact_pos() {
                            self.can_interact = ctx.layer_id_at(pos) == Some(ui.layer_id());
                        }

                        if !ctx.wants_keyboard_input() && self.can_interact {
                            ui.input(|input| self.handle_input(input, key_mapper));
                        }

                        ui.input(|input| {
                            self.handle_dropped_files(input);
                        });

                        self.step_emulation(); // TODO: step only when accepting input (stop audio?)
                        self.draw_framebuffer(ctx, ui, size);
                    });

                egui::Window::new("Input Test").show(ctx, |ui| {
                    ui.text_edit_singleline(&mut self.input_test);
                });
            }
        }
    }

    fn handle_input(&mut self, input: &egui::InputState, key_mapper: &mut impl KeyMapper) {
        key_mapper.map_keys(input, |event| match event {
            KeyEvent::Pressed(key) => {
                self.driver.press_key(key);
            }
            KeyEvent::Released(key) => {
                self.driver.release_key(key);
            }
        });
    }

    fn handle_dropped_files(&mut self, input: &egui::InputState) {
        for dropped_file in input.raw.dropped_files.iter() {
            log::debug!("Dropped file: {dropped_file:?}");
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(path_buf) = dropped_file.path.as_ref() {
                if let Ok(image) = std::fs::read(path_buf) {
                    match path_buf.extension().and_then(|s| s.to_str()) {
                        Some("dsk") => {
                            log::debug!("Loading DSK image into Drive A: {}", path_buf.display());
                            self.driver.load_disk(0, image, path_buf.to_path_buf());
                        }
                        Some("cdt") => {}
                        _ => {}
                    }
                }
            }
            #[cfg(target_arch = "wasm32")]
            if let Some(image) = dropped_file.bytes.as_ref() {
                let path_buf = PathBuf::from(dropped_file.name.clone());
                match path_buf.extension().and_then(|s| s.to_str()) {
                    Some("dsk") => {
                        log::debug!("Loading DSK image into Drive A: {}", path_buf.display());
                        self.driver
                            .load_disk(0, image.to_vec(), path_buf.to_path_buf());
                    }
                    Some("cdt") => {}
                    _ => {}
                }
            }
        }
    }

    fn draw_framebuffer(
        &mut self,
        _ctx: &egui::Context,
        ui: &mut egui::Ui,
        size: egui::Vec2,
    ) -> egui::Response {
        let texture = egui::load::SizedTexture::new(self.video.framebuffer(), size);
        ui.image(texture)
    }

    fn step_emulation(&mut self) {
        log::trace!("Starting new frame");
        let start = Instant::now();

        self.time_available += self.frame_start.elapsed().as_micros() as usize;
        self.frame_start = Instant::now();

        // TODO: Allow running at 60Hz??? Does CPC really support that?
        while self.time_available >= 20_000 {
            log::trace!("Stepping emulator for 20_000 microseconds");
            self.driver.step(20_000, &mut self.video, &mut self.audio);
            self.time_available -= 20_000; // TODO:: take into account actually executed cycles
        }

        if self.time_available > 0 {
            log::trace!("Stepping emulator for {} microseconds", self.time_available);
            self.driver
                .step(self.time_available, &mut self.video, &mut self.audio);
            self.time_available = 0; // TODO:: take into account actually executed cycles
        }

        log::trace!(
            "Frame emulated in {} microseconds",
            start.elapsed().as_micros()
        );
    }
}

pub enum KeyEvent<'k> {
    Pressed(&'k str),
    Released(&'k str),
}

pub trait KeyMapper: Default {
    fn binding(&self, guest_key: &str, shifted: bool) -> Option<&str>;
    fn try_set_binding(
        &mut self,
        guest_key: &str,
        shifted: bool,
        input: &egui::InputState,
    ) -> Result<bool, Box<dyn std::error::Error>>;
    fn clear_binding(
        &mut self,
        guest_key: &str,
        shifted: bool,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn reset_all_bindings(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn map_keys(&mut self, input: &egui::InputState, callback: impl FnMut(KeyEvent));
}

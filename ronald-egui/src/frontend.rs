use std::time::Instant;

use eframe::{egui, egui_wgpu};

use ronald_core::{
    Driver,
    constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH},
    system::System,
};

use crate::frontend::{audio::CpalAudio, video::EguiWgpuVideo};

mod audio;
mod video;

pub struct Frontend<S, K>
where
    S: System<'static> + 'static,
    K: KeyMapper,
{
    driver: Driver<S>,
    audio: CpalAudio,
    video: EguiWgpuVideo,
    frame_start: Instant,
    time_available: usize,
    key_mapper: K,
    input_test: String,
    can_interact: bool,
}

impl<S, K> Frontend<S, K>
where
    S: System<'static> + 'static,
    K: KeyMapper,
{
    pub fn new(render_state: &egui_wgpu::RenderState) -> Self {
        let driver = Driver::new();
        let audio = CpalAudio::new();
        let video = EguiWgpuVideo::new(render_state);
        let key_mapper = K::default();

        Self {
            driver,
            audio,
            video,
            frame_start: Instant::now(),
            time_available: 0,
            key_mapper,
            input_test: String::new(),
            can_interact: true,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, ui: Option<&mut egui::Ui>) {
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
                    ui.input(|input| self.handle_input(input));
                }

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
                        self.can_interact = Some(ui.layer_id()) == ctx.top_layer_id();

                        if !ctx.wants_keyboard_input() && self.can_interact {
                            ui.input(|input| self.handle_input(input));
                        }

                        self.step_emulation(); // TODO: step only when accepting input (stop audio?)
                        self.draw_framebuffer(ctx, ui, size);
                    });

                egui::Window::new("Input Test").show(ctx, |ui| {
                    ui.text_edit_singleline(&mut self.input_test);
                });
            }
        }
    }

    fn handle_input(&mut self, input: &egui::InputState) {
        for event in self.key_mapper.map_keys(input) {
            match event {
                KeyEvent::Pressed(key)  => {self.driver.press_key(key);            }
                KeyEvent::Released(key)  => {self.driver.release_key(key);            }
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

        // TODO: Allow running at 60Hz
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
    fn bind_key(&mut self, key: &str, input: &egui::InputState) -> Result<(), Box<dyn std::error::Error>>;
    fn reset_bindings(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn map_keys(&mut self, input: &egui::InputState) -> impl Iterator<Item = KeyEvent<'_>>;
}

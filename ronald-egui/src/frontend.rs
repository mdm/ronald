use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use eframe::{egui, egui_wgpu};

use ronald_core::{
    Driver,
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
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context, ui: Option<&mut egui::Ui>) {
        match ui {
            Some(ui) => {
                let size = ui.min_rect().size();

                if !ctx.wants_keyboard_input() {
                    ui.input(|input| self.handle_input(input));
                }
                self.step_emulation();
                self.draw_framebuffer(ctx, ui, size);
            }
            None => {
                let size = egui::Vec2::new(SCREEN_BUFFER_WIDTH as f32, SCREEN_BUFFER_HEIGHT as f32);

                egui::Window::new("Screen")
                    .collapsible(false)
                    .resizable(false)
                    .default_size(size)
                    .show(ctx, |ui| {
                        if !ctx.wants_keyboard_input() {
                            ui.input(|input| self.handle_input(input));
                        }
                        self.step_emulation();
                        self.draw_framebuffer(ctx, ui, size);
                    });

                egui::Window::new("Input Test").show(ctx, |ui| {
                    ui.text_edit_singleline(&mut self.input_test);
                });
            }
        }
    }

    fn handle_input(&mut self, input: &egui::InputState) {
        for event in &input.events {
            if let egui::Event::Key {
                key,
                pressed,
                modifiers,
                ..
            } = event
            {
                if *pressed {
                    log::trace!("Key pressed: {:?} with modifiers: {:?}", key, modifiers);
                    if key.name() == "Backspace" {
                        self.driver.press_key("Delete");
                    } else {
                        self.driver.press_key(key.name());
                    }
                } else {
                    log::trace!("Key released: {:?}", key);
                    if key.name() == "Backspace" {
                        self.driver.release_key("Delete");
                    } else {
                        self.driver.release_key(key.name());
                    }
                }
            }
        }
    }

    fn draw_framebuffer(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui, size: egui::Vec2) {
        let texture = egui::load::SizedTexture::new(self.video.framebuffer(), size);
        ui.image(texture);
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

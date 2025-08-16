use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::spawn,
};

use eframe::egui;
use pollster::FutureExt as _;
use web_time::Instant;

use ronald_core::{
    Driver,
    constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH},
    system::System,
};

use crate::frontend::{audio::CpalAudio, video::EguiVideo};
use crate::key_mapper::{KeyMapStore, KeyMapper};

mod audio;
mod video;

#[derive(Debug)]
struct File {
    path_buf: PathBuf,
    image: Vec<u8>,
}

pub struct Frontend<S>
where
    S: System<'static> + 'static,
{
    driver: Driver<S>,
    audio: CpalAudio,
    video: EguiVideo,
    frame_start: Instant,
    time_available: usize,
    input_test: String,
    can_interact: bool,
    picked_file_disk_a: Arc<Mutex<Option<File>>>,
    picked_file_disk_b: Arc<Mutex<Option<File>>>,
    picked_file_tape: Arc<Mutex<Option<File>>>,
    dropped_files: Vec<File>,
}

impl<S> Frontend<S>
where
    S: System<'static> + 'static,
{
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_wgpu(render_state: &eframe::egui_wgpu::RenderState) -> Self {
        let driver = Driver::new();
        let audio = CpalAudio::new();
        let video = EguiVideo::new_wgpu(render_state);

        Self {
            driver,
            audio,
            video,
            frame_start: Instant::now(),
            time_available: 0,
            input_test: String::new(),
            can_interact: true,
            picked_file_disk_a: Arc::new(Mutex::new(None)),
            picked_file_disk_b: Arc::new(Mutex::new(None)),
            picked_file_tape: Arc::new(Mutex::new(None)),
            dropped_files: Vec::new(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_glow(ctx: &egui::Context) -> Self {
        let driver = Driver::new();
        let audio = CpalAudio::new();
        let video = EguiVideo::new_glow(ctx);

        Self {
            driver,
            audio,
            video,
            frame_start: Instant::now(),
            time_available: 0,
            input_test: String::new(),
            can_interact: true,
            picked_file_disk_a: Arc::new(Mutex::new(None)),
            picked_file_disk_b: Arc::new(Mutex::new(None)),
            picked_file_tape: Arc::new(Mutex::new(None)),
            dropped_files: Vec::new(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn pick_file_disk_a(&mut self) {
        let picked_file = Arc::clone(&self.picked_file_disk_a);
        spawn(move || {
            if let Some(file) = rfd::AsyncFileDialog::new()
                .set_title("Load DSK into Drive A:")
                .add_filter("DSK Disk Image", &["dsk"])
                .pick_file()
                .block_on()
                && let Ok(mut picked_file) = picked_file.lock()
            {
                *picked_file = Some(File {
                    path_buf: file.path().to_path_buf(),
                    image: file.read().block_on(),
                });
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    pub fn pick_file_disk_a(&mut self) {
        let picked_file = Arc::clone(&self.picked_file_disk_a);
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(file) = rfd::AsyncFileDialog::new()
                .set_title("Load DSK into Drive A:")
                .add_filter("DSK Disk Image", &["dsk"])
                .pick_file()
                .await
                && let Ok(mut picked_file) = picked_file.lock()
            {
                *picked_file = Some(File {
                    path_buf: file.file_name().into(),
                    image: file.read().block_on(),
                });
            }
        });
    }

    pub fn ui<K>(
        &mut self,
        ctx: &egui::Context,
        ui: Option<&mut egui::Ui>,
        key_mapper: &mut KeyMapper<K>,
    ) where
        K: KeyMapStore,
    {
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

                if !ctx.wants_keyboard_input() {
                    ui.input(|input| self.handle_input(input, key_mapper));
                }

                self.handle_dropped_files(ctx);
                self.handle_picked_files();

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

                        if !ctx.wants_keyboard_input() {
                            ui.input(|input| self.handle_input(input, key_mapper));
                        }

                        self.handle_dropped_files(ctx);
                        self.handle_picked_files();

                        self.step_emulation(); // TODO: step only when accepting input (stop audio?)
                        self.draw_framebuffer(ctx, ui, size);
                    });

                egui::Window::new("Input Test").show(ctx, |ui| {
                    ui.text_edit_singleline(&mut self.input_test);
                });
            }
        }
    }

    fn handle_input<K>(&mut self, input: &egui::InputState, key_mapper: &mut KeyMapper<K>)
    where
        K: KeyMapStore,
    {
        if self.can_interact {
            key_mapper.map_keys(input, |event| match event {
                KeyEvent::Pressed(key) => {
                    self.driver.press_key(key);
                }
                KeyEvent::Released(key) => {
                    self.driver.release_key(key);
                }
            });
        }

        for dropped_file in input.raw.dropped_files.iter() {
            log::debug!("Dropped file: {dropped_file:?}");
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(path_buf) = dropped_file.path.as_ref()
                && let Ok(image) = std::fs::read(path_buf)
            {
                self.dropped_files.push(File {
                    path_buf: path_buf.to_path_buf(),
                    image,
                });
            }
            #[cfg(target_arch = "wasm32")]
            if let Some(image) = dropped_file.bytes.as_ref() {
                let path_buf = PathBuf::from(dropped_file.name.clone());

                self.dropped_files.push(File {
                    path_buf: path_buf.to_path_buf(),
                    image: image.to_vec(),
                });
            }
        }
    }

    fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        if let Some(dropped_file) = self.dropped_files.last() {
            let extension = dropped_file
                .path_buf
                .extension()
                .map(|s| s.to_ascii_lowercase())
                .and_then(|s| s.into_string().ok());
            match extension.as_deref() {
                Some("dsk") => {
                    egui::Modal::new("drive_selection_modal".into()).show(ctx, |ui| {
                        let filename = self
                            .dropped_files
                            .last()
                            .and_then(|f| f.path_buf.file_name())
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                            .unwrap_or("unknown file".into());

                        ui.label(format!("Choose disk drive to load \"{filename}\" into:"));

                        ui.horizontal(|ui| {
                            if ui.button("Drive A").clicked()
                                && let Ok(mut picked_file) = self.picked_file_disk_a.lock()
                            {
                                *picked_file = self.dropped_files.pop();
                            }

                            if ui.button("Drive B").clicked()
                                && let Ok(mut picked_file) = self.picked_file_disk_b.lock()
                            {
                                *picked_file = self.dropped_files.pop();
                            }
                        });
                    });
                }
                Some("cdt") => {
                    if let Ok(mut picked_file) = self.picked_file_tape.try_lock() {
                        *picked_file = self.dropped_files.pop();
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_picked_files(&mut self) {
        if let Ok(mut picked_file) = self.picked_file_disk_a.try_lock()
            && let Some(picked_file) = picked_file.take()
        {
            self.driver
                .load_disk(0, picked_file.image, picked_file.path_buf);
        }

        if let Ok(mut picked_file) = self.picked_file_disk_b.try_lock()
            && let Some(picked_file) = picked_file.take()
        {
            self.driver
                .load_disk(1, picked_file.image, picked_file.path_buf);
        }

        if let Ok(mut picked_file) = self.picked_file_tape.try_lock()
            && let Some(picked_file) = picked_file.take()
        {
            todo!("handle tape loading");
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

use std::{path::PathBuf, thread::spawn};

use eframe::{egui, egui_wgpu};
use serde::{Deserialize, Serialize};
use web_time::Instant;

#[cfg(target_arch = "wasm32")]
use web_sys;

use ronald_core::{
    AudioSink, Driver,
    constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH},
    debug::{
        breakpoint::{AnyBreakpoint, Breakpoint, BreakpointManager},
        view::{DisassembledInstruction, SystemDebugView},
    },
    system::SystemConfig,
};

use crate::frontend::{audio::CpalAudio, video::EguiWgpuVideo};
use crate::key_mapper::{KeyEvent, KeyMapStore, KeyMapper};
use crate::utils::sync::{Shared, SharedExt, shared};

mod audio;
mod video;

#[derive(Debug)]
struct File {
    path_buf: PathBuf,
    image: Vec<u8>,
}

pub struct Frontend {
    initialized: bool,
    driver: Driver,
    audio: CpalAudio,
    video: EguiWgpuVideo,
    frame_start: Instant,
    time_available: usize,
    can_interact: bool,
    paused: bool,
    force_pause: bool,
    show_debug_overlay: bool,
    picked_file_disk_a: Shared<Option<File>>,
    picked_file_disk_b: Shared<Option<File>>,
    picked_file_tape: Shared<Option<File>>,
    dropped_files: Vec<File>,
}

impl Frontend {
    pub fn new(render_state: &egui_wgpu::RenderState) -> Self {
        let driver = Driver::new();
        Self::with_driver_and_render_state(driver, render_state)
    }

    pub fn with_config(render_state: &egui_wgpu::RenderState, config: &SystemConfig) -> Self {
        let driver = Driver::with_config(config);
        Self::with_driver_and_render_state(driver, render_state)
    }

    fn with_driver_and_render_state(driver: Driver, render_state: &egui_wgpu::RenderState) -> Self {
        let audio = CpalAudio::new();
        #[cfg(target_arch = "wasm32")]
        {
            // On WASM we need to ensure audio is played immediately
            audio.play_audio();
        }
        let video = EguiWgpuVideo::new(render_state);

        Self {
            initialized: false,
            driver,
            audio,
            video,
            frame_start: Instant::now(),
            time_available: 0,
            can_interact: true,
            paused: false,
            force_pause: false,
            show_debug_overlay: false,
            picked_file_disk_a: shared(None),
            picked_file_disk_b: shared(None),
            picked_file_tape: shared(None),
            dropped_files: Vec::new(),
        }
    }

    pub fn ui<K>(
        &mut self,
        ctx: &egui::Context,
        ui: Option<&mut egui::Ui>,
        key_mapper: &mut KeyMapper<K>,
        can_interact: bool,
    ) where
        K: KeyMapStore,
    {
        match ui {
            Some(ui) => {
                let size = self.calculate_emulator_display_size(ui);
                self.run_frame(ctx, ui, size, false, can_interact, key_mapper);
            }
            None => {
                let size = egui::Vec2::new(SCREEN_BUFFER_WIDTH as f32, SCREEN_BUFFER_HEIGHT as f32);

                if let Some(window) = egui::Window::new("Screen")
                    .collapsible(false)
                    .resizable(false)
                    .default_size(size)
                    .show(ctx, |ui| {
                        self.run_frame(ctx, ui, size, true, can_interact, key_mapper);
                    })
                    && !self.initialized
                {
                    let layer_id = window.response.layer_id;
                    ctx.move_to_top(layer_id);
                    self.initialized = true;
                }
            }
        }
    }

    pub fn pick_file_disk_a(&mut self) {
        self.pick_file_internal(
            "Load DSK into Drive A:",
            "DSK Disk Image",
            "dsk",
            self.picked_file_disk_a.clone(),
        );
    }

    pub fn pick_file_disk_b(&mut self) {
        self.pick_file_internal(
            "Load DSK into Drive B:",
            "DSK Disk Image",
            "dsk",
            self.picked_file_disk_b.clone(),
        );
    }

    pub fn pick_file_tape(&mut self) {
        self.pick_file_internal(
            "Load Tape:",
            "CDT Tape Image",
            "cdt",
            self.picked_file_tape.clone(),
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pick_file_internal(
        &mut self,
        title: &str,
        filter_name: &str,
        extension: &str,
        picked_file: Shared<Option<File>>,
    ) {
        let title = title.to_string();
        let filter_name = filter_name.to_string();
        let extension = extension.to_string();
        spawn(move || {
            if let Some(file) = rfd::FileDialog::new()
                .set_title(&title)
                .add_filter(&filter_name, &[&extension])
                .pick_file()
            {
                if let Ok(image) = std::fs::read(&file) {
                    picked_file.with_mut(|f| {
                        *f = Some(File {
                            path_buf: file,
                            image,
                        });
                    });
                }
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn pick_file_internal(
        &mut self,
        title: &str,
        filter_name: &str,
        extension: &str,
        picked_file: Shared<Option<File>>,
    ) {
        let title = title.to_string();
        let filter_name = filter_name.to_string();
        let extension = extension.to_string();
        wasm_bindgen_futures::spawn_local(async move {
            if let Some(file) = rfd::AsyncFileDialog::new()
                .set_title(&title)
                .add_filter(&filter_name, &[&extension])
                .pick_file()
                .await
            {
                let file_data = File {
                    path_buf: file.file_name().into(),
                    image: file.read().await,
                };
                picked_file.with_mut(|f| {
                    *f = Some(file_data);
                });
            }
        });
    }

    fn run_frame<K>(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        size: egui::Vec2,
        workbench: bool,
        can_interact: bool,
        key_mapper: &mut KeyMapper<K>,
    ) -> egui::Response
    where
        K: KeyMapStore,
    {
        self.can_interact = if workbench {
            // Workbench mode - check if window is active
            let is_active_window = ctx.top_layer_id() == Some(ui.layer_id());
            is_active_window && can_interact && self.dropped_files.is_empty()
        } else {
            // Emulator only mode
            can_interact && self.dropped_files.is_empty()
        };

        if !ctx.wants_keyboard_input() {
            ui.input(|input| self.handle_input(input, key_mapper));
        }

        self.handle_dropped_files(ctx);
        self.handle_picked_files();

        #[cfg(target_arch = "wasm32")]
        if !self.has_window_focus() || !self.can_interact || self.force_pause {
            self.pause();
        }
        #[cfg(not(target_arch = "wasm32"))]
        if !self.can_interact || self.force_pause {
            self.pause();
        }

        self.step_emulation();
        self.draw_framebuffer(ctx, ui, size)
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
                            if ui.button("Drive A").clicked() {
                                self.picked_file_disk_a.with_mut(|f| {
                                    *f = self.dropped_files.pop();
                                });
                            }

                            if ui.button("Drive B").clicked() {
                                self.picked_file_disk_b.with_mut(|f| {
                                    *f = self.dropped_files.pop();
                                });
                            }
                        });
                    });
                }
                Some("cdt") => {
                    self.picked_file_tape.try_with_mut(|f| {
                        *f = self.dropped_files.pop();
                    });
                }
                _ => {}
            }
        }
    }

    fn handle_picked_files(&mut self) {
        if let Some(picked_file) = self.picked_file_disk_a.try_with_mut(|f| f.take()).flatten() {
            self.driver
                .load_disk(0, picked_file.image, picked_file.path_buf);
        }

        if let Some(picked_file) = self.picked_file_disk_b.try_with_mut(|f| f.take()).flatten() {
            self.driver
                .load_disk(1, picked_file.image, picked_file.path_buf);
        }

        if let Some(picked_file) = self.picked_file_tape.try_with_mut(|f| f.take()).flatten() {
            todo!("handle tape loading");
        }
    }

    fn step_emulation(&mut self) {
        if self.paused {
            return;
        }

        log::trace!("Starting new frame");
        let start = Instant::now();

        self.time_available += self.frame_start.elapsed().as_micros() as usize;
        self.frame_start = Instant::now();

        // TODO: Allow running at 60Hz??? Does CPC really support that?
        while self.time_available >= 20_000 {
            log::trace!("Stepping emulator for 20_000 microseconds");
            let breakpoint_hit = self.driver.step(20_000, &mut self.video, &mut self.audio);
            self.time_available -= 20_000; // TODO:: take into account actually executed cycles

            if breakpoint_hit {
                self.force_pause = true;
                return;
            }
        }

        if self.time_available > 0 {
            log::trace!("Stepping emulator for {} microseconds", self.time_available);
            let breakpoint_hit =
                self.driver
                    .step(self.time_available, &mut self.video, &mut self.audio);
            self.time_available = 0; // TODO:: take into account actually executed cycles

            if breakpoint_hit {
                self.force_pause = true;
                return;
            }
        }

        log::trace!(
            "Frame emulated in {} microseconds",
            start.elapsed().as_micros()
        );
    }

    fn draw_framebuffer(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        size: egui::Vec2,
    ) -> egui::Response {
        let texture = egui::load::SizedTexture::new(self.video.framebuffer(), size);
        let response = ui.image(texture);

        // Update persistent overlay state
        if response.hovered() {
            self.show_debug_overlay = true;
        } else if !response
            .rect
            .contains(ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default())
        {
            // Only hide if pointer is completely outside the image area
            self.show_debug_overlay = false;
        }

        // Show debug overlay when breakpoint is hit or persistent overlay state is true
        let show_overlay = self.paused || self.show_debug_overlay;

        // log::debug!("Show overlay: {show_overlay}");
        if show_overlay {
            self.draw_debug_overlay(ui, &response, size);
        }

        response
    }

    fn pause(&mut self) {
        if !self.paused {
            self.paused = true;
            self.audio.pause_audio();
            log::debug!("Emulation paused");
        }
    }

    fn resume(&mut self) {
        if self.paused {
            self.paused = false;
            self.audio.play_audio();
            self.frame_start = Instant::now(); // Reset timing
            log::debug!("Emulation resumed");
            dbg!(self.can_interact, self.force_pause);
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn has_window_focus(&self) -> bool {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                return document.has_focus().unwrap_or(true);
            }
        }
        true
    }

    fn calculate_emulator_display_size(&self, ui: &mut egui::Ui) -> egui::Vec2 {
        let central_panel_size = ui.max_rect().size();
        if central_panel_size.x * (SCREEN_BUFFER_HEIGHT as f32 / SCREEN_BUFFER_WIDTH as f32)
            < central_panel_size.y
        {
            egui::Vec2::new(
                central_panel_size.x,
                central_panel_size.x * (SCREEN_BUFFER_HEIGHT as f32 / SCREEN_BUFFER_WIDTH as f32),
            )
        } else {
            egui::Vec2::new(
                central_panel_size.y * (SCREEN_BUFFER_WIDTH as f32 / SCREEN_BUFFER_HEIGHT as f32),
                central_panel_size.y,
            )
        }
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn debug_view(&mut self) -> &SystemDebugView {
        self.driver.debug_view()
    }

    pub fn disassemble(&self, start_address: u16, count: usize) -> Vec<DisassembledInstruction> {
        self.driver.disassemble(start_address, count)
    }

    pub fn breakpoint_manager(&mut self) -> &mut BreakpointManager {
        self.driver.breakpoint_manager()
    }

    fn draw_debug_overlay(
        &mut self,
        ui: &mut egui::Ui,
        screen_response: &egui::Response,
        screen_size: egui::Vec2,
    ) {
        let rect = screen_response.rect;
        let overlay_height = 40.0;
        let overlay_rect = egui::Rect::from_min_size(
            egui::Pos2::new(rect.min.x, rect.min.y),
            egui::Vec2::new(screen_size.x, overlay_height),
        );

        // Draw semi-transparent background
        ui.painter().rect_filled(
            overlay_rect,
            egui::CornerRadius::default(),
            egui::Color32::from_black_alpha(128),
        );

        // Draw the overlay content in a child UI
        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(overlay_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );

        child_ui.spacing_mut().item_spacing.x = 8.0;
        child_ui.add_space(8.0);

        // Status text
        let status_text = if self.force_pause || self.paused {
            "Paused"
        } else {
            "Running"
        };

        child_ui.colored_label(egui::Color32::WHITE, status_text);
        child_ui.add_space(16.0);

        // Control buttons
        if self.force_pause || self.paused {
            // Show Run and Step buttons when paused
            if child_ui.button("Run").clicked() {
                self.resume_execution();
            }
            if child_ui.button("Step").clicked() {
                log::debug!("Stepping one instruction");
                self.step_into();
            }
        } else {
            // Show Pause button when running
            if child_ui.button("Pause").clicked() {
                self.pause_execution();
            }
        }
    }

    fn resume_execution(&mut self) {
        self.force_pause = false;
        self.resume();
    }

    fn pause_execution(&mut self) {
        self.force_pause = true;
        self.pause();
    }

    fn step_over(&mut self) {
        //TODO: only step over calls, not jumps
        let current_pc = self.driver.debug_view().cpu.register_pc;

        let disassembly = self.driver.disassemble(current_pc, 1);
        if let Some(instruction) = disassembly.first() {
            let next_pc = current_pc.wrapping_add(instruction.length as u16);

            let mut breakpoint = AnyBreakpoint::pc_breakpoint(next_pc);
            breakpoint.set_one_shot(true);
            self.driver.breakpoint_manager().add_breakpoint(breakpoint);

            self.force_pause = false;
            self.resume();
        }
    }

    fn step_into(&mut self) {
        let mut breakpoint = AnyBreakpoint::pc_step();
        breakpoint.set_one_shot(true);
        self.driver.breakpoint_manager().add_breakpoint(breakpoint);

        self.force_pause = false;
        self.resume();
    }
}

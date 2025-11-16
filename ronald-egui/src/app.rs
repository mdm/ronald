use eframe::egui;
use serde::{Deserialize, Serialize};
use web_time::Instant;

use crate::debug::{CpuDebugWindow, CrtcDebugWindow, GateArrayDebugWindow, MemoryDebugWindow};
use crate::frontend::Frontend;
use crate::key_map_editor::KeyMapEditor;
use crate::key_mapper::KeyMapper;
use crate::system_config::SystemConfigModal;

pub use ronald_core::system::SystemConfig;

pub use crate::key_mapper::KeyMapStore;
pub use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct RonaldApp<S>
where
    S: KeyMapStore,
{
    workbench: bool,
    dark_mode: bool,
    system_config: SystemConfig,
    #[serde(skip)]
    frontend: Option<Frontend>,
    #[serde(skip)]
    key_map_editor: KeyMapEditor,
    #[serde(skip)]
    key_mapper: KeyMapper<S>,
    #[serde(skip)]
    system_config_modal: SystemConfigModal,
    cpu_debug_window: CpuDebugWindow,
    crtc_debug_window: CrtcDebugWindow,
    gate_array_debug_window: GateArrayDebugWindow,
    memory_debug_window: MemoryDebugWindow,
}

impl<S> Default for RonaldApp<S>
where
    S: KeyMapStore,
{
    fn default() -> Self {
        Self {
            workbench: false,
            dark_mode: true,
            system_config: SystemConfig::default(),
            frontend: None,
            key_map_editor: KeyMapEditor::default(),
            key_mapper: KeyMapper::default(),
            system_config_modal: SystemConfigModal::default(),
            cpu_debug_window: CpuDebugWindow::default(),
            crtc_debug_window: CrtcDebugWindow::default(),
            gate_array_debug_window: GateArrayDebugWindow::default(),
            memory_debug_window: MemoryDebugWindow::default(),
        }
    }
}

impl<S> RonaldApp<S>
where
    S: KeyMapStore,
{
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        cc.egui_ctx
            .set_theme(egui::Theme::from_dark_mode(app.dark_mode));

        app
    }
}
impl<S> eframe::App for RonaldApp<S>
where
    S: KeyMapStore,
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let start = Instant::now();
        egui_extras::install_image_loaders(ctx);

        self.render_menu_bar(ctx);
        self.initialize_frontend(ctx, frame);
        self.render_emulator_only_mode(ctx);
        self.render_workbench_mode(ctx);
        self.key_map_editor.ui(ctx, &mut self.key_mapper);
        let config_changed = self.system_config_modal.ui(ctx, &mut self.system_config);
        if config_changed && let Some(render_state) = frame.wgpu_render_state() {
            let new_frontend = Frontend::with_config(render_state, &self.system_config);
            self.frontend = Some(new_frontend);
        }

        if self.workbench
            && let Some(frontend) = &mut self.frontend
        {
            self.cpu_debug_window.ui(ctx, frontend);
            self.crtc_debug_window.ui(ctx, frontend);
            self.gate_array_debug_window.ui(ctx, frontend);
            self.memory_debug_window.ui(ctx, frontend);
        }

        ctx.request_repaint();
        let elapsed = Instant::now() - start;
        log::debug!("Frame time: {} us", elapsed.as_micros());
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl<S> RonaldApp<S>
where
    S: KeyMapStore,
{
    fn render_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("View", |ui| {
                    if ui
                        .add(egui::Button::new("Emulator Only").selected(!self.workbench))
                        .clicked()
                    {
                        self.workbench = false;
                        ui.close_menu();
                    }
                    if ui
                        .add(egui::Button::new("Workbench").selected(self.workbench))
                        .clicked()
                    {
                        self.workbench = true;
                        ui.close_menu();
                    }
                    if self.workbench {
                        ui.separator();
                        if ui
                            .add(egui::Button::new("CPU").selected(self.cpu_debug_window.show))
                            .clicked()
                        {
                            self.cpu_debug_window.show = !self.cpu_debug_window.show;
                            ui.close_menu();
                        }
                        if ui
                            .add(
                                egui::Button::new("Memory").selected(self.memory_debug_window.show),
                            )
                            .clicked()
                        {
                            self.memory_debug_window.show = !self.memory_debug_window.show;
                            ui.close_menu();
                        }
                        if ui
                            .add(egui::Button::new("CRTC").selected(self.crtc_debug_window.show))
                            .clicked()
                        {
                            self.crtc_debug_window.show = !self.crtc_debug_window.show;
                            ui.close_menu();
                        }
                        if ui
                            .add(
                                egui::Button::new("Gate Array")
                                    .selected(self.gate_array_debug_window.show),
                            )
                            .clicked()
                        {
                            self.gate_array_debug_window.show = !self.gate_array_debug_window.show;
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Organize Windows").clicked() {
                            ui.ctx().memory_mut(|mem| mem.reset_areas());
                            ui.close_menu();
                        }
                    }
                });
                ui.menu_button("Media", |ui| {
                    if ui.button("Drive A: Load DSK").clicked() {
                        ui.close_menu();
                        if let Some(frontend) = &mut self.frontend {
                            frontend.pick_file_disk_a();
                        }
                    }
                    if ui.button("Drive B: Load DSK").clicked() {
                        ui.close_menu();
                        if let Some(frontend) = &mut self.frontend {
                            frontend.pick_file_disk_b();
                        }
                    }
                    if ui.button("Tape: Load CDT").clicked() {
                        ui.close_menu();
                        if let Some(frontend) = &mut self.frontend {
                            frontend.pick_file_tape();
                        }
                    }
                });
                ui.menu_button("Settings", |ui| {
                    ui.menu_button("Emulator Theme", |ui| {
                        if ui
                            .add(egui::Button::new("Light").selected(!self.dark_mode))
                            .clicked()
                        {
                            self.dark_mode = false;
                            ui.ctx().set_theme(egui::Theme::Light);
                            ui.close_menu();
                        }
                        if ui
                            .add(egui::Button::new("Dark").selected(self.dark_mode))
                            .clicked()
                        {
                            self.dark_mode = true;
                            ui.ctx().set_theme(egui::Theme::Dark);
                            ui.close_menu();
                        }
                    });
                    ui.separator();
                    if ui.button("System Configuration").clicked() {
                        self.system_config_modal.show = true;
                        ui.close_menu();
                    }
                    if ui.button("Key Bindings").clicked() {
                        self.key_map_editor.show = true;
                        ui.close_menu();
                    }
                });
            });
        });
    }

    fn initialize_frontend(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let (Some(render_state), None) = (&frame.wgpu_render_state, &self.frontend) {
            // On WASM, show a welcome modal to work around the fact that browser audio contexts
            // cannot be started without user interaction.
            #[cfg(target_arch = "wasm32")]
            egui::Modal::new("welcome_modal".into()).show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label(format!("Welcome to Ronald {}", env!("CARGO_PKG_VERSION")));
                    ui.add_space(10.0);
                    ui.label("This emulator recreates the classic Amstrad CPC.");
                    ui.add_space(20.0);
                    if ui.button("Start Emulator").clicked() {
                        let frontend = Frontend::with_config(render_state, &self.system_config);
                        self.frontend = Some(frontend);
                    }
                    ui.add_space(10.0);
                });
            });
            #[cfg(not(target_arch = "wasm32"))]
            {
                let frontend = Frontend::with_config(render_state, &self.system_config);
                self.frontend = Some(frontend);
            }
        }
    }

    fn render_emulator_only_mode(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(frontend) = &mut self.frontend
                && !self.workbench
            {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight)
                        .with_cross_align(egui::Align::TOP),
                    |ui| {
                        frontend.ui(
                            ctx,
                            Some(ui),
                            &mut self.key_mapper,
                            !self.key_map_editor.show && !self.system_config_modal.show,
                        );
                    },
                );
            }
        });
    }

    fn render_workbench_mode(&mut self, ctx: &egui::Context) {
        if let Some(frontend) = &mut self.frontend
            && self.workbench
        {
            frontend.ui(
                ctx,
                None,
                &mut self.key_mapper,
                !self.key_map_editor.show && !self.system_config_modal.show,
            );
        }
    }
}

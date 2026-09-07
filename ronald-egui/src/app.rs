use eframe::egui;
use serde::{Deserialize, Serialize};
use web_time::Instant;

use crate::debug::{
    CpuDebugWindow, CrtcDebugWindow, FdcDebugWindow, GateArrayDebugWindow, MemoryDebugWindow,
};
use crate::frontend::Frontend;
use crate::key_map_editor::KeyMapEditor;
use crate::key_mapper::KeyMapper;
use crate::system_config::{SystemConfig, SystemConfigModal, SystemConfigState, build_core_config};
use crate::utils::sync::{Shared, SharedExt, shared};

pub use crate::key_mapper::KeyMapStore;
#[cfg(not(target_arch = "wasm32"))]
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
    core_config: Shared<SystemConfigState>,
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
    fdc_debug_window: FdcDebugWindow,
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
            core_config: shared(SystemConfigState::Unknown),
            frontend: None,
            key_map_editor: KeyMapEditor::default(),
            key_mapper: KeyMapper::default(),
            system_config_modal: SystemConfigModal::default(),
            cpu_debug_window: CpuDebugWindow::default(),
            crtc_debug_window: CrtcDebugWindow::default(),
            fdc_debug_window: FdcDebugWindow::default(),
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
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let start = Instant::now();
        egui_extras::install_image_loaders(ui.ctx());

        self.render_menu_bar(ui);
        self.initialize_frontend(ui);
        self.render_emulator_only_mode(ui);
        self.render_workbench_mode(ui);
        self.key_map_editor.ui(ui, &mut self.key_mapper);
        let config_changed = self.system_config_modal.ui(ui, &mut self.system_config);
        if config_changed {
            build_core_config(&self.system_config, self.core_config.clone());
        }

        if let Some(SystemConfigState::Valid(core_config)) =
            self.core_config.try_with_mut(|config| config.take())
            && let Some(render_state) = frame.wgpu_render_state()
        {
            self.frontend = Some(Frontend::with_config(render_state, core_config));
        }

        if self.workbench
            && let Some(frontend) = &mut self.frontend
        {
            self.cpu_debug_window.ui(ui, frontend);
            self.crtc_debug_window.ui(ui, frontend);
            self.fdc_debug_window.ui(ui, frontend);
            self.gate_array_debug_window.ui(ui, frontend);
            self.memory_debug_window.ui(ui, frontend);
        }

        ui.ctx().request_repaint();
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
    fn render_menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("menu_bar").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("View", |ui| {
                    if ui
                        .add(egui::Button::new("Emulator Only").selected(!self.workbench))
                        .clicked()
                    {
                        self.workbench = false;
                        ui.close();
                    }
                    if ui
                        .add(egui::Button::new("Workbench").selected(self.workbench))
                        .clicked()
                    {
                        self.workbench = true;
                        ui.close();
                    }
                    if self.workbench {
                        ui.separator();
                        if ui
                            .add(egui::Button::new("CPU").selected(self.cpu_debug_window.show))
                            .clicked()
                        {
                            self.cpu_debug_window.show = !self.cpu_debug_window.show;
                            ui.close();
                        }
                        if ui
                            .add(
                                egui::Button::new("Memory").selected(self.memory_debug_window.show),
                            )
                            .clicked()
                        {
                            self.memory_debug_window.show = !self.memory_debug_window.show;
                            ui.close();
                        }
                        if ui
                            .add(egui::Button::new("CRTC").selected(self.crtc_debug_window.show))
                            .clicked()
                        {
                            self.crtc_debug_window.show = !self.crtc_debug_window.show;
                            ui.close();
                        }
                        if ui
                            .add(egui::Button::new("FDC").selected(self.fdc_debug_window.show))
                            .clicked()
                        {
                            self.fdc_debug_window.show = !self.fdc_debug_window.show;
                            ui.close();
                        }
                        if ui
                            .add(
                                egui::Button::new("Gate Array")
                                    .selected(self.gate_array_debug_window.show),
                            )
                            .clicked()
                        {
                            self.gate_array_debug_window.show = !self.gate_array_debug_window.show;
                            ui.close();
                        }
                        ui.separator();
                        if ui.button("Organize Windows").clicked() {
                            ui.ctx().memory_mut(|mem| mem.reset_areas());
                            ui.close();
                        }
                    }
                });
                ui.menu_button("Media", |ui| {
                    if ui.button("Drive A: Load DSK").clicked() {
                        ui.close();
                        if let Some(frontend) = &mut self.frontend {
                            frontend.pick_file_disk_a();
                        }
                    }
                    if ui.button("Drive B: Load DSK").clicked() {
                        ui.close();
                        if let Some(frontend) = &mut self.frontend {
                            frontend.pick_file_disk_b();
                        }
                    }
                    if ui.button("Tape: Load CDT").clicked() {
                        ui.close();
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
                            ui.close();
                        }
                        if ui
                            .add(egui::Button::new("Dark").selected(self.dark_mode))
                            .clicked()
                        {
                            self.dark_mode = true;
                            ui.ctx().set_theme(egui::Theme::Dark);
                            ui.close();
                        }
                    });
                    ui.separator();
                    if ui.button("System Configuration").clicked() {
                        self.system_config_modal.show = true;
                        ui.close();
                    }
                    if ui.button("Key Bindings").clicked() {
                        self.key_map_editor.show = true;
                        ui.close();
                    }
                });
            });
        });
    }

    #[allow(unused_variables)]
    fn initialize_frontend(&mut self, ui: &mut egui::Ui) {
        if self.frontend.is_some() {
            return;
        }

        if self
            .core_config
            .with_mut(|config| matches!(config, SystemConfigState::Invalid))
        {
            self.system_config_modal.show = true;
            return;
        }

        // On WASM, show a welcome modal to work around the fact that browser audio contexts
        // cannot be started without user interaction.
        #[cfg(target_arch = "wasm32")]
        {
            let system_config = &self.system_config;
            let core_config = &self.core_config;
            egui::Modal::new("welcome_modal".into()).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label(format!("Welcome to Ronald {}", env!("CARGO_PKG_VERSION")));
                    ui.add_space(10.0);
                    ui.label("This emulator recreates the classic Amstrad CPC.");
                    ui.add_space(20.0);
                    if ui.button("Start Emulator").clicked() {
                        build_core_config(system_config, core_config.clone());
                    }
                    ui.add_space(10.0);
                });
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        build_core_config(&self.system_config, self.core_config.clone());
    }

    fn render_emulator_only_mode(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if let Some(frontend) = &mut self.frontend
                && !self.workbench
            {
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight)
                        .with_cross_align(egui::Align::TOP),
                    |ui| {
                        frontend.ui(
                            ui,
                            false,
                            &mut self.key_mapper,
                            !self.key_map_editor.show && !self.system_config_modal.show,
                        );
                    },
                );
            }
        });
    }

    fn render_workbench_mode(&mut self, ui: &mut egui::Ui) {
        if let Some(frontend) = &mut self.frontend
            && self.workbench
        {
            frontend.ui(
                ui,
                true,
                &mut self.key_mapper,
                !self.key_map_editor.show && !self.system_config_modal.show,
            );
        }
    }
}

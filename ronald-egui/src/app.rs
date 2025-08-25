use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::system::CPC464;

use crate::frontend::Frontend;
use crate::key_map_editor::KeyMapEditor;
use crate::key_mapper::KeyMapper;

pub use crate::key_mapper::{KeyMap, KeyMapStore};
pub use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct RonaldApp<S>
where
    S: KeyMapStore,
{
    workbench: bool,
    dark_mode: bool,
    #[serde(skip)]
    frontend: Option<Frontend<CPC464>>,
    #[serde(skip)]
    key_map_editor: KeyMapEditor,
    #[serde(skip)]
    key_mapper: KeyMapper<S>,
}

impl<S> Default for RonaldApp<S>
where
    S: KeyMapStore,
{
    fn default() -> Self {
        Self {
            workbench: false,
            dark_mode: true,
            frontend: None,
            key_map_editor: KeyMapEditor::default(),
            key_mapper: KeyMapper::default(),
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

        // Apply the saved theme preference
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
        egui_extras::install_image_loaders(ctx);

        self.render_menu_bar(ctx);
        self.initialize_frontend(ctx, frame);
        self.render_emulator_only_mode(ctx);
        self.render_workbench_mode(ctx);
        self.key_map_editor.ui(ctx, &mut self.key_mapper);

        ctx.request_repaint();
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
                    ui.menu_button("Theme", |ui| {
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
                        let frontend = Frontend::new(render_state);
                        self.frontend = Some(frontend);
                    }
                    ui.add_space(10.0);
                });
            });
            #[cfg(not(target_arch = "wasm32"))]
            {
                let frontend = Frontend::new(render_state);
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
                            !self.key_map_editor.show,
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
            frontend.ui(ctx, None, &mut self.key_mapper, !self.key_map_editor.show);
        }
    }
}

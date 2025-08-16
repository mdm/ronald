use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::system::CPC464;

use crate::frontend::Frontend;
use crate::key_mapper::KeyMapper;
use crate::key_map_editor::KeyMapEditor;

pub use crate::key_mapper::{KeyMap, KeyMapStore};
pub use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RonaldApp<S>
where
    S: KeyMapStore,
{
    screen_only: bool,
    dark_mode: bool,
    #[serde(skip)]
    frontend: Option<Frontend<CPC464>>,
    #[serde(skip)]
    key_map_editor: KeyMapEditor,
    #[serde(skip)]
    key_mapper: KeyMapper<S>,
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
        cc.egui_ctx.set_theme(egui::Theme::from_dark_mode(app.dark_mode));

        app
    }
}
impl<S> eframe::App for RonaldApp<S>
where
    S: KeyMapStore,
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("View", |ui| {
                    if ui
                        .add(egui::Button::new("Emulator Only").selected(self.screen_only))
                        .clicked()
                    {
                        self.screen_only = true;
                        ui.close_menu();
                    }
                    if ui
                        .add(egui::Button::new("Workbench").selected(!self.screen_only))
                        .clicked()
                    {
                        self.screen_only = false;
                        ui.close_menu();
                    }
                });
                ui.menu_button("Media", |ui| {
                    if ui.button("Drive A: Load DSK").clicked() {
                        ui.close_menu();
                        if let Some(frontend) = &mut self.frontend {
                            frontend.pick_file_disk_a();
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

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(frontend) = &mut self.frontend {
                if self.screen_only {
                    ui.with_layout(
                        egui::Layout::centered_and_justified(egui::Direction::LeftToRight)
                            .with_cross_align(egui::Align::TOP),
                        |ui| {
                            frontend.ui(ctx, Some(ui), &mut self.key_mapper);
                        },
                    );
                }
            }
        });

        if let (Some(render_state), None) = (&frame.wgpu_render_state, &self.frontend) {
            let frontend = Frontend::new(render_state);

            self.frontend = Some(frontend);
        }

        if let Some(frontend) = &mut self.frontend {
            if !self.screen_only {
                frontend.ui(ctx, None, &mut self.key_mapper);
            }
        }

        self.key_map_editor.ui(ctx, &mut self.key_mapper);

        ctx.request_repaint();
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

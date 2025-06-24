use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::system::CPC464;

use frontend::Frontend;
use keyboard::Keyboard;

mod frontend;
mod keyboard;

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RonaldApp {
    screen_only: bool,
    #[serde(skip)]
    frontend: Option<Frontend<CPC464>>,
    #[serde(skip)]
    keyboard: Keyboard,
}

impl RonaldApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
impl eframe::App for RonaldApp {
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
                    }
                    if ui
                        .add(egui::Button::new("Workbench").selected(!self.screen_only))
                        .clicked()
                    {
                        self.screen_only = false;
                    }
                });
                ui.menu_button("Settings", |ui| {
                    if ui.button("Key Bindings").clicked() {
                        self.keyboard.show = true;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(frontend) = &mut self.frontend {
                if self.screen_only {
                    frontend.ui(ctx, Some(ui));
                }
            }
        });

        if let (Some(render_state), None) = (&frame.wgpu_render_state, &self.frontend) {
            let frontend = Frontend::new(render_state);

            self.frontend = Some(frontend);
        }

        if let Some(frontend) = &mut self.frontend {
            if !self.screen_only {
                frontend.ui(ctx, None);
            }
        }

        self.keyboard.ui(ctx);

        ctx.request_repaint();
    }
}

use eframe::egui;
use serde::{Deserialize, Serialize};

use ronald_core::system::CPC464;

use frontend::Frontend;
use keyboard::Keyboard;

pub use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

pub use crate::frontend::{KeyEvent, KeyMapper};

mod frontend;
mod keyboard;

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct RonaldApp<K>
where
    K: KeyMapper,
{
    screen_only: bool,
    #[serde(skip)]
    frontend: Option<Frontend<CPC464>>,
    #[serde(skip)]
    keyboard: Keyboard,
    #[serde(skip)]
    key_mapper: K,
}

impl<K> RonaldApp<K>
where
    K: KeyMapper,
{
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // TODO: figure out how to skip inital window size and store screen_only
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}
impl<K> eframe::App for RonaldApp<K>
where
    K: KeyMapper,
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
                ui.menu_button("Settings", |ui| {
                    if ui.button("Key Bindings").clicked() {
                        self.keyboard.show = true;
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

        self.keyboard.ui(ctx, &mut self.key_mapper);

        ctx.request_repaint();
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        // log::debug!("RawInput modifiers: {:?}", raw_input.modifiers);
        for event in &raw_input.events {
            if let egui::Event::Key {
                physical_key: Some(key),
                ..
            } = event
            {
                log::debug!("Raw key event: {:?}", key,);
            }
        }
    }
}

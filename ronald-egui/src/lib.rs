use std::time::Instant;

use eframe::egui;

// mod frontend;

// TODO: look into serializing parts of this struct to restore ui state
pub struct RonaldUi {
    label: String,
    screen_only: bool,
    hovering_screen: Option<Instant>,
    last_hover_pos: Option<egui::Pos2>,
}

impl Default for RonaldUi {
    fn default() -> Self {
        Self {
            label: "Hello, Ronald!".to_string(),
            screen_only: false,
            hovering_screen: None,
            last_hover_pos: None,
        }
    }
}

impl RonaldUi {
    // pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    pub fn new() -> Self {
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.label);
        });

        let mut screen = egui::Window::new("Screen")
            .collapsible(false)
            .resizable(false)
            .default_size(egui::Vec2::new(640.0, 400.0))
            .show(ctx, |ui| {
                ui.label("This is where the Amstrad CPC screen would be displayed.");
                let window_rect = ui.min_rect();
                let pointer_pos = ctx.pointer_hover_pos();

                if let Some(pos) = pointer_pos {
                    if window_rect.contains(pos) {
                        if self.last_hover_pos != pointer_pos {
                            self.hovering_screen = Some(Instant::now());
                        }
                    } else {
                        self.hovering_screen = None;
                    }
                    self.last_hover_pos = pointer_pos;
                }

                if let Some(hovering) = self.hovering_screen {
                    if hovering.elapsed().as_secs() < 5 {
                        egui::show_tooltip_at(
                            ctx,
                            ui.layer_id(),
                            egui::Id::new("screen_tooltip"),
                            window_rect.min,
                            |ui| {
                                if ui
                                    .button(if self.screen_only {
                                        "Workbench"
                                    } else {
                                        "Screen Only"
                                    })
                                    .clicked()
                                {
                                    self.screen_only = !self.screen_only;
                                }
                            },
                        );
                    } else {
                        self.hovering_screen = None;
                    }
                }
            });

        ctx.request_repaint();
    }
}

// // Setup winit + wgpu yourself
// let event_loop = EventLoop::new();
// let window = WindowBuilder::new().build(&event_loop).unwrap();
//
// // Initialize wgpu
// let instance = wgpu::Instance::default();
// let surface = unsafe { instance.create_surface(&window) }.unwrap();
// let adapter = instance.request_adapter(...).await.unwrap();
// let (device, queue) = adapter.request_device(...).await.unwrap();
//
// // Setup egui context and egui_wgpu renderer
// let egui_ctx = egui::Context::default();
// let mut egui_wgpu_renderer = egui_wgpu::Renderer::new(&device, ...);
//
// // Now you have access to `device` and `queue` freely

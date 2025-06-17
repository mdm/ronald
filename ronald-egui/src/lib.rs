use std::time::Instant;

use eframe::{egui, wgpu};
use serde::{Deserialize, Serialize};

use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

use frontend::video::EguiWgpuVideo;

mod frontend;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct RonaldApp {
    label: String,
    screen_only: bool,
    #[serde(skip)]
    hovering_screen: Option<Instant>,
    #[serde(skip)]
    last_hover_pos: Option<egui::Pos2>,
    #[serde(skip)]
    frontend_video: Option<EguiWgpuVideo>,
    #[serde(skip)]
    framebuffer_texture_id: Option<egui::TextureId>,
}

impl Default for RonaldApp {
    fn default() -> Self {
        Self {
            label: "Hello, Ronald!".to_string(),
            screen_only: false,
            hovering_screen: None,
            last_hover_pos: None,
            frontend_video: None,
            framebuffer_texture_id: None,
        }
    }
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.label);
        });

        if let (Some(render_state), None) = (&frame.wgpu_render_state, &self.frontend_video) {
            // TODO: pass full render state to EguiWgpuVideo and have framebuffer() return a TextureId
            let frontend_video =
                EguiWgpuVideo::new(&render_state.device, render_state.queue.clone());

            let framebuffer_texture_id = render_state.renderer.write().register_native_texture(
                &render_state.device,
                frontend_video.framebuffer(),
                wgpu::FilterMode::Linear,
            );

            self.frontend_video = Some(frontend_video);
            self.framebuffer_texture_id = Some(framebuffer_texture_id);
        }

        let framebuffer_size =
            egui::Vec2::new(SCREEN_BUFFER_WIDTH as f32, SCREEN_BUFFER_HEIGHT as f32);

        let mut screen = egui::Window::new("Screen")
            .collapsible(false)
            .resizable(false)
            .default_size(framebuffer_size)
            .show(ctx, |ui| {
                if let Some(texture_id) = self.framebuffer_texture_id {
                    let texture = egui::load::SizedTexture::new(texture_id, framebuffer_size);
                    ui.image(texture);
                }
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

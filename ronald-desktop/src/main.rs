use std::{error::Error, sync::Arc};

use egui_wgpu::wgpu;

use ronald_egui::RonaldUi;

struct RendererState {
    window: Arc<winit::window::Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    egui_ctx: egui::Context,
    egui_winit: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
}

impl RendererState {
    // TODO: Return Result when egui_wgpu uses a newer version of wgpu
    async fn new(window: Arc<winit::window::Window>) -> Option<Self> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).ok()?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .ok()?;

        let size = window.inner_size();

        let capabilities = surface.get_capabilities(&adapter);
        let surface_format = capabilities.formats[0];

        let egui_ctx = egui::Context::default();
        let egui_winit = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            None, // TODO: Is this what we want?
            None, // TODO: Is this what we want?
            None, // TODO: Is this what we want?
        );
        // TODO: Double check parameters below. Do we want to use SRGB (also above)? See docs & eframe source code.
        let egui_renderer =
            egui_wgpu::Renderer::new(&device, surface_format.add_srgb_suffix(), None, 1, false);

        let state = Self {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            egui_ctx,
            egui_winit,
            egui_renderer,
        };

        state.configure_surface();

        Some(state)
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: self.size.width,
            height: self.size.height,
            present_mode: wgpu::PresentMode::AutoVsync, // TODO: Investigate if this is best for our use case
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
        };

        self.surface.configure(&self.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        self.configure_surface();
    }

    fn render(&mut self) -> Result<(), Box<dyn Error>> {
        let raw_input = self.egui_winit.take_egui_input(&self.window);
        let full_output = self.egui_ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Hello from winit + egui + wgpu!");
            });
        });

        let surface_texture = self.surface.get_current_texture()?;
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &self.egui_ctx,
            &full_output.shapes,
            screen_descriptor,
        );

        let mut encoder = self.device.create_command_encoder(&Default::default());

        Ok(())
    }
}

#[derive(Default)]
struct RonaldApp {
    renderer_state: Option<RendererState>,
}

impl winit::application::ApplicationHandler for RonaldApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        todo!()
    }
}

// fn main() -> eframe::Result {
fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let event_loop = winit::event_loop::EventLoop::new()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = RonaldApp::default();
    event_loop.run_app(&mut app);

    Ok(())
}

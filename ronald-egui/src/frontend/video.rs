use eframe::egui;
use eframe::egui_wgpu;
use eframe::wgpu;

use ronald_core::VideoSink;
use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

pub struct EguiWgpuVideo {
    queue: wgpu::Queue,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    framebuffer_texture_id: egui::TextureId,
}

impl EguiWgpuVideo {
    pub fn new(render_state: &egui_wgpu::RenderState) -> Self {
        let texture_extent = wgpu::Extent3d {
            width: SCREEN_BUFFER_WIDTH as u32,
            height: SCREEN_BUFFER_HEIGHT as u32,
            depth_or_array_layers: 1,
        };

        let texture = render_state
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("Framebuffer Texture"),
                size: texture_extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let framebuffer_texture_id = render_state.renderer.write().register_native_texture(
            &render_state.device,
            &texture_view,
            wgpu::FilterMode::Linear,
        );

        Self {
            queue: render_state.queue.clone(),
            texture,
            texture_view,
            framebuffer_texture_id,
        }
    }

    pub fn framebuffer(&self) -> egui::TextureId {
        self.framebuffer_texture_id
    }
}

impl VideoSink for EguiWgpuVideo {
    fn draw_frame(&mut self, buffer: &Vec<u8>) {
        let texture_extent = wgpu::Extent3d {
            width: SCREEN_BUFFER_WIDTH as u32,
            height: SCREEN_BUFFER_HEIGHT as u32,
            depth_or_array_layers: 1,
        };

        let bytes_per_pixel = 4;
        let bytes_per_row = SCREEN_BUFFER_WIDTH as u32 * bytes_per_pixel;
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            buffer,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(SCREEN_BUFFER_HEIGHT as u32),
            },
            texture_extent,
        );
    }
}

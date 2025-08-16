use eframe::egui;
use ronald_core::VideoSink;
use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

#[cfg(not(target_arch = "wasm32"))]
use eframe::egui_wgpu;
#[cfg(not(target_arch = "wasm32"))]
use eframe::wgpu;

pub struct EguiVideo {
    #[cfg(not(target_arch = "wasm32"))]
    wgpu_data: WgpuVideoData,
    #[cfg(target_arch = "wasm32")]
    framebuffer_texture_handle: egui::TextureHandle,
}

#[cfg(not(target_arch = "wasm32"))]
struct WgpuVideoData {
    queue: wgpu::Queue,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    framebuffer_texture_id: egui::TextureId,
}

impl EguiVideo {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_wgpu(render_state: &egui_wgpu::RenderState) -> Self {
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
            wgpu_data: WgpuVideoData {
                queue: render_state.queue.clone(),
                texture,
                texture_view,
                framebuffer_texture_id,
            },
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new_glow(ctx: &egui::Context) -> Self {
        let color_image = egui::ColorImage::new(
            [SCREEN_BUFFER_WIDTH, SCREEN_BUFFER_HEIGHT],
            egui::Color32::BLACK,
        );

        let framebuffer_texture_handle = ctx.load_texture(
            "framebuffer",
            color_image,
            egui::TextureOptions::LINEAR,
        );

        Self {
            framebuffer_texture_handle,
        }
    }

    pub fn framebuffer(&self) -> egui::TextureId {
        #[cfg(not(target_arch = "wasm32"))]
        return self.wgpu_data.framebuffer_texture_id;
        
        #[cfg(target_arch = "wasm32")]
        return self.framebuffer_texture_handle.id();
    }
}

impl VideoSink for EguiVideo {
    fn draw_frame(&mut self, buffer: &Vec<u8>) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let texture_extent = wgpu::Extent3d {
                width: SCREEN_BUFFER_WIDTH as u32,
                height: SCREEN_BUFFER_HEIGHT as u32,
                depth_or_array_layers: 1,
            };

            let bytes_per_pixel = 4;
            let bytes_per_row = SCREEN_BUFFER_WIDTH as u32 * bytes_per_pixel;
            self.wgpu_data.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &self.wgpu_data.texture,
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

        #[cfg(target_arch = "wasm32")]
        {
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [SCREEN_BUFFER_WIDTH, SCREEN_BUFFER_HEIGHT],
                buffer,
            );
            self.framebuffer_texture_handle.set(
                color_image,
                egui::TextureOptions::LINEAR,
            );
        }
    }
}

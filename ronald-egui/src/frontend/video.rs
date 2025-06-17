use eframe::wgpu;

use ronald_core::VideoSink;
use ronald_core::constants::{SCREEN_BUFFER_HEIGHT, SCREEN_BUFFER_WIDTH};

pub struct EguiWgpuVideo {
    queue: wgpu::Queue,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
}

impl EguiWgpuVideo {
    pub fn new(device: &wgpu::Device, queue: wgpu::Queue) -> Self {
        let texture_extent = wgpu::Extent3d {
            width: SCREEN_BUFFER_WIDTH as u32,
            height: SCREEN_BUFFER_HEIGHT as u32,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Framebuffer Texture"),
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let bytes_per_pixel = 4;
        let bytes_per_row = SCREEN_BUFFER_WIDTH as u32 * bytes_per_pixel;
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &vec![255; bytes_per_pixel as usize * SCREEN_BUFFER_WIDTH * SCREEN_BUFFER_HEIGHT],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(SCREEN_BUFFER_HEIGHT as u32),
            },
            texture_extent,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            queue,
            texture,
            texture_view,
        }
    }

    pub fn framebuffer(&self) -> &wgpu::TextureView {
        &self.texture_view
    }
}

impl VideoSink for EguiWgpuVideo {
    fn draw_frame(&mut self, buffer: &Vec<(u8, u8, u8)>) {
        let mut frame = vec![0u8; 4 * SCREEN_BUFFER_WIDTH * SCREEN_BUFFER_HEIGHT];
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel[0] = buffer[i].0; // R
            pixel[1] = buffer[i].1; // G
            pixel[2] = buffer[i].2; // B
            pixel[3] = 255; // A
        }
    }
}

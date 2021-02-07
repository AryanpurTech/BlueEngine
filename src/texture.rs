use crate::render;
use image::{load_from_memory, GenericImageView};
use wgpu::{self, TextureDataLayout};

impl render::Renderer {
    pub fn new_texture(
        &mut self,
        name: &'static str,
        diffuse_bytes: &'static [u8],
    ) -> Result<(), ()> {
        let diffuse_image =
            load_from_memory(diffuse_bytes).expect("Couldn't Load Image For Texture");
        let diffuse_rgba = diffuse_image
            .as_rgba8()
            .expect("Couldn't Obtain RGBA Data Of The Texture Image");

        let dimentions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimentions.0,
            height: dimentions.1,
            depth: 1,
        };

        let diffuse_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        self.queue.write_texture(
            wgpu::TextureCopyView {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            diffuse_bytes,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimentions.0,
                rows_per_image: dimentions.1,
            },
            texture_size,
        );

        Ok(())
    }
}

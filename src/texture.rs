use crate::definitions::Renderer;
use image::{load_from_memory, GenericImageView};
use wgpu::{self, AddressMode, BindGroupEntry, FilterMode};

impl Renderer {
    pub fn new_texture(
        &mut self,
        name: &'static str,
        diffuse_bytes: Vec<u8>,
        mode: &'static str,
    ) -> Result<usize, ()> {
        let mut MODE: AddressMode;
        if mode == "repeat" {
            MODE = AddressMode::Repeat;
        } else if mode == "mirror_repeat" {
            MODE = AddressMode::MirrorRepeat;
        } else {
            MODE = AddressMode::ClampToEdge;
        };

        let diffuse_image =
            load_from_memory(diffuse_bytes.as_slice()).expect("Couldn't Load Image For Texture");
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
            diffuse_bytes.as_slice(),
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimentions.0,
                rows_per_image: dimentions.1,
            },
            texture_size,
        );

        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: MODE.clone(),
            address_mode_v: MODE.clone(),
            address_mode_w: MODE,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let diffuse_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            label: Some("Diffuse Bind Group"),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
        });

        let address = self.texture_bind_group.len();
        self.texture_bind_group.push(diffuse_bind_group);

        Ok(address)
    }
}

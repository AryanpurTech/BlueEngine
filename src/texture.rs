use crate::definitions::Renderer;
use image::{load_from_memory, GenericImageView};
use wgpu::{self, AddressMode, BindGroupEntry};

impl Renderer {
    pub fn new_texture(
        &mut self,
        name: &'static str,
        diffuse_bytes: Vec<u8>,
        mode: &'static str,
    ) -> Result<usize, ()> {
        let _mode: AddressMode;
        if mode == "repeat" {
            _mode = AddressMode::Repeat;
        } else if mode == "mirror_repeat" {
            _mode = AddressMode::MirrorRepeat;
        } else {
            _mode = AddressMode::ClampToEdge;
        };

        let img =
            load_from_memory(diffuse_bytes.as_slice()).expect("Couldn't Load Image For Texture");
        //let diffuse_rgba = diffuse_image
        //    .as_rgba8()
        //    .expect("Couldn't Obtain RGBA Data Of The Texture Image");

        let rgba = img
            .as_rgba8()
            .expect("Couldn't Obtain RGBA Data Of The Texture Image");
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        self.queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: _mode,
            address_mode_v: _mode,
            address_mode_w: _mode,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let diffuse_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            label: Some("Diffuse Bind Group"),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let address = self.texture_bind_group.len();
        self.texture_bind_group.push(diffuse_bind_group);

        Ok(address)
    }
}

/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::{TextureData, TextureMode, Textures};
use image::GenericImageView;
use wgpu::{Sampler, Texture, TextureView};

impl crate::header::Renderer {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    /// Creates and adds the texture to render queue
    pub fn build_and_append_texture(
        &mut self,
        name: &'static str,
        texture_data: TextureData,
        texture_mode: TextureMode,
        //texture_format: TextureFormat,
    ) -> Result<usize, anyhow::Error> {
        let textures = self
            .build_texture(name, texture_data, texture_mode)
            .expect("Couldn't create shaders");
        let index = self.texture_bind_group.len();
        self.texture_bind_group.push(textures);
        Ok(index)
    }

    /// Creates a new texture data
    pub fn build_texture(
        &mut self,
        name: &'static str,
        texture_data: TextureData,
        texture_mode: TextureMode,
        //texture_format: TextureFormat,
    ) -> Result<Textures, ()> {
        let mode: wgpu::AddressMode;
        match texture_mode {
            TextureMode::Clamp => mode = wgpu::AddressMode::Repeat,
            TextureMode::Repeat => mode = wgpu::AddressMode::MirrorRepeat,
            TextureMode::MirrorRepeat => mode = wgpu::AddressMode::ClampToEdge,
        }

        /*let img_format = match texture_format {
            TextureFormat::PNG => image::ImageFormat::Png,
            TextureFormat::BMP => image::ImageFormat::Bmp,
            TextureFormat::JPEG => image::ImageFormat::Jpeg,
            TextureFormat::PNM => image::ImageFormat::Pnm,
        };*/

        let img = match texture_data {
            TextureData::Bytes(data) => image::load_from_memory(data.as_slice())
                .expect(format!("Couldn't Load Image For Texture Of {}", name).as_str()),
            TextureData::Image(data) => data,
        };

        let rgba = img
            .as_rgba8()
            .expect("Couldn't Obtain RGBA Data Of The Texture Image");
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: mode,
            address_mode_v: mode,
            address_mode_w: mode,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let diffuse_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            label: Some("Diffuse Bind Group"),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Ok(diffuse_bind_group)
    }

    /// Appends a texture to render queue
    pub fn append_texture(&mut self, buffer: Textures) -> Result<usize, anyhow::Error> {
        let index = self.texture_bind_group.len();
        self.texture_bind_group.push(buffer);
        Ok(index)
    }

    /// Deltes texture data
    pub fn remove_texture(&mut self, index: usize) -> Result<(), anyhow::Error> {
        self.texture_bind_group.remove(index);
        Ok(())
    }

    pub(crate) fn build_depth_buffer(
        label: &str,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> (Texture, TextureView, Sampler) {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        return (texture, view, sampler)
    }
}

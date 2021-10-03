use image::GenericImageView;
use wgpu::{BindGroupLayout, util::DeviceExt};
use crate::header::{TextureData, TextureMode, Textures, UniformBuffer, UniformBuffers};

impl crate::header::Renderer {
    /// Creates and adds the uniform buffers to render queue
    pub fn build_and_append_uniform_buffers(
        &mut self,
        uniforms: Vec<UniformBuffer>,
    ) -> Result<(usize, BindGroupLayout), anyhow::Error> {
        let uniform_buffers = self
            .build_uniform_buffer(uniforms)
            .expect("Couldn't create uniform buffer");
        let index = self.shaders.len();
        self.uniform_bind_group.push(uniform_buffers.0);
        Ok((index, uniform_buffers.1))
    }

    /// Creates a new uniform buffer group, according to a list of types
    pub fn build_uniform_buffer(
        &mut self,
        uniforms: Vec<UniformBuffer>,
    ) -> Result<(UniformBuffers, BindGroupLayout), anyhow::Error> {
        let mut buffer_entry = Vec::<wgpu::BindGroupEntry>::new();
        let mut buffer_layout = Vec::<wgpu::BindGroupLayoutEntry>::new();
        let mut buffer_vec = Vec::<wgpu::Buffer>::new();
        for i in uniforms.iter() {
            match i {
                UniformBuffer::Matrix(name, value) => {
                    buffer_vec.push(self.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(*name),
                            contents: bytemuck::cast_slice(&[*value]),
                            usage: wgpu::BufferUsages::UNIFORM,
                        },
                    ));
                }
                UniformBuffer::Array(name, value) => {
                    buffer_vec.push(self.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(*name),
                            contents: bytemuck::cast_slice(&[*value]),
                            usage: wgpu::BufferUsages::UNIFORM,
                        },
                    ));
                }
                UniformBuffer::Float(name, value) => {
                    buffer_vec.push(self.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(*name),
                            contents: bytemuck::cast_slice(&[*value]),
                            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                        },
                    ));
                }
            }
        }
        for i in 0..buffer_vec.len() {
            let descriptor = wgpu::BindGroupEntry {
                binding: i as u32,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer_vec.get(i).unwrap(),
                    offset: 0,
                    size: None,
                }),
            };
            buffer_entry.push(descriptor);
            buffer_layout.push(wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });
        }

        let uniform_bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("uniform dynamic bind group layout"),
                    entries: &buffer_layout.as_slice(),
                });

        let uniform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Groups"),
            layout: &uniform_bind_group_layout,
            entries: &buffer_entry.as_slice(),
        });

        Ok((uniform_bind_group, uniform_bind_group_layout))
    }

    /// Appends a uniform buffer to render queue
    pub fn append_uniform_buffer(
        &mut self,
        buffer: UniformBuffers,
    ) -> Result<usize, anyhow::Error> {
        let index = self.uniform_bind_group.len();
        self.uniform_bind_group.push(buffer);
        Ok(index)
    }

    /// Removes uniform buffer group
    pub fn remove_uniform_buffer(&mut self, index: usize) -> Result<(), anyhow::Error> {
        self.uniform_bind_group.remove(index);
        Ok(())
    }
}

// ============= TEXTURE ============= //

impl crate::header::Renderer {
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
}

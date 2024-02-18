/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use image::GenericImageView;
use wgpu::{util::DeviceExt, BindGroupLayout, Sampler, Texture, TextureView};

use crate::{
    header::{
        Pipeline, PipelineData, ShaderSettings, Shaders, StringBuffer, TextureData, TextureMode,
        Textures, UniformBuffers, Vertex, VertexBuffers,
    },
    InstanceRaw,
};

impl crate::header::Renderer {
    /// Creates a new render pipeline. Could be thought of as like materials in game engines.
    pub fn build_pipeline(
        &mut self,
        shader: Shaders,
        vertex_buffer: VertexBuffers,
        texture: Textures,
        uniform: Option<UniformBuffers>,
    ) -> color_eyre::Result<Pipeline> {
        Ok(Pipeline {
            shader: PipelineData::Data(shader),
            vertex_buffer: PipelineData::Data(vertex_buffer),
            texture: PipelineData::Data(texture),
            uniform: PipelineData::Data(uniform),
        })
    }

    /// Creates a shader group, the input must be spir-v compiled vertex and fragment shader
    pub fn build_shader(
        &mut self,
        name: impl StringBuffer,
        shader_source: String,
        uniform_layout: Option<&BindGroupLayout>,
        settings: ShaderSettings,
    ) -> color_eyre::Result<Shaders> {
        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(format!("{} Shader", name.as_str()).as_str()),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let mut bind_group_layouts = vec![
            &self.texture_bind_group_layout,
            &self.default_uniform_bind_group_layout,
        ];
        if uniform_layout.is_some() {
            bind_group_layouts.push(uniform_layout.unwrap())
        }

        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &bind_group_layouts.as_slice(),
                    push_constant_ranges: &[],
                });

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(name.as_str()),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc(), InstanceRaw::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.config.format,
                        write_mask: wgpu::ColorWrites::ALL,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: settings.topology,
                    strip_index_format: settings.strip_index_format,
                    front_face: settings.front_face,
                    cull_mode: settings.cull_mode, //Some(wgpu::Face::Back),
                    polygon_mode: settings.polygon_mode,
                    conservative: settings.conservative,
                    //clamp_depth: settings.clamp_depth,
                    unclipped_depth: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: crate::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState {
                    count: settings.count,
                    mask: settings.mask,
                    alpha_to_coverage_enabled: settings.alpha_to_coverage_enabled,
                },
                multiview: None,
            });

        Ok(render_pipeline)
    }

    /// Creates a new texture data
    pub fn build_texture(
        &mut self,
        name: impl StringBuffer,
        texture_data: TextureData,
        texture_mode: TextureMode,
        //texture_format: TextureFormat,
    ) -> color_eyre::Result<Textures> {
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
                .expect(format!("Couldn't Load Image For Texture Of {}", name.as_str()).as_str()),
            TextureData::Image(data) => data,
            TextureData::Path(path) => image::open(path)
                .expect(format!("Couldn't Load Image For Texture Of {}", name.as_str()).as_str()),
        };

        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(name.as_str()),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
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

    pub(crate) fn build_depth_buffer(
        label: impl StringBuffer,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> (Texture, TextureView, Sampler) {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label.as_str()),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[wgpu::TextureFormat::Depth32Float],
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
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        return (texture, view, sampler);
    }

    /// Creates a new uniform buffer part
    ///
    /// This function doesn't build the entire uniform buffers list, but rather only one of them
    pub fn build_uniform_buffer_part<T: bytemuck::Zeroable + bytemuck::Pod>(
        &self,
        name: impl StringBuffer,
        value: T,
    ) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(name.as_str()),
                contents: bytemuck::cast_slice(&[value]),
                usage: wgpu::BufferUsages::UNIFORM,
            })
    }

    /// Creates a new uniform buffer group, according to a list of types
    pub fn build_uniform_buffer(
        &mut self,
        uniforms: &Vec<wgpu::Buffer>,
    ) -> color_eyre::Result<(UniformBuffers, BindGroupLayout)> {
        let mut buffer_entry = Vec::<wgpu::BindGroupEntry>::new();
        let mut buffer_layout = Vec::<wgpu::BindGroupLayoutEntry>::new();

        for i in 0..uniforms.len() {
            let descriptor = wgpu::BindGroupEntry {
                binding: i as u32,
                resource: uniforms.get(i).unwrap().as_entire_binding(),
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

    /// Creates a new vertex buffer and indecies
    pub fn build_vertex_buffer(
        &mut self,
        verticies: &Vec<Vertex>,
        indicies: &Vec<u16>,
    ) -> color_eyre::Result<VertexBuffers> {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(verticies.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indicies.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            });

        Ok(VertexBuffers {
            vertex_buffer,
            index_buffer,
            length: indicies.len() as u32,
        })
    }

    /// Creates a new instance buffer for the object
    pub fn build_instance(&self, instance_data: Vec<InstanceRaw>) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            })
    }
}

impl crate::SignalStorage {
    /// Creates a new live event storage
    pub fn new() -> Self {
        Self { events: vec![] }
    }

    /// Adds an event
    pub fn add_signal(&mut self, key: impl StringBuffer, event: Box<dyn crate::Signal>) {
        self.events.push((key.as_string(), event));
    }

    /// Removes an event
    pub fn remove_signal(&mut self, key: impl StringBuffer) {
        self.events.retain(|k| k.0 != key.as_string());
    }

    /// Gets an event
    pub fn get_signal<T: 'static>(
        &mut self,
        key: impl StringBuffer,
    ) -> Option<Result<&mut T, downcast::TypeMismatch>> {
        // fetch the event
        let event = self
            .events
            .iter_mut()
            .find(|k| k.0 == key.as_string())
            .map(|k| &mut k.1);

        if event.is_some() {
            // downcast the event
            let event = event.unwrap();
            let event_type = event.downcast_mut::<T>();
            Some(event_type)
        } else {
            None
        }
    }
}

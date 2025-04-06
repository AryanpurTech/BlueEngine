/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use image::GenericImageView;
use wgpu::{BindGroupLayout, Sampler, Texture, TextureView, util::DeviceExt};

use crate::{
    InstanceRaw, UnsignedIntType,
    prelude::{Shaders, StringBuffer, Textures, UniformBuffers, Vertex},
};

/// Container for pipeline values. Each pipeline takes only 1 vertex shader,
/// 1 fragment shader, 1 texture data, and optionally a vector of uniform data.
#[derive(Debug)]
pub struct Pipeline {
    /// the shader buffer that's sent to the gpu
    pub shader: PipelineData<crate::Shaders>,
    /// The vertex buffer that's sent to the gpu. This includes indices as well
    pub vertex_buffer: PipelineData<VertexBuffers>,
    /// The texture that's sent to the gpu.
    pub texture: PipelineData<crate::Textures>,
    /// the Uniform buffers that are sent to the gpu
    pub uniform: PipelineData<Option<crate::UniformBuffers>>,
}
unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}

/// Container for pipeline data. Allows for sharing resources with other objects
#[derive(Debug)]
pub enum PipelineData<T> {
    /// No data, just a reference to a buffer
    Copy(String),
    /// The actual data
    Data(T),
}

/// Container for vertex and index buffer
#[derive(Debug)]
pub struct VertexBuffers {
    /// An array of vertices. A vertex is a point in 3D space containing
    /// an X, Y, and a Z coordinate between -1 and +1
    pub vertex_buffer: wgpu::Buffer,
    /// An array of indices. Indices are a way to reuse vertices,
    /// this in turn helps greatly in reduction of amount of vertices needed to be sent to the GPU
    pub index_buffer: wgpu::Buffer,
    /// The length of the vertex buffer
    pub length: u32,
}
unsafe impl Send for VertexBuffers {}
unsafe impl Sync for VertexBuffers {}

/// Defines how the texture data is
#[derive(Debug, Clone)]
pub enum TextureData {
    /// the texture file bytes directly
    Bytes(Vec<u8>),
    /// the texture as a [`image::DynamicImage`]
    Image(image::DynamicImage),
    /// path to a texture file to load
    Path(String),
}
unsafe impl Send for TextureData {}
unsafe impl Sync for TextureData {}

/// Defines how the borders of texture would look like
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureMode {
    /// Expands the texture to fit the object
    Clamp,
    /// Repeats the texture instead of stretching
    Repeat,
    /// Repeats the texture, but mirrors it on edges
    MirrorRepeat,
}
unsafe impl Send for TextureMode {}
unsafe impl Sync for TextureMode {}

/// These definitions are taken from wgpu API docs
#[derive(Debug, Clone, Copy)]
pub struct ShaderSettings {
    // ===== PRIMITIVE ===== //
    /// The primitive topology used to interpret vertices
    pub topology: crate::ShaderPrimitive,
    /// When drawing strip topologies with indices, this is the
    /// required format for the index buffer. This has no effect
    /// on non-indexed or non-strip draws.
    pub strip_index_format: Option<crate::IndexFormat>,
    /// The face to consider the front for the purpose of
    /// culling and stencil operations.
    pub front_face: crate::FrontFace,
    /// The face culling mode
    pub cull_mode: Option<crate::CullMode>,
    /// Controls the way each polygon is rasterized. Can be
    /// either `Fill` (default), `Line` or `Point`
    ///
    /// Setting this to something other than `Fill` requires
    /// `NON_FILL_POLYGON_MODE` feature to be enabled
    pub polygon_mode: crate::PolygonMode,
    /// If set to true, the polygon depth is clamped to 0-1
    /// range instead of being clipped.
    ///
    /// Enabling this requires the `DEPTH_CLAMPING` feature
    /// to be enabled
    pub clamp_depth: bool,
    /// If set to true, the primitives are rendered with
    /// conservative overestimation. I.e. any rastered
    /// pixel touched by it is filled. Only valid for PolygonMode::Fill!
    ///
    /// Enabling this requires `CONSERVATIVE_RASTERIZATION`
    /// features to be enabled.
    pub conservative: bool,
    // ===== Multisample ===== //
    /// The number of samples calculated per pixel (for MSAA).
    /// For non-multisampled textures, this should be `1`
    pub count: u32,
    /// Bitmask that restricts the samples of a pixel modified
    /// by this pipeline. All samples can be enabled using the
    /// value `!0`
    pub mask: u64,
    /// When enabled, produces another sample mask per pixel
    /// based on the alpha output value, that is ANDead with the
    /// sample_mask and the primitive coverage to restrict the
    /// set of samples affected by a primitive.

    /// The implicit mask produced for alpha of zero is guaranteed
    /// to be zero, and for alpha of one is guaranteed to be all
    /// 1-s.
    pub alpha_to_coverage_enabled: bool,
}
impl Default for ShaderSettings {
    fn default() -> Self {
        Self {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            clamp_depth: false,
            conservative: false,
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: true,
        }
    }
}
unsafe impl Send for ShaderSettings {}
unsafe impl Sync for ShaderSettings {}

/// This function helps in converting pixel value to the value that is between -1 and +1
pub fn pixel_to_cartesian(value: f32, max: u32) -> f32 {
    let mut result = value / max as f32;

    if value == max as f32 {
        result = 0.0;
    } else if result < max as f32 / 2.0 {
    }

    if result > -1.0 { result } else { -1.0 }
}

impl crate::prelude::Renderer {
    /// Creates a new render pipeline. Could be thought of as like materials in game engines.
    pub fn build_pipeline(
        &mut self,
        shader: Shaders,
        vertex_buffer: VertexBuffers,
        texture: Textures,
        uniform: Option<UniformBuffers>,
    ) -> Pipeline {
        Pipeline {
            shader: PipelineData::Data(shader),
            vertex_buffer: PipelineData::Data(vertex_buffer),
            texture: PipelineData::Data(texture),
            uniform: PipelineData::Data(uniform),
        }
    }

    /// Creates a shader group, the input must be spir-v compiled vertex and fragment shader
    pub fn build_shader(
        &mut self,
        name: impl StringBuffer,
        shader_source: String,
        uniform_layout: Option<&BindGroupLayout>,
        settings: ShaderSettings,
    ) -> Shaders {
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
        if let Some(uniform_layout) = uniform_layout {
            bind_group_layouts.push(uniform_layout);
        }

        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: bind_group_layouts.as_slice(),
                    push_constant_ranges: &[],
                });

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(name.as_str()),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Vertex::desc(), InstanceRaw::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.config.format,
                        write_mask: wgpu::ColorWrites::ALL,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
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
                cache: None,
            });

        render_pipeline
    }

    /// Creates a new texture data
    pub fn build_texture(
        &mut self,
        name: impl StringBuffer,
        texture_data: TextureData,
        texture_mode: TextureMode,
    ) -> Result<Textures, crate::error::Error> {
        let mode: wgpu::AddressMode = match texture_mode {
            TextureMode::Clamp => wgpu::AddressMode::Repeat,
            TextureMode::Repeat => wgpu::AddressMode::MirrorRepeat,
            TextureMode::MirrorRepeat => wgpu::AddressMode::ClampToEdge,
        };

        let img = match texture_data {
            TextureData::Bytes(data) => image::load_from_memory(data.as_slice())?,
            TextureData::Image(data) => data,
            TextureData::Path(path) => image::open(path)?,
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
            format: self.config.format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
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

        (texture, view, sampler)
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
        uniforms: &[wgpu::Buffer],
    ) -> (UniformBuffers, BindGroupLayout) {
        let mut buffer_entry = Vec::<wgpu::BindGroupEntry>::new();
        let mut buffer_layout = Vec::<wgpu::BindGroupLayoutEntry>::new();

        for i in 0..uniforms.len() {
            if let Some(uniform) = uniforms.get(i) {
                let descriptor = wgpu::BindGroupEntry {
                    binding: i as u32,
                    resource: uniform.as_entire_binding(),
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
        }

        let uniform_bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("uniform dynamic bind group layout"),
                    entries: buffer_layout.as_slice(),
                });

        let uniform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Groups"),
            layout: &uniform_bind_group_layout,
            entries: buffer_entry.as_slice(),
        });

        (uniform_bind_group, uniform_bind_group_layout)
    }

    /// Creates a new vertex buffer and indices
    pub fn build_vertex_buffer(
        &mut self,
        vertices: &Vec<Vertex>,
        indices: &Vec<UnsignedIntType>,
    ) -> VertexBuffers {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            });

        VertexBuffers {
            vertex_buffer,
            index_buffer,
            length: indices.len() as u32,
        }
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

        if let Some(event) = event {
            // downcast the event
            let event_type = event.downcast_mut::<T>();
            Some(event_type)
        } else {
            None
        }
    }
}

impl Default for crate::SignalStorage {
    fn default() -> Self {
        Self::new()
    }
}

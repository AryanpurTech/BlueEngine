use crate::definitions::{
    Pipeline, Shaders, Textures, UniformBuffer, UniformBuffers, Vertex, VertexBuffers,
};
use image::GenericImageView;
use wgpu::util::DeviceExt;

impl crate::definitions::Renderer {
    /// Creates and adds the pipeline to render queue
    pub fn build_and_append_pipeline(
        &mut self,
        shader_index: usize,
        buffer_index: usize,
        texture_index: Option<usize>,
        uniform_buffer: Option<usize>,
    ) -> Result<usize, anyhow::Error> {
        let pipe = self
            .build_pipeline(shader_index, buffer_index, texture_index, uniform_buffer)
            .expect("Couldn't Create Render Pipeline");
        self.render_pipelines.push(pipe);
        Ok(self.render_pipelines.len() - 1)
    }

    /// Creates a new render pipeline. Could be thought of as like materials in game engines.
    pub fn build_pipeline(
        &mut self,
        shader_index: usize,
        vertex_buffer_index: usize,
        texture_index: Option<usize>,
        uniform_index: Option<usize>,
    ) -> Result<Pipeline, anyhow::Error> {
        Ok(Pipeline {
            shader_index,
            vertex_buffer_index,
            texture_index,
            uniform_index,
        })
    }

    /// Appends a pipeline to render queue
    pub fn append_pipeline(&mut self, pipeline: Pipeline) -> Result<usize, anyhow::Error> {
        self.render_pipelines.push(pipeline);
        Ok(self.render_pipelines.len() - 1)
    }

    /// Allows to modify a pipeline
    pub fn get_pipeline(&mut self, index: usize) -> Result<&mut Pipeline, anyhow::Error> {
        Ok(self.render_pipelines.get_mut(index).unwrap())
    }

    /// Deletes a render pipeline
    pub fn remove_pipeline(&mut self, index: usize) -> Result<(), anyhow::Error> {
        self.render_pipelines.remove(index);
        Ok(())
    }

    /// Creates and adds the shaders to render queue
    pub fn build_and_append_shaders(
        &mut self,
        name: &'static str,
        vertex_shader: Vec<u8>,
        fragment_shader: Vec<u8>,
    ) -> Result<usize, anyhow::Error> {
        let shaders = self
            .build_shaders(name, vertex_shader, fragment_shader)
            .expect("Couldn't create shaders");
        let index = self.shaders.len();
        self.shaders.push(shaders);
        Ok(index)
    }

    /// Creates a shader group, the input must be spir-v compiled vertex and fragment shader
    pub fn build_shaders(
        &mut self,
        name: &'static str,
        vertex_shader: Vec<u8>,
        fragment_shader: Vec<u8>,
    ) -> Result<Shaders, anyhow::Error> {
        let vs_module = self
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Vertex Shader Source"),
                source: wgpu::util::make_spirv(vertex_shader.as_slice()),
                flags: wgpu::ShaderFlags::VALIDATION,
            });
        let fs_module = self
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Fragment Shader Source"),
                source: wgpu::util::make_spirv(fragment_shader.as_slice()),
                flags: wgpu::ShaderFlags::VALIDATION,
            });
        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        &self.texture_bind_group_layout,
                        &self.uniform_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(name),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vs_module,
                    entry_point: "main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &fs_module,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb,
                        alpha_blend: wgpu::BlendState::REPLACE,
                        color_blend: wgpu::BlendState::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    polygon_mode: wgpu::PolygonMode::Fill,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            });

        Ok(render_pipeline)
    }

    /// Appends a shader to render queue
    pub fn append_shaders(&mut self, shader: Shaders) -> Result<usize, anyhow::Error> {
        let index = self.shaders.len();
        self.shaders.push(shader);
        Ok(index)
    }

    /// Allows to modify a shader
    pub fn get_shader(&mut self, index: usize) -> Result<&mut Shaders, anyhow::Error> {
        Ok(self.shaders.get_mut(index).unwrap())
    }

    /// Deletes a shader group
    pub fn remove_sahder(&mut self, index: usize) -> Result<(), anyhow::Error> {
        self.shaders.remove(index);
        Ok(())
    }

    /// Creates and adds the vertex buffers to render queue
    pub fn build_and_append_vertex_buffers(
        &mut self,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
    ) -> Result<usize, anyhow::Error> {
        let vertex_buffers = self
            .build_vertex_buffers(verticies, indicies)
            .expect("Couldn't create vertex buffer");
        let index = self.vertex_buffers.len();
        self.vertex_buffers.push(vertex_buffers);
        Ok(index)
    }

    /// Creates a new vertex buffer and indecies
    pub fn build_vertex_buffers(
        &mut self,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
    ) -> Result<VertexBuffers, anyhow::Error> {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(verticies.as_slice()),
                usage: wgpu::BufferUsage::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indicies.as_slice()),
                usage: wgpu::BufferUsage::INDEX,
            });

        Ok(VertexBuffers {
            vertex_buffer,
            index_buffer,
            length: indicies.len() as u32,
        })
    }

    /// Appends a vertex buffer to render queue
    pub fn append_vertex_buffer(
        &mut self,
        vertex_buffer: VertexBuffers,
    ) -> Result<usize, anyhow::Error> {
        let index = self.vertex_buffers.len();
        self.vertex_buffers.push(vertex_buffer);
        Ok(index)
    }

    /// Removes vertex and index buffer group
    pub fn remove_vertex_buffer(&mut self, index: usize) -> Result<(), anyhow::Error> {
        self.vertex_buffers.remove(index);
        Ok(())
    }

    /// Creates and adds the uniform buffers to render queue
    pub fn build_and_append_uniform_buffers(
        &mut self,
        uniforms: Vec<UniformBuffer>,
    ) -> Result<usize, anyhow::Error> {
        let uniform_buffers = self
            .build_uniform_buffer(uniforms)
            .expect("Couldn't create uniform buffer");
        let index = self.shaders.len();
        self.uniform_bind_group.push(uniform_buffers);
        Ok(index)
    }

    /// Creates a new uniform buffer group, according to a list of types
    pub fn build_uniform_buffer(
        &mut self,
        uniforms: Vec<UniformBuffer>,
    ) -> Result<UniformBuffers, anyhow::Error> {
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
                            usage: wgpu::BufferUsage::UNIFORM,
                        },
                    ));
                }
                UniformBuffer::Array(name, value) => {
                    buffer_vec.push(self.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(*name),
                            contents: bytemuck::cast_slice(&[*value]),
                            usage: wgpu::BufferUsage::UNIFORM,
                        },
                    ));
                }
                UniformBuffer::Float(name, value) => {
                    buffer_vec.push(self.device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(*name),
                            contents: bytemuck::cast_slice(&[*value]),
                            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                        },
                    ));
                }
            }
        }
        for i in 0..buffer_vec.len() {
            let descriptor = wgpu::BindGroupEntry {
                binding: i as u32,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &buffer_vec.get(i).unwrap(),
                    offset: 0,
                    size: None,
                },
            };
            buffer_entry.push(descriptor);
            buffer_layout.push(wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
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
        self.uniform_bind_group_layout = uniform_bind_group_layout;

        let uniform_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Groups"),
            layout: &self.uniform_bind_group_layout,
            entries: &buffer_entry.as_slice(),
        });

        Ok(uniform_bind_group)
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

    /// Creates and adds the texture to render queue
    pub fn build_and_append_texture(
        &mut self,
        name: &'static str,
        diffuse_bytes: Vec<u8>,
        mode: &'static str,
    ) -> Result<usize, anyhow::Error> {
        let textures = self
            .build_texture(name, diffuse_bytes, mode)
            .expect("Couldn't create shaders");
        let index = self.texture_bind_group.len();
        self.texture_bind_group.push(textures);
        Ok(index)
    }

    /// Creates a new texture data
    pub fn build_texture(
        &mut self,
        name: &'static str,
        diffuse_bytes: Vec<u8>,
        mode: &'static str,
    ) -> Result<Textures, ()> {
        let _mode: wgpu::AddressMode;
        if mode == "repeat" {
            _mode = wgpu::AddressMode::Repeat;
        } else if mode == "mirror_repeat" {
            _mode = wgpu::AddressMode::MirrorRepeat;
        } else {
            _mode = wgpu::AddressMode::ClampToEdge;
        };

        let img = image::load_from_memory(diffuse_bytes.as_slice())
            .expect("Couldn't Load Image For Texture");

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

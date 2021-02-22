use crate::definitions::{Buffers, Pipeline, UniformBuffer, Vertex};
use anyhow::*;
use std::ops::Range;
use wgpu::util::DeviceExt;

impl crate::definitions::Renderer {
    pub fn new_pipeline(
        &mut self,
        shader_index: usize,
        buffer_index: usize,
        texture_index: Option<usize>,
        uniform_buffer: Option<usize>,
    ) {
        self.render_pipeline.push(Pipeline {
            shader_index,
            buffer_index,
            texture_index,
            uniform_buffer,
        });
    }

    pub fn remove_pipeline(&mut self, index: usize) -> Result<()> {
        self.render_pipeline.remove(index);
        Ok(())
    }

    pub fn new_shaders(
        &mut self,
        name: &'static str,
        vertex_shader: Vec<u8>,
        fragment_shader: Vec<u8>,
    ) -> Result<usize> {
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

        println!("creations success");

        let index = self.shaders.len();
        self.shaders.push(render_pipeline);
        Ok(index)
    }

    pub fn remove_sahder(&mut self, index: usize) -> Result<()> {
        self.shaders.remove(index);
        Ok(())
    }

    pub fn new_buffers(
        &mut self,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
        instances: Range<u32>,
    ) -> Result<usize> {
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

        self.buffers.push(Buffers {
            vertex_buffer,
            index_buffer,
            length: indicies.len() as u32,
            instances,
        });

        Ok(self.buffers.len() - 1)
    }

    pub fn remove_buffers(&mut self, index: usize) -> Result<()> {
        self.buffers.remove(index);
        Ok(())
    }

    pub fn new_uniform_buffer(&mut self, uniforms: Vec<UniformBuffer>) -> Result<usize> {
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
                            usage: wgpu::BufferUsage::UNIFORM,
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
        self.uniform_bind_group.push(uniform_bind_group);

        Ok(self.uniform_bind_group.len() - 1)
    }

    pub fn remove_buffer_entry(&mut self, index: usize) -> Result<()> {
        self.uniform_bind_group.remove(index);
        Ok(())
    }
}

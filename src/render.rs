use crate::definitions::{Buffers, Pipeline, Renderer, UniformBindGroup, UniformBuffer, Vertex};
use anyhow::*;
use std::ops::Range;
use wgpu::{self, util::DeviceExt};
use winit::{
    event::{KeyboardInput, VirtualKeyCode, WindowEvent},
    window::Window,
};

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let uniform_dynamic_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform dynamic bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_static_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform static bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &uniform_static_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,

            texture_bind_group_layout,
            uniform_dynamic_bind_group_layout,
            uniform_static_bind_group_layout,
            render_pipeline_layout,

            shaders: Vec::new(),
            texture_bind_group: Vec::new(),
            uniform_bind_group: Vec::new(),
            render_pipeline: Vec::new(),
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => {
                if state == &winit::event::ElementState::Released {}
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        let mut already_loaded_shader: usize = 5;
        let mut already_loaded_texture: usize = 5;
        let mut already_loaded_uniform_buffer: usize = 5;

        for i in self.render_pipeline.iter() {
            if already_loaded_shader != i.shader_index.clone() || i.shader_index.clone() == 0 {
                render_pass.set_pipeline(
                    self.shaders.get(i.shader_index.clone()).expect(
                        format!("Couldn't find shader at index: {}", i.shader_index).as_str(),
                    ),
                );
                already_loaded_shader = i.shader_index;
            }

            if i.texture_index.is_some() {
                let texture_index = i.texture_index.unwrap();
                if already_loaded_texture != texture_index.clone() || texture_index.clone() == 0 {
                    render_pass.set_bind_group(
                        0,
                        self.texture_bind_group.get(texture_index.clone()).unwrap(),
                        &[],
                    );
                    already_loaded_texture = texture_index;
                }
            }

            if i.uniform_buffer.is_some() {
                let uniform_buffer_index = i.uniform_buffer.unwrap();
                let uniform_buffer_enum = self.uniform_bind_group.get(uniform_buffer_index);

                if uniform_buffer_enum.is_some() {
                    if let UniformBindGroup::Matrix(uniform_buffer) = uniform_buffer_enum.unwrap() {
                        if already_loaded_uniform_buffer != uniform_buffer_index.clone()
                            || uniform_buffer_index.clone() == 0
                        {
                            render_pass.set_bind_group(1, uniform_buffer, &[]);
                            already_loaded_uniform_buffer = uniform_buffer_index;
                        }
                    } else if let UniformBindGroup::Array(uniform_buffer) =
                        uniform_buffer_enum.unwrap()
                    {
                        if already_loaded_uniform_buffer != uniform_buffer_index.clone()
                            || uniform_buffer_index.clone() == 0
                        {
                            render_pass.set_bind_group(1, uniform_buffer, &[]);
                            already_loaded_uniform_buffer = uniform_buffer_index;
                        }
                    }
                }
            }

            render_pass.set_vertex_buffer(0, i.buffers.vertex_buffer.slice(..));
            render_pass
                .set_index_buffer(i.buffers.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..i.buffers.length, 0, i.buffers.instances.clone());
        }

        drop(render_pass);

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    pub fn new_pipeline(
        &mut self,
        shader_index: usize,
        buffers: Buffers,
        texture_index: Option<usize>,
        uniform_buffer: Option<usize>,
    ) {
        self.render_pipeline.push(Pipeline {
            shader_index,
            buffers,
            texture_index,
            uniform_buffer,
        });
    }

    pub fn new_shaders(
        &mut self,
        name: &'static str,
        vertex_shader: Vec<u8>,
        fragment_shader: Vec<u8>,
        is_static: bool,
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

        let dynamic_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &self.texture_bind_group_layout,
                    &self.uniform_dynamic_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(name),
                layout: Some(if is_static {
                    &self.render_pipeline_layout
                } else {
                    &dynamic_layout
                }),
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

    pub fn new_buffers(
        &mut self,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
        instances: Range<u32>,
    ) -> Buffers {
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

        return Buffers {
            vertex_buffer,
            index_buffer,
            length: indicies.len() as u32,
            instances,
        };
    }

    pub fn new_uniform_buffer(
        &mut self,
        name: &str,
        uniform: UniformBuffer,
        is_static: bool,
    ) -> Result<usize> {
        if let UniformBuffer::Matrix(value) = uniform {
            let uniform_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(name),
                        contents: bytemuck::cast_slice(&[value]),
                        usage: wgpu::BufferUsage::UNIFORM,
                    });

            if is_static {
                let uniform_bind_group =
                    self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some(format!("{} bind group", name).as_str()),
                        layout: &self.uniform_static_bind_group_layout,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &uniform_buffer,
                                offset: 0,
                                size: None,
                            },
                        }],
                    });

                self.uniform_bind_group
                    .push(UniformBindGroup::Array(uniform_bind_group));
            } else {
                let uniform_bind_group =
                    self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some(format!("{} bind group", name).as_str()),
                        layout: &self.uniform_dynamic_bind_group_layout,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &uniform_buffer,
                                offset: 0,
                                size: None,
                            },
                        }],
                    });

                self.uniform_bind_group
                    .push(UniformBindGroup::Array(uniform_bind_group));
            }
        } else if let UniformBuffer::Array(value) = uniform {
            let uniform_buffer =
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(name),
                        contents: bytemuck::cast_slice(&[value]),
                        usage: wgpu::BufferUsage::UNIFORM,
                    });

            if is_static {
                let uniform_bind_group =
                    self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some(format!("{} bind group", name).as_str()),
                        layout: &self.uniform_static_bind_group_layout,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &uniform_buffer,
                                offset: 0,
                                size: None,
                            },
                        }],
                    });

                self.uniform_bind_group
                    .push(UniformBindGroup::Array(uniform_bind_group));
            } else {
                let uniform_bind_group =
                    self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some(format!("{} bind group", name).as_str()),
                        layout: &self.uniform_dynamic_bind_group_layout,
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Buffer {
                                buffer: &uniform_buffer,
                                offset: 0,
                                size: None,
                            },
                        }],
                    });

                self.uniform_bind_group
                    .push(UniformBindGroup::Array(uniform_bind_group));
            }
        }
        Ok(self.uniform_bind_group.len() - 1)
    }
}

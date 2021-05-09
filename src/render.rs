use crate::definitions::Renderer;
use anyhow::Result;
use wgpu::Features;
use winit::window::Window;

impl Renderer {
    pub(crate) async fn new(window: &Window) -> Self {
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
                    features: Features::NON_FILL_POLYGON_MODE,
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

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform dynamic bind group layout"),
                entries: &[],
            });

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,

            texture_bind_group_layout,
            uniform_bind_group_layout,

            shaders: Vec::new(),
            vertex_buffers: Vec::new(),
            texture_bind_group: Vec::new(),
            uniform_bind_group: Vec::new(),
            render_pipelines: Vec::new(),
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
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
        let mut already_loaded_buffer: usize = 5;
        let mut already_loaded_texture: usize = 5;
        let mut already_loaded_uniform_buffer: usize = 5;

        for i in self.render_pipelines.iter() {
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

            if i.uniform_index.is_some() {
                let uniform_buffer_index = i.uniform_index.clone().unwrap();
                let uniform_buffer_enum_option = self.uniform_bind_group.get(uniform_buffer_index);

                if uniform_buffer_enum_option.is_some() {
                    let uniform_buffer = uniform_buffer_enum_option.expect(
                        format!(
                            "Uniform buffer group at {} doesn't exist",
                            uniform_buffer_index
                        )
                        .as_str(),
                    );
                    if already_loaded_uniform_buffer != uniform_buffer_index.clone()
                        || uniform_buffer_index.clone() == 0
                    {
                        render_pass.set_bind_group(1, uniform_buffer, &[]);
                        already_loaded_uniform_buffer = uniform_buffer_index;
                    }
                }
            }

            if already_loaded_buffer != i.vertex_buffer_index.clone()
                || i.vertex_buffer_index.clone() == 0
            {
                let buffers = self.vertex_buffers.get(i.vertex_buffer_index).unwrap();
                render_pass.set_vertex_buffer(0, buffers.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(buffers.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..buffers.length, 0, 0..1);
                already_loaded_buffer = i.vertex_buffer_index;
            }
        }

        drop(render_pass);

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}

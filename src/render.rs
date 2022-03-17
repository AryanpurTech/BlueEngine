/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::Renderer;
use anyhow::Result;
use wgpu::Features;
use winit::window::Window;

#[cfg(not(target_feature = "NON_FILL_POLYGON_MODE"))]
fn get_render_features() -> Features {
    Features::empty()
}
#[cfg(target_feature = "NON_FILL_POLYGON_MODE")]
fn get_render_features() -> Features {
    Features::NON_FILL_POLYGON_MODE
}

impl Renderer {
    pub(crate) async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surface),
                //force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    features: get_render_features(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let default_uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform dynamic bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let depth_buffer = Renderer::build_depth_buffer("Depth Buffer", &device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,

            texture_bind_group_layout,
            default_uniform_bind_group_layout,
            depth_buffer,

            shaders: Vec::new(),
            vertex_buffers: Vec::new(),
            texture_bind_group: Vec::new(),
            uniform_bind_group: Vec::new(),
            render_pipelines: Vec::new(),
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        self.depth_buffer = Self::build_depth_buffer("Depth Buffer", &self.device, &self.config);
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let frame = self.surface.get_current_frame()?.output;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_buffer.1,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        let mut already_loaded_shader: usize = 0;
        let mut already_loaded_buffer: usize = 5;
        let mut already_loaded_texture: usize = 0;
        let mut already_loaded_uniform_buffer: usize = 0;

        render_pass.set_bind_group(
            1,
            self.uniform_bind_group
                .get(0)
                .expect("Couldn't find the camera uniform data"),
            &[],
        );
        render_pass.set_pipeline(
            self.shaders
                .get(0)
                .expect("Couldn't find the default shader"),
        );
        render_pass.set_bind_group(
            0,
            self.texture_bind_group
                .get(0)
                .expect("Couldn't find the default texture"),
            &[],
        );

        for i in self.render_pipelines.iter() {
            if already_loaded_shader != i.shader_index.clone() || i.shader_index.clone() >= 1 {
                render_pass.set_pipeline(
                    self.shaders.get(i.shader_index.clone()).expect(
                        format!("Couldn't find shader at index: {}", i.shader_index).as_str(),
                    ),
                );
                already_loaded_shader = i.shader_index;
            }

            if already_loaded_texture != i.texture_index.clone() || i.texture_index.clone() >= 1 {
                render_pass.set_bind_group(
                    0,
                    self.texture_bind_group
                        .get(i.texture_index.clone())
                        .unwrap(),
                    &[],
                );
                already_loaded_texture = i.texture_index;
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
                        || uniform_buffer_index.clone() == 1
                    {
                        render_pass.set_bind_group(2, uniform_buffer, &[]);
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

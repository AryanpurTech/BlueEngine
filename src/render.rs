/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::{
    header::{uniform_type, Camera, Renderer, ShaderSettings, TextureData},
    utils::default_resources::{DEFAULT_COLOR, DEFAULT_MATRIX_4, DEFAULT_SHADER, DEFAULT_TEXTURE},
    ObjectStorage, PipelineData,
};
use winit::window::Window;

impl Renderer {
    /// Creates a new renderer.
    ///
    /// # Arguments
    /// * `window` - The window to create the renderer for.
    /// * `power_preference` - The power preference to use.
    pub(crate) async fn new(
        window: &Window,
        settings: crate::WindowDescriptor,
    ) -> color_eyre::Result<Self> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: settings.backends,
            ..Default::default()
        });
        #[cfg(not(feature = "android"))]
        let surface = Some(unsafe {
            instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window)?)?
        });
        #[cfg(feature = "android")]
        let surface = None;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: settings.power_preference,
                #[cfg(not(feature = "android"))]
                compatible_surface: Some(&surface.as_ref().unwrap()),
                #[cfg(feature = "android")]
                compatible_surface: surface,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device"),
                    required_features: settings.features,
                    required_limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        #[cfg(not(feature = "android"))]
        let tex_format = surface.as_ref().unwrap().get_capabilities(&adapter).formats[0];

        #[cfg(feature = "android")]
        let tex_format = wgpu::TextureFormat::Rgba8UnormSrgb;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: tex_format, //wgpu::TextureFormat::Bgra8UnormSrgb,
            #[cfg(feature = "android")]
            width: 1080,
            #[cfg(not(feature = "android"))]
            width: size.width,
            #[cfg(feature = "android")]
            height: 2300,
            #[cfg(not(feature = "android"))]
            height: size.height,
            #[cfg(feature = "android")]
            present_mode: wgpu::PresentMode::Mailbox,
            #[cfg(not(feature = "android"))]
            present_mode: settings.present_mode,
            alpha_mode: settings.alpha_mode,
            view_formats: vec![tex_format],
            desired_maximum_frame_latency: settings.desired_maximum_frame_latency,
        };
        #[cfg(not(feature = "android"))]
        surface.as_ref().unwrap().configure(&device, &config);

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
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), //comparison: false,
                        // filtering: true,
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

        let mut renderer = Self {
            #[cfg(feature = "android")]
            instance,
            adapter,
            #[cfg(not(feature = "android"))]
            surface,
            #[cfg(feature = "android")]
            surface: None,
            device,
            queue,
            config,
            size,

            texture_bind_group_layout,
            default_uniform_bind_group_layout,
            depth_buffer,

            default_data: None,
            camera: None,
            clear_color: wgpu::Color::BLACK,
            scissor_rect: None,
        };

        let default_texture = renderer.build_texture(
            "Default Texture",
            TextureData::Bytes(DEFAULT_TEXTURE.to_vec()),
            crate::header::TextureMode::Clamp,
            //crate::header::TextureFormat::PNG
        )?;

        let default_uniform = renderer.build_uniform_buffer(&vec![
            renderer.build_uniform_buffer_part("Transformation Matrix", DEFAULT_MATRIX_4),
            renderer.build_uniform_buffer_part(
                "Color",
                uniform_type::Array4 {
                    data: DEFAULT_COLOR,
                },
            ),
        ])?;

        let default_shader = renderer.build_shader(
            "Default Shader",
            DEFAULT_SHADER.to_string(),
            Some(&default_uniform.1),
            ShaderSettings::default(),
        )?;

        renderer.default_data = Some((default_texture, default_shader, default_uniform.0));

        Ok(renderer)
    }

    /// Resize the window.
    /// # Arguments
    /// * `new_size` - The new window size.
    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // check if new_size is non zero
        if new_size.width != 0 && new_size.height != 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            #[cfg(not(feature = "android"))]
            self.surface
                .as_ref()
                .unwrap()
                .configure(&self.device, &self.config);
            #[cfg(not(feature = "android"))]
            {
                self.depth_buffer =
                    Self::build_depth_buffer("Depth Buffer", &self.device, &self.config);
            }
        }
    }

    /// Render the scene. Returns the command encoder, the texture view, and the surface texture.
    ///
    /// # Arguments
    /// * `objects` - The object storage.
    /// * `camera` - The camera.
    pub(crate) fn pre_render(
        &mut self,
        objects: &ObjectStorage,
        window_size: winit::dpi::PhysicalSize<u32>,
        camera: &Camera,
    ) -> Result<
        Option<(
            wgpu::CommandEncoder,
            wgpu::TextureView,
            wgpu::SurfaceTexture,
        )>,
        wgpu::SurfaceError,
    > {
        let surface = if let Some(ref surface) = self.surface {
            surface
        } else {
            return Ok(None);
        };

        let frame = if let Ok(frame) = surface.get_current_texture() {
            frame
        } else {
            return Ok(None);
        };

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
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_buffer.1,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        if self.scissor_rect.is_some() {
            let scissor_rect = self.scissor_rect.unwrap();
            // check if scissor bounds are smaller than the window
            if scissor_rect.0 + scissor_rect.2 < window_size.width as u32
                && scissor_rect.1 + scissor_rect.3 < window_size.height as u32
            {
                render_pass.set_scissor_rect(
                    scissor_rect.0,
                    scissor_rect.1,
                    scissor_rect.2,
                    scissor_rect.3,
                );
            }
        }

        let default_data = self.default_data.as_ref().unwrap();

        render_pass.set_bind_group(0, &default_data.0, &[]);
        render_pass.set_pipeline(&default_data.1);
        render_pass.set_bind_group(1, &camera.uniform_data, &[]);

        // sort the object list in descending render order
        let mut object_list: Vec<_> = objects.iter().collect();
        object_list.sort_by(|(_, a), (_, b)| a.render_order.cmp(&b.render_order).reverse());

        for (_, i) in object_list {
            if i.is_visible {
                let i = i;

                let vertex_buffer = get_pipeline_vertex_buffer(&i.pipeline.vertex_buffer, objects);
                let shader = get_pipeline_shader(&i.pipeline.shader, objects);
                let texture = get_pipeline_texture(&i.pipeline.texture, objects);
                let uniform = get_pipeline_uniform_buffer(&i.pipeline.uniform, objects);

                // vertex
                if vertex_buffer.is_some() {
                    let vertex_buffer = vertex_buffer.unwrap();
                    render_pass.set_vertex_buffer(0, vertex_buffer.vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, i.instance_buffer.slice(..));
                    render_pass.set_index_buffer(
                        vertex_buffer.index_buffer.slice(..),
                        wgpu::IndexFormat::Uint16,
                    );

                    // shader
                    if shader.is_some() {
                        render_pass.set_pipeline(&shader.unwrap());
                    }
                    // texture
                    if texture.is_some() {
                        render_pass.set_bind_group(0, &texture.unwrap(), &[]);
                    }
                    // uniform
                    if uniform.is_some() {
                        let uniform = uniform.unwrap();
                        if uniform.is_some() {
                            render_pass.set_bind_group(2, uniform.as_ref().unwrap(), &[]);
                        }
                    }
                    render_pass.draw_indexed(0..vertex_buffer.length, 0, 0..i.instances.len() as _);
                }
            }
        }
        drop(render_pass);

        Ok(Some((encoder, view, frame)))
    }

    /// Render the scene.
    ///
    /// # Arguments
    /// * `encoder` - The command encoder.
    /// * `frame` - The surface texture.
    pub(crate) fn render(
        &mut self,
        encoder: wgpu::CommandEncoder,
        frame: wgpu::SurfaceTexture,
    ) -> Result<(), wgpu::SurfaceError> {
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}

// =========================== Extract Pipeline Data ===========================
// I couldn't make them into one function, so here they are, four of them

/// Get the pipeline vertex buffer.
fn get_pipeline_vertex_buffer<'a>(
    data: &'a PipelineData<crate::VertexBuffers>,
    objects: &'a ObjectStorage,
) -> Option<&'a crate::VertexBuffers> {
    match data {
        PipelineData::Copy(object_id) => {
            let data = objects.get(object_id.as_str());
            if data.is_some() {
                get_pipeline_vertex_buffer(&data.unwrap().pipeline.vertex_buffer, objects)
            } else {
                None
            }
        }
        PipelineData::Data(data) => Some(data),
    }
}

/// Get the pipeline shader.
fn get_pipeline_shader<'a>(
    data: &'a PipelineData<crate::Shaders>,
    objects: &'a ObjectStorage,
) -> Option<&'a crate::Shaders> {
    match data {
        PipelineData::Copy(object_id) => {
            let data = objects.get(object_id.as_str());
            if data.is_some() {
                get_pipeline_shader(&data.unwrap().pipeline.shader, objects)
            } else {
                None
            }
        }
        PipelineData::Data(data) => Some(data),
    }
}

/// Get the pipeline texture.
fn get_pipeline_texture<'a>(
    data: &'a PipelineData<crate::Textures>,
    objects: &'a ObjectStorage,
) -> Option<&'a crate::Textures> {
    match data {
        PipelineData::Copy(object_id) => {
            let data = objects.get(object_id.as_str());
            if data.is_some() {
                get_pipeline_texture(&data.unwrap().pipeline.texture, objects)
            } else {
                None
            }
        }
        PipelineData::Data(data) => Some(data),
    }
}

/// Get the pipeline uniform_buffer.
fn get_pipeline_uniform_buffer<'a>(
    data: &'a PipelineData<Option<crate::UniformBuffers>>,
    objects: &'a ObjectStorage,
) -> Option<&'a Option<crate::UniformBuffers>> {
    match data {
        PipelineData::Copy(object_id) => {
            let data = objects.get(object_id.as_str());
            if data.is_some() {
                get_pipeline_uniform_buffer(&data.unwrap().pipeline.uniform, objects)
            } else {
                None
            }
        }
        PipelineData::Data(data) => Some(data),
    }
}

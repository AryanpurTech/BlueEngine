use crate::{
    CameraContainer, ObjectStorage, PipelineData, WindowSize,
    prelude::{ShaderSettings, TextureData},
    utils::default_resources::{DEFAULT_COLOR, DEFAULT_SHADER, DEFAULT_TEXTURE},
};

/// Main renderer class. this will contain all methods and data related to the renderer
#[derive(Debug)]
pub struct Renderer {
    /// A [`wgpu::Surface`] represents a platform-specific surface
    /// (e.g. a window) onto which rendered images may be presented.
    pub surface: Option<wgpu::Surface<'static>>,
    /// Context for all of the gpu objects
    pub instance: wgpu::Instance,
    /// Handle to a physical graphics and/or compute device.
    #[allow(unused)]
    pub adapter: wgpu::Adapter,
    /// Open connection to a graphics and/or compute device.
    pub device: wgpu::Device,
    /// Handle to a command queue on a device.
    pub queue: wgpu::Queue,
    /// Describes a [`wgpu::Surface`]
    pub config: wgpu::SurfaceConfiguration,
    /// The size of the window
    pub size: WindowSize,
    /// The texture bind group layout
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    /// The uniform bind group layout
    pub default_uniform_bind_group_layout: wgpu::BindGroupLayout,
    /// The depth buffer, used to render object depth
    pub depth_buffer: (wgpu::Texture, wgpu::TextureView, wgpu::Sampler),
    /// The default data used within the renderer
    pub default_data: Option<(crate::Textures, crate::Shaders, crate::UniformBuffers)>,
    /// The camera used in the engine
    pub camera: Option<crate::UniformBuffers>,
    /// Background clear color
    pub clear_color: wgpu::Color,
    /// Scissor cut section of the screen to render to
    /// (x, y, width, height)
    pub scissor_rect: Option<(u32, u32, u32, u32)>,
    /// The texture data that holds data for the headless mode
    #[cfg(feature = "headless")]
    pub headless_texture_data: Vec<u8>,
}
unsafe impl Sync for Renderer {}
unsafe impl Send for Renderer {}

impl Renderer {
    /// Creates a new renderer.
    pub(crate) async fn new(
        size: WindowSize,
        settings: crate::EngineSettings,
    ) -> Result<Self, crate::error::Error> {
        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: settings.backends,
            ..Default::default()
        });

        match instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: settings.power_preference,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
        {
            Ok(adapter) => {
                let (device, queue) = adapter
                    .request_device(&wgpu::DeviceDescriptor {
                        label: Some("Device"),
                        required_features: settings.features,
                        required_limits: settings.limits,
                        memory_hints: if settings.power_preference
                            == wgpu::PowerPreference::HighPerformance
                        {
                            wgpu::MemoryHints::Performance
                        } else {
                            wgpu::MemoryHints::MemoryUsage
                        },
                        trace: wgpu::Trace::Off,
                        ..Default::default()
                    })
                    .await?;

                let texture_format = wgpu::TextureFormat::Bgra8UnormSrgb;

                #[cfg(target_os = "android")]
                let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;

                let config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: texture_format, //wgpu::TextureFormat::Bgra8UnormSrgb,
                    #[cfg(target_os = "android")]
                    width: 1080,
                    #[cfg(not(feature = "android"))]
                    width: size.0,
                    #[cfg(target_os = "android")]
                    height: 2300,
                    #[cfg(not(target_os = "android"))]
                    height: size.1,
                    #[cfg(target_os = "android")]
                    present_mode: wgpu::PresentMode::Mailbox,
                    #[cfg(not(target_os = "android"))]
                    present_mode: settings.present_mode,
                    alpha_mode: settings.alpha_mode,
                    view_formats: vec![texture_format],
                    desired_maximum_frame_latency: settings.desired_maximum_frame_latency,
                };

                let texture_bind_group_layout =
                    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Texture {
                                    sample_type: wgpu::TextureSampleType::Float {
                                        filterable: true,
                                    },
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
                    instance,
                    adapter,
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

                    #[cfg(feature = "headless")]
                    headless_texture_data: Vec::<u8>::with_capacity((size.0 * size.1) as usize * 4),
                };

                renderer.build_default_data();

                Ok(renderer)
            }
            Err(e) => Err(crate::error::Error::AdapterNotFound(e)),
        }
    }

    pub(crate) fn build_default_data(&mut self) {
        if let Ok(default_texture) = self.build_texture(
            "Default Texture",
            TextureData::Bytes(DEFAULT_TEXTURE.to_vec()),
            crate::prelude::TextureMode::Clamp,
            //crate::prelude::TextureFormat::PNG
        ) {
            let default_uniform = self.build_uniform_buffer(&vec![
                self.build_uniform_buffer_part("Transformation Matrix", crate::Matrix4::IDENTITY),
                self.build_uniform_buffer_part("Color", DEFAULT_COLOR),
            ]);

            let default_shader = self.build_shader(
                "Default Shader",
                DEFAULT_SHADER.to_string(),
                Some(&default_uniform.1),
                ShaderSettings::default(),
            );

            self.default_data = Some((default_texture, default_shader, default_uniform.0));
        } else {
            eprintln!("Could not build the default texture, there may be something wrong!");
            self.default_data = None;
        }
    }

    /// Resize the window.
    #[cfg(all(not(feature = "headless"), feature = "window"))]
    pub(crate) fn resize(&mut self, new_size: WindowSize) {
        // check if new_size is non-zero
        if new_size.0 != 0 && new_size.1 != 0 {
            self.size = new_size;
            self.config.width = new_size.0;
            self.config.height = new_size.1;
            #[cfg(not(target_os = "android"))]
            if let Some(surface) = self.surface.as_ref() {
                surface.configure(&self.device, &self.config);
                self.depth_buffer =
                    Self::build_depth_buffer("Depth Buffer", &self.device, &self.config);
            }
        }
    }

    /// Render the scene. Returns the command encoder, the texture view, and the surface texture.
    pub(crate) fn pre_render(
        &mut self,
        objects: &ObjectStorage,
        window_size: WindowSize,
        camera: &CameraContainer,
    ) -> Result<
        Option<(
            wgpu::CommandEncoder,
            wgpu::TextureView,
            Option<wgpu::SurfaceTexture>,
            Option<(wgpu::Buffer, wgpu::Texture)>,
        )>,
        wgpu::SurfaceError,
    > {
        #[cfg(not(feature = "headless"))]
        let surface = if let Some(ref surface) = self.surface {
            surface
        } else {
            return Ok(None);
        };
        #[cfg(not(feature = "headless"))]
        let frame = if let Ok(frame) = surface.get_current_texture() {
            frame
        } else {
            return Ok(None);
        };

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        #[cfg(feature = "headless")]
        let render_target = self.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.config.width,
                height: self.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[self.config.format],
        });

        // ? There might be a way to enable both headless and normal rendering,
        // ? However the cost of it can increase a lot
        #[cfg(feature = "headless")]
        let view = render_target.create_view(&wgpu::TextureViewDescriptor::default());
        #[cfg(not(feature = "headless"))]
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        #[cfg(feature = "headless")]
        let headless_output_staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: self.headless_texture_data.capacity() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
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
                depth_slice: None,
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

        if let Some(scissor_rect) = self.scissor_rect {
            // check if scissor bounds are smaller than the window
            if scissor_rect.0 + scissor_rect.2 < window_size.0
                && scissor_rect.1 + scissor_rect.3 < window_size.1
            {
                render_pass.set_scissor_rect(
                    scissor_rect.0,
                    scissor_rect.1,
                    scissor_rect.2,
                    scissor_rect.3,
                );
            }
        }

        if let Some(default_data) = self.default_data.as_ref() {
            render_pass.set_bind_group(0, &default_data.0, &[]);
            render_pass.set_pipeline(&default_data.1);
        }

        // sort the object list in descending render order
        // ! There needs to be a better way for this, to not iterate twice
        let mut object_list = objects.iter().collect::<Vec<_>>();
        object_list.sort_by(|(_, a), (_, b)| a.render_order.cmp(&b.render_order).reverse());

        for (_, i) in object_list {
            if let Some(camera_data) = i.camera_effect.as_ref() {
                if let Some(camera) = camera.get(camera_data.as_ref()) {
                    render_pass.set_bind_group(1, &camera.uniform_data, &[]);
                }
            } else {
                if let Some(main_camera) = camera.get("main") {
                    render_pass.set_bind_group(1, &main_camera.uniform_data, &[]);
                }
            }

            if i.is_visible {
                let vertex_buffer = get_pipeline_vertex_buffer(&i.pipeline.vertex_buffer, objects);
                let shader = get_pipeline_shader(&i.pipeline.shader, objects);
                let texture = get_pipeline_texture(&i.pipeline.texture, objects);
                let uniform = get_pipeline_uniform_buffer(&i.pipeline.uniform, objects);

                // vertex
                if let Some(vertex_buffer) = vertex_buffer {
                    render_pass.set_vertex_buffer(0, vertex_buffer.vertex_buffer.slice(..));
                    render_pass.set_vertex_buffer(1, i.instance_buffer.slice(..));
                    render_pass.set_index_buffer(
                        vertex_buffer.index_buffer.slice(..),
                        #[cfg(not(feature = "u32"))]
                        wgpu::IndexFormat::Uint16,
                        #[cfg(feature = "u32")]
                        wgpu::IndexFormat::Uint32,
                    );

                    // shader
                    if let Some(shader) = shader {
                        render_pass.set_pipeline(shader);
                    }
                    // texture
                    if let Some(texture) = texture {
                        render_pass.set_bind_group(0, texture, &[]);
                    }
                    // uniform
                    if let Some(Some(uniform)) = uniform {
                        render_pass.set_bind_group(2, uniform, &[]);
                    }
                    render_pass.draw_indexed(0..vertex_buffer.length, 0, 0..i.instances.len() as _);
                }
            }
        }
        drop(render_pass);

        Ok(Some((
            encoder,
            view,
            #[cfg(feature = "headless")]
            None,
            #[cfg(not(feature = "headless"))]
            Some(frame),
            #[cfg(feature = "headless")]
            Some((headless_output_staging_buffer, render_target)),
            #[cfg(not(feature = "headless"))]
            None,
        )))
    }

    /// Render the scene.
    pub(crate) fn render(
        &mut self,
        encoder: wgpu::CommandEncoder,
        _frame: Option<wgpu::SurfaceTexture>,
        _headless: Option<(wgpu::Buffer, wgpu::Texture)>,
    ) {
        #[cfg(feature = "headless")]
        {
            #[allow(clippy::expect_used)]
            let (output_staging_buffer, render_target) =
                _headless.expect("Error unpacking headless content. This should not error!");

            let mut encoder = encoder;

            #[cfg(feature = "headless")]
            encoder.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    texture: &render_target,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &output_staging_buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        // This needs to be a multiple of 256. Normally we would need to pad
                        // it but we here know it will work out anyways.
                        bytes_per_row: Some(self.config.width * 4),
                        rows_per_image: Some(self.config.height),
                    },
                },
                wgpu::Extent3d {
                    width: self.config.width,
                    height: self.config.height,
                    depth_or_array_layers: 1,
                },
            );

            // submit will accept anything that implements IntoIter
            self.queue.submit(Some(encoder.finish()));

            pollster::block_on(async {
                let buffer_slice = output_staging_buffer.slice(..);
                let (sender, receiver) = std::sync::mpsc::channel();
                buffer_slice.map_async(wgpu::MapMode::Read, move |r| sender.send(r).unwrap());
                self.device
                    .poll(wgpu::PollType::wait_indefinitely())
                    .unwrap();
                receiver.recv().unwrap().unwrap();
                {
                    let view = buffer_slice.get_mapped_range();
                    self.headless_texture_data.extend_from_slice(&view[..]);
                }
                output_staging_buffer.unmap();
            });
        }

        #[cfg(not(feature = "headless"))]
        if let Some(frame) = _frame {
            // submit will accept anything that implements IntoIter
            self.queue.submit(Some(encoder.finish()));

            frame.present();
        }
    }

    /// Sets the background color
    pub fn set_clear_color(&mut self, r: f64, g: f64, b: f64, a: f64) {
        self.clear_color = wgpu::Color { r, g, b, a }
    }
}

// =========================== Extract Pipeline Data ===========================
macro_rules! gen_pipeline {
    ($function_name:ident, $buffer_type:ty, $buffer_field:ident) => {
        fn $function_name<'a>(
            data: &'a PipelineData<$buffer_type>,
            objects: &'a ObjectStorage,
        ) -> Option<&'a $buffer_type> {
            match data {
                PipelineData::Copy(object_id) => {
                    let data = objects.get(object_id);
                    if let Some(data) = data {
                        $function_name(&data.pipeline.$buffer_field, objects)
                    } else {
                        None
                    }
                }
                PipelineData::Data(data) => Some(data),
            }
        }
    };
}

gen_pipeline!(
    get_pipeline_vertex_buffer,
    crate::VertexBuffers,
    vertex_buffer
);
gen_pipeline!(get_pipeline_shader, crate::Shaders, shader);
gen_pipeline!(get_pipeline_texture, crate::Textures, texture);

/// Get the pipeline uniform_buffer.
fn get_pipeline_uniform_buffer<'a>(
    data: &'a PipelineData<Option<crate::UniformBuffers>>,
    objects: &'a ObjectStorage,
) -> Option<&'a Option<crate::UniformBuffers>> {
    match data {
        PipelineData::Copy(object_id) => {
            let data = objects.get(object_id);
            if let Some(data) = data {
                get_pipeline_uniform_buffer(&data.pipeline.uniform, objects)
            } else {
                None
            }
        }
        PipelineData::Data(data) => Some(data),
    }
}

// ? ADD VISIBILITY TAGS FOR DIFFERENT RENDER PASS TO USE AND RENDER ONLY THE OBJECTS THEY NEED

use crate::{
    CameraContainer, ObjectStorage, PipelineData,
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
    pub size: winit::dpi::PhysicalSize<u32>,
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
}
unsafe impl Sync for Renderer {}
unsafe impl Send for Renderer {}

impl Renderer {
    /// Creates a new renderer.
    pub(crate) async fn new(
        size: winit::dpi::PhysicalSize<u32>,
        settings: crate::WindowDescriptor,
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
            Some(adapter) => {
                let (device, queue) = adapter
                    .request_device(
                        &wgpu::DeviceDescriptor {
                            label: Some("Device"),
                            required_features: settings.features,
                            required_limits: settings.limits,
                            memory_hints: wgpu::MemoryHints::Performance,
                        },
                        None, // Trace path
                    )
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
                    width: size.width,
                    #[cfg(target_os = "android")]
                    height: 2300,
                    #[cfg(not(target_os = "android"))]
                    height: size.height,
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
                };

                renderer.build_default_data();

                Ok(renderer)
            }
            None => Err(crate::error::Error::AdapterNotFound),
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
    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // check if new_size is non-zero
        if new_size.width != 0 && new_size.height != 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            #[cfg(not(target_os = "android"))]
            if let Some(surface) = self.surface.as_ref() {
                surface.configure(&self.device, &self.config);
                {
                    self.depth_buffer =
                        Self::build_depth_buffer("Depth Buffer", &self.device, &self.config);
                }
            }
        }
    }

    /// Render the scene. Returns the command encoder, the texture view, and the surface texture.
    pub(crate) fn pre_render(
        &mut self,
        objects: &ObjectStorage,
        window_size: winit::dpi::PhysicalSize<u32>,
        camera: &CameraContainer,
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

        if let Some(scissor_rect) = self.scissor_rect {
            // check if scissor bounds are smaller than the window
            if scissor_rect.0 + scissor_rect.2 < window_size.width
                && scissor_rect.1 + scissor_rect.3 < window_size.height
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
        let mut object_list: Vec<_> = objects.iter().collect();
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

        Ok(Some((encoder, view, frame)))
    }

    /// Render the scene.
    pub(crate) fn render(&mut self, encoder: wgpu::CommandEncoder, frame: wgpu::SurfaceTexture) {
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
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
                    let data = objects.get(object_id.as_str());
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
            let data = objects.get(object_id.as_str());
            if let Some(data) = data {
                get_pipeline_uniform_buffer(&data.pipeline.uniform, objects)
            } else {
                None
            }
        }
        PipelineData::Data(data) => Some(data),
    }
}

/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::{
    header::{uniform_type, Camera, Renderer, ShaderSettings, TextureData},
    utils::default_resources::{DEFAULT_COLOR, DEFAULT_MATRIX_4, DEFAULT_SHADER, DEFAULT_TEXTURE},
    ObjectStorage,
};
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
    /// Creates a new renderer.
    ///
    /// # Arguments
    /// * `window` - The window to create the renderer for.
    /// * `power_preference` - The power preference to use.
    pub(crate) async fn new(
        window: &Window,
        power_preference: crate::PowerPreference,
    ) -> anyhow::Result<Self> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        #[cfg(not(feature = "android"))]
        let surface = Some(unsafe { instance.create_surface(window) }.unwrap());
        #[cfg(feature = "android")]
        let surface = None;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: power_preference,
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
                    features: get_render_features(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        #[cfg(not(feature = "android"))]
        let tex_format = {
            let format = surface.as_ref().unwrap().get_capabilities(&adapter);
            format
                .formats
                .iter()
                .copied()
                .filter(|f| f.describe().srgb)
                .next()
                .unwrap_or(format.formats[0])
        };
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
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![tex_format],
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

    /// Render the scene. Returns the command encoder, the texture view, and the surface texture.
    ///
    /// # Arguments
    /// * `objects` - The object storage.
    /// * `camera` - The camera.
    pub(crate) fn pre_render(
        &mut self,
        objects: &ObjectStorage,
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

        let frame = surface.get_current_texture()?;
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
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_buffer.1,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        //? Scissor
        //render_pass.set_scissor_rect(50, 50, 500, 500);

        let default_data = self.default_data.as_ref().unwrap();

        render_pass.set_bind_group(0, &default_data.0, &[]);
        render_pass.set_pipeline(&default_data.1);
        render_pass.set_bind_group(1, &camera.uniform_data, &[]);

        for i in objects.iter() {
            if i.1.is_visible {
                let i = i.1;
                render_pass.set_pipeline(&i.pipeline.shader);
                render_pass.set_bind_group(0, &i.pipeline.texture, &[]);
                if i.pipeline.uniform.is_some() {
                    render_pass.set_bind_group(2, &i.pipeline.uniform.as_ref().unwrap(), &[]);
                }
                render_pass.set_vertex_buffer(0, i.pipeline.vertex_buffer.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    i.pipeline.vertex_buffer.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.draw_indexed(0..i.pipeline.vertex_buffer.length, 0, 0..1);
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

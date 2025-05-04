#![cfg(feature = "egui")]

use blue_engine::{
    CommandEncoder, DEPTH_FORMAT, Matrix4, TextureView, Vector4, Window as Win, wgpu,
};
pub use egui;
use egui::ViewportId;

/// The egui plugin
pub struct EGUIPlugin {
    pub context: Option<egui::Context>,
    pub platform: Option<egui_winit::State>,
    pub renderer: Option<egui_wgpu::Renderer>,
    pub full_output: Option<egui::FullOutput>,
    pub raw_input: Option<egui::RawInput>,
}

impl EGUIPlugin {
    /// Creates the egui context and platform details
    pub fn new() -> Self {
        Self {
            context: None,
            platform: None,
            renderer: None,
            full_output: None,
            raw_input: None,
        }
    }

    pub fn ui<F: FnMut(&egui::Context)>(&mut self, callback: F, window: &Win) {
        if let Some(window) = window.window.as_ref() {
            let raw_input = if let Some(platform) = self.platform.as_mut() {
                let raw_input = platform.take_egui_input(window).clone();
                Some(raw_input.clone())
            } else {
                None
            };

            if let Some(context) = self.context.as_ref() {
                if let Some(raw_input) = raw_input {
                    self.full_output = Some(context.run(raw_input.clone(), callback));
                }
            }
        }
    }
}

impl Default for EGUIPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl blue_engine::Signal for EGUIPlugin {
    fn init(&mut self, engine: &mut blue_engine::Engine) {
        if let Some(window) = engine.window.window.as_ref() {
            let context = egui::Context::default();

            let platform = egui_winit::State::new(
                // ! IN CASE IT CRASHES, this is the culprit
                context.clone(),
                ViewportId::ROOT,
                &window,
                #[cfg(not(target_os = "android"))]
                Some(window.scale_factor() as f32),
                #[cfg(target_os = "android")]
                None,
                Some(egui_winit::winit::window::Theme::Dark),
                #[cfg(not(target_os = "android"))]
                Some(engine.renderer.device.limits().max_texture_dimension_2d as usize),
                #[cfg(target_os = "android")]
                None,
            );
            #[cfg(target_os = "android")]
            let format = blue_engine::wgpu::TextureFormat::Rgba8UnormSrgb;

            #[cfg(not(target_os = "android"))]
            let format = engine.renderer.config.format;

            let renderer = egui_wgpu::Renderer::new(
                &engine.renderer.device,
                format,
                Some(DEPTH_FORMAT),
                1,
                true,
            );

            self.platform = Some(platform);
            self.renderer = Some(renderer);
            self.context = Some(context);
        }
    }
    fn window_events(
        &mut self,
        engine: &mut blue_engine::Engine,
        event: &blue_engine::WindowEvent,
    ) {
        if let Some(window) = engine.window.window.as_ref() {
            if let Some(platform) = self.platform.as_mut() {
                //? has a return, maybe useful in the future
                let _ = platform.on_window_event(window.as_ref(), event);
            }
        }
    }

    fn frame(
        &mut self,
        engine: &mut blue_engine::Engine,
        encoder: &mut CommandEncoder,
        view: &TextureView,
    ) {
        if let Some(window) = engine.window.as_ref() {
            if engine.renderer.surface.is_some() && self.full_output.is_some() {
                let egui::FullOutput {
                    platform_output,
                    textures_delta,
                    shapes,
                    pixels_per_point,
                    ..
                } = self
                    .full_output
                    .as_ref()
                    .expect("Failed to get egui output");

                if let Some(platform) = self.platform.as_mut() {
                    platform.handle_platform_output(window, platform_output.clone());
                }

                let paint_jobs = self
                    .context
                    .as_ref()
                    .map(|context| context.tessellate(shapes.clone(), *pixels_per_point));

                let screen_descriptor = egui_wgpu::ScreenDescriptor {
                    size_in_pixels: [
                        engine.renderer.config.width,
                        #[cfg(target_os = "android")]
                        {
                            renderer.config.height - 20
                        },
                        #[cfg(not(target_os = "android"))]
                        engine.renderer.config.height,
                    ],
                    pixels_per_point: *pixels_per_point,
                };

                if let Some(renderer) = self.renderer.as_mut() {
                    if let Some(paint_jobs) = paint_jobs {
                        for (id, image_delta) in &textures_delta.set {
                            renderer.update_texture(
                                &engine.renderer.device,
                                &engine.renderer.queue,
                                *id,
                                image_delta,
                            );
                        }

                        renderer.update_buffers(
                            &engine.renderer.device,
                            &engine.renderer.queue,
                            encoder,
                            &paint_jobs,
                            &screen_descriptor,
                        );

                        let render_pass =
                            encoder.begin_render_pass(&blue_engine::RenderPassDescriptor {
                                label: Some("Render pass"),
                                color_attachments: &[Some(
                                    blue_engine::RenderPassColorAttachment {
                                        view,
                                        resolve_target: None,
                                        ops: blue_engine::Operations {
                                            load: blue_engine::LoadOp::Load,
                                            store: wgpu::StoreOp::Store,
                                        },
                                    },
                                )],
                                depth_stencil_attachment: Some(
                                    wgpu::RenderPassDepthStencilAttachment {
                                        view: &engine.renderer.depth_buffer.1,
                                        depth_ops: Some(wgpu::Operations {
                                            load: wgpu::LoadOp::Clear(1.0),
                                            store: wgpu::StoreOp::Store,
                                        }),
                                        stencil_ops: None,
                                    },
                                ),
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            });

                        let mut render_pass = render_pass.forget_lifetime();
                        renderer.render(&mut render_pass, &paint_jobs, &screen_descriptor);
                    }
                }
            }
        }
    }
}

// ===============================================================================================
// It's usage needs to be checked more
struct Callback;
impl egui_wgpu::CallbackTrait for Callback {
    fn paint(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        resources: &egui_wgpu::CallbackResources,
    ) {
        let resources: &TriangleRenderResources = resources.get().unwrap();
        render_pass.set_bind_group(0, &resources.default_data.0, &[]);
        render_pass.set_pipeline(&resources.default_data.1);
        render_pass.set_bind_group(1, &resources.camera_data, &[]);

        // Draw our triangle!
        render_pass.set_pipeline(&resources.shader);
        render_pass.set_bind_group(0, &resources.texture, &[]);

        render_pass.set_bind_group(2, &resources.uniform, &[]);

        render_pass.set_vertex_buffer(0, resources.vertex_buffer.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            resources.vertex_buffer.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(0..resources.vertex_buffer.length, 0, 0..1);
    }
}

struct TriangleRenderResources {
    pub shader: wgpu::RenderPipeline,
    pub vertex_buffer: blue_engine::VertexBuffers,
    pub texture: wgpu::BindGroup,
    pub uniform: blue_engine::UniformBuffers,
    pub default_data: (
        blue_engine::Textures,
        blue_engine::Shaders,
        blue_engine::UniformBuffers,
    ),
    pub camera_data: wgpu::BindGroup,
}

pub struct EmbeddedRender {
    object: blue_engine::Object,
}
impl EmbeddedRender {
    pub fn new(
        object: blue_engine::Object,
        cc: &mut blue_engine::Renderer,
        renderer: &mut egui_wgpu::Renderer,
    ) -> Self {
        let mut object = object;
        let buffers = object.update_and_return(cc);

        let camera_data = cc.build_uniform_buffer(&[
            cc.build_uniform_buffer_part("Camera Uniform", Matrix4::IDENTITY)
        ]);

        let default_texture = cc
            .build_texture(
                "Default Texture",
                blue_engine::TextureData::Bytes(
                    blue_engine::utils::default_resources::DEFAULT_TEXTURE.to_vec(),
                ),
                blue_engine::TextureMode::Clamp,
                //crate::header::TextureFormat::PNG
            )
            .unwrap();

        let default_texture_2 = cc
            .build_texture(
                "Default Texture",
                blue_engine::TextureData::Bytes(
                    blue_engine::utils::default_resources::DEFAULT_TEXTURE.to_vec(),
                ),
                blue_engine::TextureMode::Clamp,
            )
            .unwrap();

        let default_uniform = cc.build_uniform_buffer(&[
            cc.build_uniform_buffer_part("Transformation Matrix", Matrix4::IDENTITY),
            cc.build_uniform_buffer_part("Color", Vector4::ONE),
        ]);

        let default_shader = cc.build_shader(
            "Default Shader",
            blue_engine::utils::default_resources::DEFAULT_SHADER.to_string(),
            Some(&default_uniform.1),
            blue_engine::ShaderSettings::default(),
        );

        renderer.callback_resources.insert(TriangleRenderResources {
            shader: buffers.2,
            texture: default_texture,
            vertex_buffer: buffers.0,
            uniform: buffers.1,
            default_data: (default_texture_2, default_shader, default_uniform.0),
            camera_data: camera_data.0,
        });

        Self { object }
    }

    pub fn prepare(
        &mut self,
        brenderer: &mut blue_engine::Renderer,
        erenderer: &mut egui_wgpu::Renderer,
        camera_data: blue_engine::UniformBuffers,
    ) {
        let object_pipeline = self.object.update_and_return(brenderer);

        let resources: &mut TriangleRenderResources =
            erenderer.callback_resources.get_mut().unwrap();

        resources.vertex_buffer = object_pipeline.0;
        resources.uniform = object_pipeline.1;
        resources.shader = object_pipeline.2;
        resources.camera_data = camera_data;
    }

    pub fn paint(&mut self, ui: &mut egui::Ui) {
        let space = ui.available_size();

        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(space.x - 5f32, space.y - 5f32),
            egui::Sense::drag(),
        );

        let callback = egui_wgpu::Callback::new_paint_callback(rect, Callback {});

        ui.painter().add(callback);
    }
}

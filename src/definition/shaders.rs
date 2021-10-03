use crate::header::{Shaders, Vertex};
use wgpu::BindGroupLayout;

impl crate::header::Renderer {
    /// Creates and adds the shaders to render queue
    pub fn build_and_append_shaders(
        &mut self,
        name: &'static str,
        shader_source: String,
        uniform_layout: Option<&BindGroupLayout>,
    ) -> Result<usize, anyhow::Error> {
        let shaders = self
            .build_shaders(name, shader_source, uniform_layout)
            .expect("Couldn't create shaders");
        let index = self.shaders.len();
        self.shaders.push(shaders);
        Ok(index)
    }

    /// Creates a shader group, the input must be spir-v compiled vertex and fragment shader
    pub fn build_shaders(
        &mut self,
        name: &str,
        shader_source: String,
        uniform_layout: Option<&BindGroupLayout>,
    ) -> Result<Shaders, anyhow::Error> {
        let shader = self
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
                //flags: wgpu::ShaderFlags::all(),
            });

        let mut bind_group_layouts = vec![
            &self.texture_bind_group_layout,
            &self.default_uniform_bind_group_layout,
        ];
        if uniform_layout.is_some() {
            bind_group_layouts.push(uniform_layout.unwrap())
        }

        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &bind_group_layouts.as_slice(),
                    push_constant_ranges: &[],
                });

        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(name),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: self.config.format,
                        write_mask: wgpu::ColorWrites::ALL,
                        blend: Some(wgpu::BlendState::REPLACE),
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None, //Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false,
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
}

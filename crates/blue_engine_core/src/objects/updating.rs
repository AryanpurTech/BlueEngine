use super::{Instance, Object};
use crate::{Matrix4, PipelineData, Renderer};

impl Object {
    /// Update and apply changes done to an object
    pub fn update(&mut self, renderer: &mut Renderer) {
        self.update_vertex_buffer(renderer);
        self.update_uniform_buffer(renderer);
        self.update_shader(renderer);
        self.update_instance_buffer(renderer);
        self.changed = false;
    }

    /// Update and apply changes done to an object and returns a pipeline
    pub fn update_and_return(
        &mut self,
        renderer: &mut Renderer,
    ) -> (crate::VertexBuffers, crate::UniformBuffers, crate::Shaders) {
        let vertex_buffer = self.update_vertex_buffer_and_return(renderer);
        let uniform_buffer = self.update_uniform_buffer_and_return(renderer);
        let shader = self.update_shader_and_return(renderer);
        self.changed = false;
        (vertex_buffer, uniform_buffer, shader)
    }

    fn update_vertex_buffer_inner(&mut self, renderer: &mut Renderer) -> crate::VertexBuffers {
        renderer.build_vertex_buffer(&self.vertices, &self.indices)
    }
    /// Update and apply changes done to the vertex buffer
    pub fn update_vertex_buffer(&mut self, renderer: &mut Renderer) {
        let updated_buffer = self.update_vertex_buffer_inner(renderer);
        self.pipeline.vertex_buffer = PipelineData::Data(updated_buffer);
    }
    /// Returns the buffer with ownership
    pub fn update_vertex_buffer_and_return(
        &mut self,
        renderer: &mut Renderer,
    ) -> crate::VertexBuffers {
        let updated_buffer = self.update_vertex_buffer_inner(renderer);
        self.pipeline.vertex_buffer = PipelineData::Data(crate::VertexBuffers {
            vertex_buffer: updated_buffer.vertex_buffer.clone(),
            index_buffer: updated_buffer.index_buffer.clone(),
            length: updated_buffer.length,
        });

        updated_buffer
    }

    fn update_shader_inner(&mut self, renderer: &mut Renderer) -> crate::Shaders {
        renderer.build_shader(
            self.name.as_ref(),
            self.shader_builder.shader.clone(),
            Some(&self.uniform_layout),
            self.shader_settings,
        )
    }
    /// Update and apply changes done to the shader
    pub fn update_shader(&mut self, renderer: &mut Renderer) {
        let updated_shader = self.update_shader_inner(renderer);
        self.pipeline.shader = PipelineData::Data(updated_shader);
    }
    /// Returns the buffer with ownership
    pub fn update_shader_and_return(&mut self, renderer: &mut Renderer) -> crate::Shaders {
        let updated_shader = self.update_shader_inner(renderer);
        self.pipeline.shader = PipelineData::Data(updated_shader.clone());

        updated_shader
    }

    fn update_uniform_buffer_inner(
        &mut self,
        renderer: &mut Renderer,
    ) -> (crate::UniformBuffers, wgpu::BindGroupLayout) {
        self.uniform_buffers[0] = renderer.build_uniform_buffer_part(
            "Transformation Matrix",
            self.translation_matrix
                * Matrix4::from_quat(self.rotation_quaternion)
                * self.scale_matrix,
        );
        self.uniform_buffers[1] = renderer.build_uniform_buffer_part("Color", self.color);

        let updated_buffer = renderer.build_uniform_buffer(&self.uniform_buffers);

        updated_buffer
    }
    /// Update and apply changes done to the uniform buffer
    pub fn update_uniform_buffer(&mut self, renderer: &mut Renderer) {
        let updated_buffer = self.update_uniform_buffer_inner(renderer);

        self.pipeline.uniform = PipelineData::Data(Some(updated_buffer.0));
        self.uniform_layout = updated_buffer.1;
    }
    /// Update and apply changes done to the uniform buffer and returns it
    pub fn update_uniform_buffer_and_return(
        &mut self,
        renderer: &mut Renderer,
    ) -> crate::UniformBuffers {
        let updated_buffer = self.update_uniform_buffer_inner(renderer);
        let updated_buffer2 = updated_buffer.0.clone();

        self.pipeline.uniform = PipelineData::Data(Some(updated_buffer.0));
        self.uniform_layout = updated_buffer.1;

        updated_buffer2
    }

    fn update_instance_buffer_inner(&mut self, renderer: &mut Renderer) -> wgpu::Buffer {
        let instance_data = self
            .instances
            .iter()
            .map(Instance::build)
            .collect::<Vec<_>>();
        renderer.build_instance(instance_data)
    }
    /// Updates the instance buffer
    pub fn update_instance_buffer(&mut self, renderer: &mut Renderer) {
        let instance_buffer = self.update_instance_buffer_inner(renderer);
        self.instance_buffer = instance_buffer;
    }
    /// Returns the buffer with ownership
    pub fn update_instance_buffer_and_return(&mut self, renderer: &mut Renderer) -> wgpu::Buffer {
        let instance_buffer = self.update_instance_buffer_inner(renderer);
        self.instance_buffer = instance_buffer.clone();

        instance_buffer
    }
}

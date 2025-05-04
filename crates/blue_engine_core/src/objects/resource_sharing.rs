use super::{Instance, Object};
use crate::PipelineData;

impl Object {
    /// References another object's vertices
    pub fn reference_vertices(&mut self, object_id: impl AsRef<str>) -> &mut Self {
        self.pipeline.vertex_buffer = PipelineData::Copy(object_id.as_ref().into());
        self
    }

    /// References another object's shader
    pub fn reference_shader(&mut self, object_id: impl AsRef<str>) -> &mut Self {
        self.pipeline.shader = PipelineData::Copy(object_id.as_ref().into());
        self
    }

    /// References another object's texture
    pub fn reference_texture(&mut self, object_id: impl AsRef<str>) -> &mut Self {
        self.pipeline.texture = PipelineData::Copy(object_id.as_ref().into());
        self
    }

    /// References another object's uniform buffer
    pub fn reference_uniform_buffer(&mut self, object_id: impl AsRef<str>) -> &mut Self {
        self.pipeline.uniform = PipelineData::Copy(object_id.as_ref().into());
        self
    }

    // ============================= Instances =============================
    /// Add an instance to the object
    pub fn add_instance(&mut self, instance: Instance) -> &mut Self {
        self.instances.push(instance);
        self.changed = true;
        self
    }
}

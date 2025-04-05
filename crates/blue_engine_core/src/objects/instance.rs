use crate::{Matrix4, Quaternion, Vector3};

/// Instance buffer data that is sent to GPU
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    /// The transformation matrix of the instance
    pub model: Matrix4,
}

/// Instance buffer data storage
#[derive(Debug, Clone, Copy)]
pub struct Instance {
    /// The position of the instance
    pub position: Vector3,
    /// The rotation of the instance
    pub rotation: Vector3,
    /// The scale of the instance
    pub scale: Vector3,
}

impl Instance {
    /// Creates a new instance
    #[deprecated]
    pub fn new(
        position: impl Into<Vector3>,
        rotation: impl Into<Vector3>,
        scale: impl Into<Vector3>,
    ) -> Self {
        Self {
            position: position.into(),
            rotation: rotation.into(),
            scale: scale.into(),
        }
    }

    /// Gathers all information and builds a Raw Instance to be sent to GPU
    pub fn build(&self) -> InstanceRaw {
        let position_matrix = Matrix4::IDENTITY * Matrix4::from_translation(self.position);
        let rotation_matrix = Matrix4::from_quat(
            Quaternion::from_rotation_x(self.rotation.x)
                * Quaternion::from_rotation_y(self.rotation.y)
                * Quaternion::from_rotation_z(self.rotation.z),
        );
        let scale_matrix = Matrix4::IDENTITY * Matrix4::from_scale(self.scale);
        InstanceRaw {
            model: position_matrix * rotation_matrix * scale_matrix,
        }
    }

    /// Sets the position
    pub fn set_position(&mut self, position: impl Into<Vector3>) {
        self.position = position.into();
    }

    /// Sets the rotation
    pub fn set_rotation(&mut self, rotation: impl Into<Vector3>) {
        self.rotation = rotation.into();
    }

    /// Sets the scale
    pub fn set_scale(&mut self, scale: impl Into<Vector3>) {
        self.scale = scale.into();
    }
}
impl Default for Instance {
    fn default() -> Self {
        Self {
            position: Vector3::ZERO,
            rotation: Vector3::ZERO,
            scale: Vector3::ONE,
        }
    }
}
impl InstanceRaw {
    /// Instance's layout description
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in the shader.
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

use crate::definitions::{OPENGL_TO_WGPU_MATRIX, Renderer, uniform_type::Matrix};
use anyhow::*;
/// Container for the camera feature. The settings here are needed for
/// algebra equations needed for camera vision and movement. Please leave it to the renderer to handle
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// The position of the camera in 3D space
    pub eye: cgmath::Point3<f32>,
    /// The target at which the camera should be looking
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    /// The field of view of the camera
    pub fovy: f32,
    /// The closest view of camera
    pub znear: f32,
    /// The furthest view of camera
    pub zfar: f32,
}


impl Camera {
    pub fn new(renderer: &mut Renderer) -> Result<Self> {
        Ok(Self {
            eye: (0.0, 0.0, 1.4).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: renderer.sc_desc.width as f32 / renderer.sc_desc.height as f32,
            fovy: 60.0,
            znear: 0.1,
            zfar: 100.0,
        })
    }

    pub fn build_view_projection_matrix(&self) -> Result<cgmath::Matrix4<f32>> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        Ok(OPENGL_TO_WGPU_MATRIX * proj * view)
    }

    pub fn new_camera_uniform_buffer(&self) -> Result<Matrix> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        let camera_matrix = OPENGL_TO_WGPU_MATRIX * proj * view;
        Ok(Matrix {
            data: camera_matrix.into(),
        })
    }

    pub fn update_view_proj(&mut self, view_proj: &mut Matrix) {
        view_proj.data = self
            .build_view_projection_matrix()
            .expect("Couldn't build view projection matrix")
            .into();
    }
}

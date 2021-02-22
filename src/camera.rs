use crate::definitions::{uniform_type::Matrix, Camera, Renderer};
use anyhow::*;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0
);

impl Camera {
    pub fn new(renderer: &mut Renderer) -> Result<Self> {
        Ok(Self {
            eye: (0.0, 0.0, 2.5).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: renderer.sc_desc.width as f32 / renderer.sc_desc.height as f32,
            fovy: 45.0,
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

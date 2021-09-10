/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::definitions::{uniform_type::Matrix, Renderer};
use anyhow::*;
/// Container for the camera feature. The settings here are needed for
/// algebra equations needed for camera vision and movement. Please leave it to the renderer to handle
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// The position of the camera in 3D space
    pub eye: glm::Vector3<f32>,
    /// The target at which the camera should be looking
    pub target: glm::Vector3<f32>,
    pub up: glm::Vector3<f32>,
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
            eye: glm::vec3(0.0, 0.0, 1.4),
            target: glm::vec3(0.0, 0.0, 0.0).into(),
            up: glm::vec3(0.0, 1.0, 0.0),
            aspect: renderer.sc_desc.width as f32 / renderer.sc_desc.height as f32,
            fovy: 75.0,
            znear: 0.1,
            zfar: 100.0,
        })
    }

    pub fn build_view_projection_matrix(&self) -> Result<glm::Matrix4<f32>> {
        let view = glm::ext::look_at_rh(self.eye, self.target, self.up);
        let proj = glm::ext::perspective::<f32>(self.fovy, self.aspect, self.znear, self.zfar);

        Ok(proj * view)
    }

    pub fn new_camera_uniform_buffer(&self) -> Result<Matrix> {
        let view = glm::ext::look_at_rh(self.eye, self.target, self.up);
        let proj = glm::ext::perspective::<f32>(self.fovy, self.aspect, self.znear, self.zfar);
        let camera_matrix = proj * view;
        Ok(Matrix::from_glm(camera_matrix))
    }

    pub fn update_view_proj(&mut self, view_proj: &mut Matrix) {
        view_proj.data = Matrix::from_glm(
            self.build_view_projection_matrix()
                .expect("Couldn't build view projection matrix"),
        ).data;
    }
}

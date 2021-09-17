/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::definitions::{uniform_type::Matrix, Camera, Renderer};
use anyhow::Result;

impl Camera {
    pub fn new(renderer: &Renderer) -> Result<Self> {
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
        )
        .data;
    }
}

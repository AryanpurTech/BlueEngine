/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::{
    header::{
        uniform_type::{self, Matrix},
        Camera, Renderer, UniformBuffer,
    },
    utils::default_resources::DEFAULT_COLOR,
};
use anyhow::Result;

use super::default_resources::DEFAULT_MATRIX_4;

impl Camera {
    pub fn new(renderer: &Renderer) -> Result<Self> {
        let mut camera = Self {
            eye: glm::vec3(0.0, 0.0, 0.0),
            target: glm::vec3(0.0, 0.0, 0.0).into(),
            up: glm::vec3(0.0, 1.0, 0.0),
            aspect: renderer.config.width as f32 / renderer.config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            view_data: DEFAULT_MATRIX_4,
            changed: true,
        };
        camera.build_view_projection_matrix()?;

        Ok(camera)
    }

    pub fn build_view_projection_matrix(&mut self) -> Result<()> {
        let view = glm::ext::look_at_rh(self.eye, self.target, self.up);
        let proj = glm::ext::perspective::<f32>(self.fovy, self.aspect, self.znear, self.zfar);
        self.view_data = proj * view;
        self.changed = true;

        Ok(())
    }

    pub fn camera_uniform_buffer(&self) -> Result<Matrix> {
        let view = glm::ext::look_at_rh(self.eye, self.target, self.up);
        let proj = glm::ext::perspective::<f32>(self.fovy, self.aspect, self.znear, self.zfar);
        let camera_matrix = proj * view;
        Ok(Matrix::from_glm(camera_matrix))
    }

    pub fn set_eye(&mut self, new_eye: [f32; 3]) -> Result<()> {
        self.eye = glm::vec3(new_eye[0], new_eye[1], new_eye[2]);
        self.build_view_projection_matrix()?;

        Ok(())
    }

    pub fn set_target(&mut self, new_target: [f32; 3]) -> Result<()> {
        self.target = glm::vec3(new_target[0], new_target[1], new_target[2]);
        self.build_view_projection_matrix()?;

        Ok(())
    }

    pub fn set_up(&mut self, new_up: [f32; 3]) -> Result<()> {
        self.up = glm::vec3(new_up[0], new_up[1], new_up[2]);
        self.build_view_projection_matrix()?;

        Ok(())
    }

    pub fn update_view_projection(&mut self, renderer: &mut Renderer) -> Result<()> {
        if self.changed {
            let updated_buffer = renderer
                .build_uniform_buffer(vec![
                    UniformBuffer::Matrix(
                        "Camera Uniform",
                        self.camera_uniform_buffer()
                            .expect("Couldn't build camera projection"),
                    ),
                    UniformBuffer::Array(
                        "Default Color",
                        uniform_type::Array {
                            data: DEFAULT_COLOR,
                        },
                    ),
                ])
                .expect("Couldn't update the camera uniform buffer")
                .0;
            let _ = std::mem::replace(&mut renderer.uniform_bind_group[0], updated_buffer);
            self.changed = false;
        }

        Ok(())
    }
}

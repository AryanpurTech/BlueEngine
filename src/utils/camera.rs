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
    /// Creates a new camera. this should've been automatically done at the time of creating an engine
    pub fn new(renderer: &Renderer) -> Result<Self> {
        let mut camera = Self {
            eye: glm::vec3(0.0, 0.0, 1.0),
            target: glm::vec3(0.0, 0.0, 0.0).into(),
            up: glm::vec3(0.0, 1.0, 0.0),
            aspect: renderer.config.width as f32 / renderer.config.height as f32,
            fov: 45.0,
            near: 0.1,
            far: 100.0,
            view_data: DEFAULT_MATRIX_4,
            changed: true,
        };
        camera.build_view_projection_matrix()?;

        Ok(camera)
    }

    /// Updates the view uniform matrix that decides how camera works
    pub fn build_view_projection_matrix(&mut self) -> Result<()> {
        let view = glm::ext::look_at_rh(self.eye, self.target, self.up);
        let proj = glm::ext::perspective::<f32>(self.fov, self.aspect, self.near, self.far);
        self.view_data = proj * view;
        self.changed = true;

        Ok(())
    }

    /// Returns a matrix uniform buffer from camera data that can be sent to GPU
    pub fn camera_uniform_buffer(&self) -> Result<Matrix> {
        Ok(Matrix::from_glm(self.view_data))
    }

    /// Sets the eye of camera
    pub fn set_eye(&mut self, new_eye: [f32; 3]) -> Result<()> {
        self.eye = glm::vec3(new_eye[0], new_eye[1], new_eye[2]);
        self.build_view_projection_matrix()?;

        Ok(())
    }

    /// Sets the target of camera
    pub fn set_target(&mut self, new_target: [f32; 3]) -> Result<()> {
        self.target = glm::vec3(new_target[0], new_target[1], new_target[2]);
        self.build_view_projection_matrix()?;

        Ok(())
    }

    /// Sets the up of camera
    pub fn set_up(&mut self, new_up: [f32; 3]) -> Result<()> {
        self.up = glm::vec3(new_up[0], new_up[1], new_up[2]);
        self.build_view_projection_matrix()?;

        Ok(())
    }

    /// Sets the field of view of camera
    pub fn set_fov(&mut self, new_fov: f32) -> Result<()> {
        self.fov = new_fov;
        self.build_view_projection_matrix()?;

        Ok(())
    }

    /// Sets how far camera can look
    pub fn set_far(&mut self, new_far: f32) -> Result<()> {
        self.far = new_far;
        self.build_view_projection_matrix()?;

        Ok(())
    }

    /// Sets how near the camera can look
    pub fn set_near(&mut self, new_near: f32) -> Result<()> {
        self.near = new_near;
        self.build_view_projection_matrix()?;

        Ok(())
    }

    /// Sets the aspect ratio of the camera
    pub fn set_aspect(&mut self, new_aspect: f32) -> Result<()> {
        self.aspect = new_aspect;
        self.build_view_projection_matrix()?;

        Ok(())
    }

    /// This builds a uniform buffer data from camera view data that is sent to the GPU in next frame
    pub fn update_view_projection(&mut self, renderer: &mut Renderer) -> Result<()> {
        if self.changed {
            let updated_buffer = renderer
                .build_uniform_buffer(vec![
                    UniformBuffer::Matrix(
                        "Camera Uniform",
                        self.camera_uniform_buffer()
                            .expect("Couldn't build camera projection"),
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

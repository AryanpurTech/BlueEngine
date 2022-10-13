/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::{uniform_type::Matrix, Camera, Renderer};
use anyhow::Result;
use winit::dpi::PhysicalSize;

use super::default_resources::DEFAULT_MATRIX_4;

impl Camera {
    /// Creates a new camera. this should've been automatically done at the time of creating an engine
    pub fn new(window_size: PhysicalSize<u32>, renderer: &mut Renderer) -> Result<Self> {
        let camera_uniform = renderer.build_uniform_buffer(&vec![
            renderer.build_uniform_buffer_part("Camera Uniform", DEFAULT_MATRIX_4)
        ])?;

        let mut camera = Self {
            position: nalgebra_glm::vec3(0.0, 0.0, 3.0),
            target: nalgebra_glm::vec3(0.0, 0.0, -1.0).into(),
            up: nalgebra_glm::vec3(0.0, 1.0, 0.0),
            resolution: (window_size.width as f32, window_size.height as f32),
            fov: 70f32 * (std::f32::consts::PI / 180f32),
            near: 0.1,
            far: 100.0,
            view_data: DEFAULT_MATRIX_4.to_im(),
            changed: true,
            uniform_data: camera_uniform.0,
            add_position_and_target: false,
        };
        camera.build_view_projection_matrix()?;

        Ok(camera)
    }

    /// Updates the view uniform matrix that decides how camera works
    pub fn build_view_projection_matrix(&mut self) -> Result<()> {
        let view = nalgebra_glm::look_at_rh(
            &self.position,
            &if self.add_position_and_target {
                self.position + self.target
            } else {
                self.target
            },
            &self.up,
        );
        let proj = nalgebra_glm::perspective(
            self.resolution.0 / self.resolution.1,
            self.fov,
            self.near,
            self.far,
        );
        self.view_data = proj * view;
        self.changed = true;

        Ok(())
    }

    /// Updates the view uniform matrix that decides how camera works
    pub fn build_view_orthographic_matrix(&mut self) -> Result<()> {
        let view = nalgebra_glm::look_at_rh(
            &self.position,
            &if self.add_position_and_target {
                self.position + self.target
            } else {
                self.target
            },
            &self.up,
        );
        let ortho = nalgebra_glm::ortho(
            0f32,
            self.resolution.0,
            0f32,
            self.resolution.1,
            self.near,
            self.far,
        );
        self.view_data = ortho * view;
        self.changed = true;

        Ok(())
    }

    /// Returns a matrix uniform buffer from camera data that can be sent to GPU
    pub fn camera_uniform_buffer(&self) -> Result<Matrix> {
        Ok(Matrix::from_im(self.view_data))
    }

    /// Sets the position of camera
    pub fn set_position(&mut self, x: f32, y: f32, z: f32) -> Result<()> {
        self.position = nalgebra_glm::vec3(x, y, z);
        self.build_view_projection_matrix()?;

        Ok(())
    }

    /// Sets the target of camera
    pub fn set_target(&mut self, x: f32, y: f32, z: f32) -> Result<()> {
        self.target = nalgebra_glm::vec3(x, y, z);
        self.build_view_projection_matrix()?;

        Ok(())
    }

    /// Sets the up of camera
    pub fn set_up(&mut self, x: f32, y: f32, z: f32) -> Result<()> {
        self.up = nalgebra_glm::vec3(x, y, z);
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
    pub fn set_resolution(&mut self, window_size: PhysicalSize<u32>) -> Result<()> {
        self.resolution = (window_size.width as f32, window_size.height as f32);
        self.build_view_projection_matrix()?;

        Ok(())
    }

    /// Enables adding position and target for the view target
    pub fn add_position_and_target(&mut self, enable: bool) {
        self.add_position_and_target = enable;
    }

    /// This builds a uniform buffer data from camera view data that is sent to the GPU in next frame
    pub fn update_view_projection(&mut self, renderer: &mut Renderer) -> Result<()> {
        if self.changed {
            let updated_buffer = renderer
                .build_uniform_buffer(&vec![renderer.build_uniform_buffer_part(
                    "Camera Uniform",
                    self.camera_uniform_buffer()
                        .expect("Couldn't build camera projection"),
                )])
                .expect("Couldn't update the camera uniform buffer")
                .0;
            self.uniform_data = updated_buffer;
            self.changed = false;
        }

        Ok(())
    }
}

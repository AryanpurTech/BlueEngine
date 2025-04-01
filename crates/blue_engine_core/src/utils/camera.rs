/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use super::default_resources::OPENGL_TO_WGPU_MATRIX;
use crate::{
    Matrix4, UniformBuffers, Vector2,
    prelude::{Renderer, Vector3},
};
use winit::dpi::PhysicalSize;

/// Container for the projection used by the camera
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Projection {
    /// Perspective projection
    ///
    /// This is the default project used by the video games and majority of graphics
    Perspective {
        /// The field of view
        fov: f32,
    },
    /// Orthographic projection
    ///
    /// This projection gives you a 2D view of the scene
    Orthographic {
        /// The size of the view
        zoom: f32,
    },
}

/// Container for the camera feature. The settings here are needed for
/// algebra equations needed for camera vision and movement. Please leave it to the renderer to handle
#[derive(Debug)]
pub struct Camera {
    /// The position of the camera in 3D space
    pub position: Vector3,
    /// The target at which the camera should be looking
    pub target: Vector3,
    /// The up vector of the camera. This defines the elevation of the camera
    pub up: Vector3,
    /// The resolution of the camera view
    pub resolution: Vector2,
    /// The projection of the camera
    pub projection: Projection,
    /// The closest view of camera
    pub near: f32,
    /// The furthest view of camera
    pub far: f32,
    /// The final data that will be sent to GPU
    pub view_data: Matrix4,
    // For checking and rebuilding it's uniform buffer
    pub(crate) changed: bool,
    /// The uniform data of the camera to be sent to the gpu
    pub uniform_data: UniformBuffers,
}
unsafe impl Send for Camera {}
unsafe impl Sync for Camera {}

/// Container for Cameras
///
/// This allows for different objects have a different camera perspective.
#[derive(Debug)]
pub struct CameraContainer {
    /// The list of cameras
    // Arc<str> is used instead of String for performance
    pub cameras: std::collections::HashMap<std::sync::Arc<str>, Camera>,
}
crate::macros::impl_deref_field!(
    CameraContainer,
    std::collections::HashMap<std::sync::Arc<str>, Camera>,
    cameras
);

impl Camera {
    /// Creates a new camera. this should've been automatically done at the time of creating an engine
    pub fn new(window_size: PhysicalSize<u32>, renderer: &mut Renderer) -> Self {
        let camera_uniform = renderer.build_uniform_buffer(&[
            renderer.build_uniform_buffer_part("Camera Uniform", crate::Matrix4::IDENTITY)
        ]);

        let mut camera = Self {
            position: Vector3::new(0.0, 0.0, 3.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            resolution: Vector2::new(window_size.width as f32, window_size.height as f32),
            projection: crate::Projection::Perspective {
                fov: 70f32 * (std::f32::consts::PI / 180f32),
            },
            near: 0.1,
            far: 100.0,
            view_data: Matrix4::IDENTITY,
            changed: true,
            uniform_data: camera_uniform.0,
        };
        camera.build_view_projection_matrix();

        camera
    }

    /// Builds a view matrix for camera projection
    pub fn build_view_matrix(&self) -> Matrix4 {
        Matrix4::look_at_rh(self.position, self.target, self.up)
    }

    /// Builds a projection matrix for camera
    pub fn build_projection_matrix(&self) -> Matrix4 {
        let aspect = self.resolution.x / self.resolution.y;

        match self.projection {
            crate::Projection::Perspective { fov } => {
                Matrix4::perspective_rh(fov, aspect, self.near, self.far)
            }
            crate::Projection::Orthographic { zoom } => {
                let width = zoom;
                let height = width / aspect;

                let left = width * -0.5;
                let right = width * 0.5;
                let bottom = height * -0.5;
                let top = height * 0.5;

                Matrix4::orthographic_rh(left, right, bottom, top, self.near, self.far)
            }
        }
    }

    /// Updates the view uniform matrix that decides how camera works
    pub fn build_view_projection_matrix(&mut self) {
        let view = self.build_view_matrix();
        let proj = self.build_projection_matrix();
        self.view_data = OPENGL_TO_WGPU_MATRIX * proj * view;
        self.changed = true;
    }

    /// Updates the view uniform matrix that decides how camera works
    pub fn build_view_orthographic_matrix(&mut self) {
        let view = self.build_view_matrix();
        let ortho = Matrix4::orthographic_rh(
            0f32,
            self.resolution.x,
            0f32,
            self.resolution.y,
            self.near,
            self.far,
        );
        self.view_data = ortho * view;
        self.changed = true;
    }

    /// This builds a uniform buffer data from camera view data that is sent to the GPU in next frame
    pub fn update_view_projection(&mut self, renderer: &mut Renderer) {
        if self.changed {
            let updated_buffer = renderer
                .build_uniform_buffer(&[renderer
                    .build_uniform_buffer_part("Camera Uniform", self.camera_uniform_buffer())])
                .0;
            self.uniform_data = updated_buffer;
            self.changed = false;
        }
    }

    /// This builds a uniform buffer data from camera view data that is sent to the GPU in next frame, and returns the bindgroup
    pub fn update_view_projection_and_return(
        &mut self,
        renderer: &mut Renderer,
    ) -> crate::UniformBuffers {
        let updated_buffer = renderer
            .build_uniform_buffer(&[
                renderer.build_uniform_buffer_part("Camera Uniform", self.camera_uniform_buffer())
            ])
            .0;

        updated_buffer
    }

    /// Returns a matrix uniform buffer from camera data that can be sent to GPU
    pub fn camera_uniform_buffer(&self) -> Matrix4 {
        self.view_data
    }

    /// Sets the position of camera
    pub fn set_position(&mut self, new_pos: impl Into<Vector3>) {
        self.position = new_pos.into();
        self.build_view_projection_matrix();
    }

    /// Sets the target of camera
    pub fn set_target(&mut self, target_pos: impl Into<Vector3>) {
        self.target = target_pos.into();
        self.build_view_projection_matrix();
    }

    /// Sets the up of camera
    pub fn set_up(&mut self, pos: impl Into<Vector3>) {
        self.up = pos.into();
        self.build_view_projection_matrix();
    }

    /// Sets how far camera can look
    pub fn set_far(&mut self, new_far: f32) {
        self.far = new_far;
        self.build_view_projection_matrix();
    }

    /// Sets how near the camera can look
    pub fn set_near(&mut self, new_near: f32) {
        self.near = new_near;
        self.build_view_projection_matrix();
    }

    /// Sets the aspect ratio of the camera
    pub fn set_resolution(&mut self, window_size: PhysicalSize<u32>) {
        self.resolution = Vector2::new(window_size.width as f32, window_size.height as f32);
        self.build_view_projection_matrix();
    }

    /// Sets the projection of the camera
    pub fn set_projection(&mut self, projection: Projection) {
        self.projection = projection;
        self.build_view_projection_matrix();
    }
}

impl CameraContainer {
    /// Creates new CameraContainer with one main camera
    pub fn new(window_size: PhysicalSize<u32>, renderer: &mut Renderer) -> Self {
        let mut cameras = std::collections::HashMap::new();
        let main_camera = Camera::new(window_size, renderer);
        cameras.insert("main".into(), main_camera);

        CameraContainer { cameras }
    }

    /// Updates the view uniform matrix that decides how camera works
    pub fn build_view_projection_matrix(&mut self) {
        if let Some(main_camera) = self.cameras.get_mut("main") {
            main_camera.build_view_projection_matrix();
        }
    }
    /// Updates the view uniform matrix that decides how camera works
    pub fn build_view_orthographic_matrix(&mut self) {
        if let Some(main_camera) = self.cameras.get_mut("main") {
            main_camera.build_view_orthographic_matrix();
        }
    }
    /// Returns a matrix uniform buffer from camera data that can be sent to GPU
    pub fn camera_uniform_buffer(&self) -> Option<Matrix4> {
        if let Some(main_camera) = self.cameras.get("main") {
            Some(main_camera.view_data)
        } else {
            None
        }
    }
    /// Sets the position of camera
    pub fn set_position(&mut self, new_pos: impl Into<Vector3>) {
        if let Some(main_camera) = self.cameras.get_mut("main") {
            main_camera.set_position(new_pos.into());
        }
    }
    /// Sets the target of camera
    pub fn set_target(&mut self, pos: impl Into<Vector3>) {
        if let Some(main_camera) = self.cameras.get_mut("main") {
            main_camera.set_target(pos.into());
        }
    }
    /// Sets the up of camera
    pub fn set_up(&mut self, pos: impl Into<Vector3>) {
        if let Some(main_camera) = self.cameras.get_mut("main") {
            main_camera.set_up(pos.into());
        }
    }
    /// Sets how far camera can look
    pub fn set_far(&mut self, new_far: f32) {
        if let Some(main_camera) = self.cameras.get_mut("main") {
            main_camera.set_far(new_far);
        }
    }
    /// Sets how near the camera can look
    pub fn set_near(&mut self, new_near: f32) {
        if let Some(main_camera) = self.cameras.get_mut("main") {
            main_camera.set_near(new_near);
        }
    }
    /// Sets the aspect ratio of the camera
    pub fn set_resolution(&mut self, window_size: PhysicalSize<u32>) {
        if let Some(main_camera) = self.cameras.get_mut("main") {
            main_camera.set_resolution(window_size);
        }
    }
    /// Sets the projection of the camera
    pub fn set_projection(&mut self, projection: Projection) {
        if let Some(main_camera) = self.cameras.get_mut("main") {
            main_camera.set_projection(projection);
        }
    }
    /// This builds a uniform buffer data from camera view data that is sent to the GPU in next frame
    pub fn update_view_projection(&mut self, renderer: &mut Renderer) {
        if let Some(main_camera) = self.cameras.get_mut("main") {
            main_camera.update_view_projection(renderer);
        }
    }
    /// This builds a uniform buffer data from camera view data that is sent to the GPU in next frame, and returns the bindgroup
    pub fn update_view_projection_and_return(
        &mut self,
        renderer: &mut Renderer,
    ) -> Option<crate::UniformBuffers> {
        match self.cameras.get_mut("main") {
            Some(main_camera) => Some(main_camera.update_view_projection_and_return(renderer)),
            None => None,
        }
    }
    /// Builds a view matrix for camera projection
    pub fn build_view_matrix(&self) -> Option<Matrix4> {
        if let Some(main_camera) = self.cameras.get("main") {
            Some(main_camera.build_view_matrix())
        } else {
            None
        }
    }
    /// Builds a projection matrix for camera
    pub fn build_projection_matrix(&self) -> Option<Matrix4> {
        if let Some(main_camera) = self.cameras.get("main") {
            Some(main_camera.build_projection_matrix())
        } else {
            None
        }
    }
}

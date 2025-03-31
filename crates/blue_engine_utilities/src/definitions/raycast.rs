#![cfg(feature = "physics")]

use blue_engine::{Camera, imports::glm};

pub struct Raycast {
    current_ray: glm::Vec3,
    projection_matrix: glm::Mat4,
    view_matrix: glm::Mat4,
    mouse_x_y: (f32, f32),
}
impl Raycast {
    pub fn new(camera: &Camera) -> Self {
        let view_matrix = camera.build_view_matrix();
        let projection_matrix = camera.build_projection_matrix();
        Self {
            projection_matrix,
            view_matrix,
            current_ray: glm::Vec3::new(0f32, 0f32, 0f32),
            mouse_x_y: (0.0, 0.0),
        }
    }

    pub fn get_current_ray(&self) -> glm::Vec3 {
        self.current_ray
    }

    pub fn update(
        &mut self,
        camera: &Camera,
        input: &blue_engine::InputHelper,
        window_size: &blue_engine::PhysicalSize<u32>,
    ) {
        self.mouse_x_y = input.mouse_diff();

        self.view_matrix = camera.build_view_matrix();
        self.current_ray = self.calculate_mouse_ray(window_size);
    }

    pub fn calculate_mouse_ray(&self, window_size: &blue_engine::PhysicalSize<u32>) -> glm::Vec3 {
        let normalized_coordinates = self.get_normalized_device_coordinates(window_size);
        let clip_coordinates = glm::vec3(normalized_coordinates.x, normalized_coordinates.y, -1f32);
        let eye_coordinates = self.to_eye_coordinates(clip_coordinates);
        self.to_world_coordinates(eye_coordinates)
        //let ray = self.projection_matrix * self.view_matrix * clip_coordinates;
    }

    pub fn to_world_coordinates(&self, eye_coordinates: glm::Vec3) -> glm::Vec3 {
        let inverted_view = glm::inverse(&self.view_matrix);
        let ray_world = inverted_view.transform_vector(&eye_coordinates);
        let mouse_ray = glm::Vec3::new(ray_world.x, ray_world.y, ray_world.z);
        mouse_ray.normalize()
    }

    pub fn to_eye_coordinates(&self, clip_coordinates: glm::Vec3) -> glm::Vec3 {
        let inverted_projection = glm::inverse(&self.projection_matrix);
        let eye_coordinates = inverted_projection.transform_vector(&clip_coordinates);
        glm::Vec3::new(eye_coordinates.x, eye_coordinates.y, -1f32)
    }

    pub fn get_normalized_device_coordinates(
        &self,
        window_size: &blue_engine::PhysicalSize<u32>,
    ) -> glm::Vec2 {
        let x = (self.mouse_x_y.0 * 2f32) / window_size.width as f32 - 1f32;
        let y = -((self.mouse_x_y.1 * 2f32) / window_size.height as f32 - 1f32);

        glm::vec2(x, y)
    }

    pub fn ray_intersects_bounding_box(
        &self,
        bounding_box: (glm::Vec3, glm::Vec3),
        _max_length: f32,
        camera: &Camera,
    ) -> Option<glm::Vec3> {
        let (min_corner, max_corner) = bounding_box;

        // calculate the inverse of the ray direction
        let inv_dir = glm::vec3(
            1f32 / self.current_ray.x,
            1f32 / self.current_ray.y,
            1f32 / self.current_ray.z,
        );

        let camera_pos =
            blue_engine::glm::vec3(camera.position.x, camera.position.y, camera.position.z);
        let min_corner = min_corner - camera_pos;
        let max_corner = max_corner - camera_pos;

        // calculate the minimum and maximum intersection distances for each axis
        let tmin = glm::vec3(
            min_corner.x * self.current_ray.x,
            min_corner.y * self.current_ray.y,
            min_corner.z * self.current_ray.z,
        );
        let tmax = glm::vec3(
            max_corner.x * inv_dir.x,
            max_corner.y * inv_dir.y,
            max_corner.z * inv_dir.z,
        );

        //println!("tmin: {:?} | tmax: {:?}", tmin, tmax);

        // Check if the ray intersects the bounding box
        let _t_enter = tmin.max();
        let _t_exit = tmax.min();

        /*println!(
            "{:?} | t_enter: {} | t_exit: {}",
            t_enter <= t_exit && t_exit >= 0.0,
            t_enter,
            t_exit
        ); */

        None
    }
}

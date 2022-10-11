use winit::window::Window;

use crate::Camera;

pub struct FlyCamera {
    pub camera_right: nalgebra_glm::Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub last_x: f64,
    pub last_y: f64,

    pub is_focus: bool,
    pub camera_speed: f32,
    pub camera_sensitivity: f32,
    pub timer: std::time::Instant,
    pub last_frame: f32,
}

impl FlyCamera {
    pub fn new(camera: &mut Camera) -> Self {
        camera.add_position_and_target(true);

        Self {
            camera_right: nalgebra_glm::cross(&camera.target, &camera.up).normalize(),
            yaw: -90f32,
            pitch: 0f32,
            last_x: 0f64,
            last_y: 0f64,

            is_focus: false,
            camera_speed: 0.5f32,
            camera_sensitivity: 0.10f32,
            timer: std::time::Instant::now(),
            last_frame: 0f32,
        }
    }

    fn update_vertices(camera: &mut Camera) -> nalgebra_glm::Vec3 {
        let camera_right = nalgebra_glm::cross(&camera.target, &camera.up).normalize();

        /*let up = nalgebra_glm::cross(&camera_right, &camera.target)
            .normalize()
            .data;
        let up = up.as_slice();
        camera.set_up(up[0], up[1], up[2]).unwrap(); */

        camera_right
    }
}

impl crate::EnginePlugin for FlyCamera {
    fn update_events(
        &mut self,
        _renderer: &mut crate::Renderer,
        window: &Window,
        _objects: &mut std::collections::HashMap<&'static str, crate::Object>,
        events: &winit::event::Event<()>,
        input: &winit_input_helper::WinitInputHelper,
        camera: &mut Camera,
    ) {
        // =========== MOVEMENT ============ //
        let current_frame = self.timer.elapsed().as_secs_f32();
        let delta = current_frame - self.last_frame;
        self.last_frame = current_frame;
        let mut camera_speed = self.camera_speed * delta;

        // ============ Window Focus ============= //
        if input.mouse_pressed(0) {
            if !self.is_focus {
                window
                    .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                    .expect("Couldn't grab the cursor");
                window.set_cursor_visible(false);
                self.is_focus = true;
            }
        }

        if self.is_focus {
            match events {
                crate::Event::DeviceEvent { event, .. } => match event {
                    crate::DeviceEvent::MouseMotion { delta: (x, y) } => {
                        let mut xoffset = *x as f32;
                        let mut yoffset = *y as f32;

                        xoffset *= self.camera_sensitivity;
                        yoffset *= self.camera_sensitivity;

                        self.yaw += xoffset;
                        self.pitch += yoffset;

                        if self.pitch > 89f32 {
                            self.pitch = 89f32;
                        }
                        if self.pitch < -89f32 {
                            self.pitch = -89f32;
                        }

                        let direction = nalgebra_glm::vec3(
                            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
                            (self.pitch * -1f32).to_radians().sin(),
                            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
                        );
                        let direction = direction.normalize().data;
                        let direction = direction.as_slice();
                        camera
                            .set_target(direction[0], direction[1], direction[2])
                            .unwrap();
                        self.camera_right = Self::update_vertices(camera);
                    }
                    _ => {}
                },
                _ => {}
            }

            if input.key_pressed(crate::VirtualKeyCode::Escape) {
                window
                    .set_cursor_grab(winit::window::CursorGrabMode::None)
                    .expect("Couldn't release the cursor");
                window.set_cursor_visible(true);
                self.is_focus = false;
            }

            // SHIFT
            if input.held_shift() {
                camera_speed *= 3f32;
            }

            // W
            if input.key_held(crate::VirtualKeyCode::W) {
                let result = (camera.position + (camera.target * camera_speed)).data;
                let result = result.as_slice();

                camera
                    .set_position(result[0], result[1], result[2])
                    .unwrap();
            }

            // S
            if input.key_held(crate::VirtualKeyCode::S) {
                let result = (camera.position - (camera.target * camera_speed)).data;
                let result = result.as_slice();

                camera
                    .set_position(result[0], result[1], result[2])
                    .unwrap();
            }
            // A
            if input.key_held(crate::VirtualKeyCode::A) {
                let result = (camera.position - (self.camera_right * camera_speed)).data;
                let result = result.as_slice();

                camera
                    .set_position(result[0], result[1], result[2])
                    .unwrap();
            }
            // D
            if input.key_held(crate::VirtualKeyCode::D) {
                let result = (camera.position + (self.camera_right * camera_speed)).data;
                let result = result.as_slice();

                camera
                    .set_position(result[0], result[1], result[2])
                    .unwrap();
            }
        }
    }

    fn update(
        &mut self,
        _renderer: &mut crate::Renderer,
        _window: &Window,
        _objects: &mut std::collections::HashMap<&'static str, crate::Object>,
        _camera: &mut Camera,
        _input: &crate::InputHelper,
        _encoder: &mut wgpu::CommandEncoder,
        _view: &wgpu::TextureView,
    ) {
    }
}

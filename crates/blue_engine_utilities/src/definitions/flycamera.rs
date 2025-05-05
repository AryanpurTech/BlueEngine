use blue_engine::{CameraContainer, DeviceEvent, ElementState, Vector3, winit};

pub struct FlyCamera {
    pub camera_right: Vector3,
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
    pub fn new(camera: &mut CameraContainer) -> Self {
        Self {
            camera_right: Self::update_vertices(camera),
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

    fn update_vertices(camera: &mut CameraContainer) -> Vector3 {
        let camera_right = camera
            .get("main")
            .unwrap()
            .target
            .cross(camera.get("main").unwrap().up)
            .normalize();

        /*let up = nalgebra_glm::cross(&camera_right, &camera.target)
            .normalize()
            .data;
        let up = up.as_slice();
        camera.set_up(up[0], up[1], up[2]).unwrap(); */
        Vector3::new(camera_right.x, camera_right.y, camera_right.z)
    }
}

impl blue_engine::Signal for FlyCamera {
    fn device_events(
        &mut self,
        engine: &mut blue_engine::Engine,
        events: &winit::event::DeviceEvent,
    ) {
        // =========== MOVEMENT ============ //
        let current_frame = self.timer.elapsed().as_secs_f32();
        let delta = current_frame - self.last_frame;
        self.last_frame = current_frame;
        let mut camera_speed = self.camera_speed * delta;

        // ============ Window Focus ============= //
        if engine
            .simple_input
            .mouse_pressed(blue_engine::MouseButton::Left)
            && !self.is_focus
        {
            engine
                .window
                .as_ref()
                .unwrap()
                .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                .expect("Couldn't grab the cursor");
            engine.window.as_ref().unwrap().set_cursor_visible(false);
            self.is_focus = true;
        }

        if self.is_focus {
            if let winit::event::DeviceEvent::MouseMotion { delta: (x, y) } = events {
                let mut xoffset = *x as f32;
                let mut yoffset = *y as f32;

                xoffset *= self.camera_sensitivity;
                yoffset *= self.camera_sensitivity;

                self.yaw += xoffset;
                self.pitch += yoffset;

                self.pitch = self.pitch.clamp(-89f32, 89f32);

                let direction = Vector3::new(
                    self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
                    (self.pitch * -1f32).to_radians().sin(),
                    self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
                );
                let direction = direction.normalize();
                engine
                    .camera
                    .set_target([direction.x, direction.y, direction.z]);
                self.camera_right = Self::update_vertices(&mut engine.camera);
            }
        }

        match events {
            blue_engine::DeviceEvent::Button { button, state } => {
                println!("{button}");
                if *button == 0 && *state == ElementState::Pressed {
                    println!("PRESSED LEFT BUTTON");
                }
            }
            DeviceEvent::Key(key) => {
                println!("{key:?}");
            }
            _ => {}
        }

        if engine
            .simple_input
            .mouse_pressed(blue_engine::MouseButton::Left)
        {
            engine
                .window
                .as_ref()
                .unwrap()
                .set_cursor_grab(winit::window::CursorGrabMode::None)
                .expect("Couldn't release the cursor");
            engine.window.as_ref().unwrap().set_cursor_visible(true);
            self.is_focus = false;
        }

        // SHIFT
        if engine.simple_input.held_shift() {
            camera_speed *= 0.003f32;
        }

        // W
        if engine.simple_input.key_held(blue_engine::KeyCode::KeyW) {
            println!("MOVE");
            let result = engine.camera.get("main").unwrap().position
                + (engine.camera.get("main").unwrap().target * camera_speed);

            engine.camera.set_position([result.x, result.y, result.z]);
            engine.camera.set_target([result.x, result.y, result.z]);
        }

        // S
        if engine.simple_input.key_held(blue_engine::KeyCode::KeyS) {
            let result = engine.camera.get("main").unwrap().position
                - (engine.camera.get("main").unwrap().target * camera_speed);

            engine.camera.set_position([result.x, result.y, result.z]);
            engine.camera.set_target([result.x, result.y, result.z]);
        }
        // A
        if engine.simple_input.key_held(blue_engine::KeyCode::KeyA) {
            let camera_pos = engine.camera.get("main").unwrap().position;
            let camera_pos = Vector3::new(camera_pos.x, camera_pos.y, camera_pos.z);
            let result = camera_pos - (self.camera_right * camera_speed);

            engine.camera.set_position([result.x, result.y, result.z]);
            engine.camera.set_target([result.x, result.y, result.z]);
        }
        // D
        if engine.simple_input.key_held(blue_engine::KeyCode::KeyD) {
            let camera_pos = engine.camera.get("main").unwrap().position;
            let camera_pos = Vector3::new(camera_pos.x, camera_pos.y, camera_pos.z);
            let result = camera_pos + (self.camera_right * camera_speed);

            engine.camera.set_position([result.x, result.y, result.z]);
            engine.camera.set_target([result.x, result.y, result.z]);
        }
    }
}

#![allow(unused)]

use blue_engine::{
    primitive_shapes::{cube, square, triangle, uv_sphere},
    uniform_type::Matrix,
    utils::default_resources::DEFAULT_MATRIX_4,
    Engine, Instance, ObjectSettings, PolygonMode, PowerPreference, RotateAxis, ShaderSettings,
    TextureData, Vertex, WindowDescriptor,
};

fn main() {
    let mut engine = Engine::new_config(blue_engine::WindowDescriptor {
        power_preference: blue_engine::PowerPreference::LowPower,
        present_mode: blue_engine::wgpu::PresentMode::Immediate,
        ..Default::default()
    })
    .expect("win");

    //let test_instance = Instance::default();
    //println!("{:?}", test_instance.to_raw());

    let texture = engine
        .renderer
        .build_texture(
            "background",
            TextureData::Path("resources/BlueLogoDiscord.png".to_string()),
            blue_engine::TextureMode::Clamp,
        )
        .unwrap();
    let texture2 = engine
        .renderer
        .build_texture(
            "background",
            TextureData::Path("resources/player.png".to_string()),
            blue_engine::TextureMode::Clamp,
        )
        .unwrap();

    let texture3 = engine
        .renderer
        .build_texture(
            "background",
            TextureData::Path("resources/image.png".to_string()),
            blue_engine::TextureMode::Clamp,
        )
        .unwrap();

    square(
        "main",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    );

    engine.objects.get_mut("main").unwrap().set_texture(texture);
    engine
        .objects
        .get_mut("main")
        .unwrap()
        .set_position(-1f32, 0f32, 0f32);

    square(
        "alt",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    );
    engine.objects.get_mut("alt").unwrap().set_texture(texture2);
    engine
        .objects
        .get_mut("alt")
        .unwrap()
        .set_position(0.2f32, 0f32, 0.001f32);

    square(
        "alt2",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    );
    engine
        .objects
        .get_mut("alt2")
        .unwrap()
        .set_texture(texture3);
    engine
        .objects
        .get_mut("alt2")
        .unwrap()
        .set_position(-0.2f32, 0f32, 0.001f32);

    let speed = -0.05;

    let mut last_time = std::time::Instant::now();
    let mut frames = 0;
    engine
        .update_loop(move |renderer, _window, objects, input, camera, plugins| {
            // calculate FPS
            let current_time = std::time::Instant::now();
            frames += 1;
            if current_time - last_time >= std::time::Duration::from_secs(1) {
                println!("{}ms/frame", 1000f32 / frames as f32);
                frames = 0;
                last_time = current_time;
            }

            let sprite = objects.get_mut("alt").unwrap();

            if input.key_held(blue_engine::KeyCode::ArrowUp) {
                sprite.set_position(
                    sprite.position.x,
                    sprite.position.y - speed,
                    sprite.position.z,
                );
                //lm.ambient_color.data = [1f32, 1f32, 1f32, 1f32];
            }
            if input.key_held(blue_engine::KeyCode::ArrowDown) {
                sprite.set_position(
                    sprite.position.x,
                    sprite.position.y + speed,
                    sprite.position.z,
                );
                //lm.ambient_color.data = [0.1f32, 0.1f32, 0.1f32, 1f32];
            }

            if input.key_held(blue_engine::KeyCode::ArrowLeft) {
                sprite.set_position(
                    sprite.position.x + speed,
                    sprite.position.y,
                    sprite.position.z,
                );
            }
            if input.key_held(blue_engine::KeyCode::ArrowRight) {
                sprite.set_position(
                    sprite.position.x - speed,
                    sprite.position.y,
                    sprite.position.z,
                );
            }

            if input.key_held(blue_engine::KeyCode::KeyE) {
                sprite.set_position(
                    sprite.position.x,
                    sprite.position.y,
                    sprite.position.z + speed,
                );
            }
            if input.key_held(blue_engine::KeyCode::KeyQ) {
                sprite.set_position(
                    sprite.position.x,
                    sprite.position.y,
                    sprite.position.z - speed,
                );
            }
        })
        .expect("Error during update loop");
}

use blue_engine::{
    primitive_shapes::{cube, square, triangle, uv_sphere},
    uniform_type::Matrix,
    utils::default_resources::DEFAULT_MATRIX_4,
    Engine, Instance, ObjectSettings, PolygonMode, PowerPreference, RotateAxis, ShaderSettings,
    TextureData, Vertex, WindowDescriptor,
};

fn main() {
    let mut engine = Engine::new().expect("win");

    //let test_instance = Instance::default();
    //println!("{:?}", test_instance.to_raw());

    let texture = engine
        .renderer
        .build_texture(
            "background",
            TextureData::Path("resources/BlueLogoDiscord.png"),
            blue_engine::TextureMode::Clamp,
        )
        .unwrap();
    let texture2 = engine
        .renderer
        .build_texture(
            "background",
            TextureData::Path("resources/SimpleJacob.png"),
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

    let speed = -0.05;
    engine
        .update_loop(move |renderer, _window, objects, input, camera, plugins| {
            let sprite = objects.get_mut("alt").unwrap();

            if input.key_held(blue_engine::VirtualKeyCode::Up) {
                sprite.set_position(
                    sprite.position.x,
                    sprite.position.y - speed,
                    sprite.position.z,
                );
                //lm.ambient_color.data = [1f32, 1f32, 1f32, 1f32];
            }
            if input.key_held(blue_engine::VirtualKeyCode::Down) {
                sprite.set_position(
                    sprite.position.x,
                    sprite.position.y + speed,
                    sprite.position.z,
                );
                //lm.ambient_color.data = [0.1f32, 0.1f32, 0.1f32, 1f32];
            }

            if input.key_held(blue_engine::VirtualKeyCode::Left) {
                sprite.set_position(
                    sprite.position.x + speed,
                    sprite.position.y,
                    sprite.position.z,
                );
            }
            if input.key_held(blue_engine::VirtualKeyCode::Right) {
                sprite.set_position(
                    sprite.position.x - speed,
                    sprite.position.y,
                    sprite.position.z,
                );
            }

            if input.key_held(blue_engine::VirtualKeyCode::E) {
                sprite.set_position(
                    sprite.position.x,
                    sprite.position.y,
                    sprite.position.z + speed,
                );
            }
            if input.key_held(blue_engine::VirtualKeyCode::Q) {
                sprite.set_position(
                    sprite.position.x,
                    sprite.position.y,
                    sprite.position.z - speed,
                );
            }
        })
        .expect("Error during update loop");
}

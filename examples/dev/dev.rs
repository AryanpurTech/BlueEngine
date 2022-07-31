use std::{ops::Mul, result};

use blue_engine::{
    header::{
        uniform_type::Matrix, Engine, ObjectSettings, RotateAxis, ShaderSettings, TextureData,
        Vertex, WindowDescriptor,
    },
    primitive_shapes::{cube, square, triangle},
    utils::{default_resources::DEFAULT_MATRIX_4, flycamera::FlyCamera},
    PolygonMode, // text::Text},
};
use std::time::Duration;

fn main() {
    let mut engine = Engine::new(WindowDescriptor {
        width: 800,
        height: 600,
        title: "title",
        decorations: false,
        resizable: true,
    })
    .expect("win");

    // ===============================

    /*let mut font = Text::new(
        include_bytes!("resource/JetBrainsMono-Medium.ttf"),
        25f32,
        &mut engine.renderer,
    )
    .unwrap();*/

    //let triangle_id = triangle(Some("Triangleee"), &mut engine, camera).unwrap();
    let window_size = engine.window.inner_size();
    let cube = cube(Some("CUBEE"), &mut engine).unwrap();
    cube.scale(0.3, 0.3, 0.3);
    let cube_index = cube.object_index;

    //let window_size = engine.window.inner_size();
    /*let change_texture = engine
    .renderer
    .build_and_append_texture(
        "name",
        TextureData::Bytes(include_bytes!("resource/BlueLogoDiscord.png").to_vec()),
        blue_engine::header::TextureMode::Clamp,
        //blue_engine::header::TextureFormat::PNG,
    )
    .unwrap();*/

    //let square = engine.get_object(square_id).unwrap();

    //square.change_color(0.0, 0.0, 1.0, 0.7).unwrap();
    //square.change_texture(change_texture);
    //square.resize(100.0, 100.0, 0.0, window_size);

    //let square = engine.objects.get_mut(square_id).unwrap();

    //square.no_stretch_update(&mut engine.renderer, engine.window.inner_size()).unwrap();
    //font.draw("Hello_World", (-100, 50), &mut engine).unwrap();

    let radius = 2f32;
    let start = std::time::SystemTime::now();
    let mut rotation = 0f32;
    let speed = -0.05;

    let mut fly_camera = FlyCamera::new(&mut engine.camera);

    engine
        .update_loop(move |_, window, objects, (event, input), camera| {
            //fly_camera.update(camera, window, event, input);

            //cube.translate(1f32, 1f32, 1f32);

            /*let sprite = objects.get_mut(cube_index).unwrap();

            if input.key_held(blue_engine::KeyboardKeys::Up) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1 + speed,
                    sprite.position.2,
                    window_size,
                );
            }
            if input.key_held(blue_engine::KeyboardKeys::Down) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1 - speed,
                    sprite.position.2,
                    window_size,
                );
            }

            if input.key_held(blue_engine::KeyboardKeys::Left) {
                sprite.position(
                    sprite.position.0 - speed,
                    sprite.position.1,
                    sprite.position.2,
                    window_size,
                );
            }
            if input.key_held(blue_engine::KeyboardKeys::Right) {
                sprite.position(
                    sprite.position.0 + speed,
                    sprite.position.1,
                    sprite.position.2,
                    window_size,
                );
            }

            if input.key_held(blue_engine::KeyboardKeys::E) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1,
                    sprite.position.2 - speed,
                    window_size,
                );
            }
            if input.key_held(blue_engine::KeyboardKeys::Q) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1,
                    sprite.position.2 + speed,
                    window_size,
                );
            } */
        })
        .expect("Error during update loop");
}

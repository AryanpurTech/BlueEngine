use std::{ops::Mul, result};

use blue_engine::{
    primitive_shapes::{cube, square, triangle, uv_sphere},
    uniform_type::Matrix,
    utils::{default_resources::DEFAULT_MATRIX_4, flycamera::FlyCamera},
    Engine, LightManager, ObjectSettings, PolygonMode, PowerPreference, RotateAxis, ShaderSettings,
    TextureData, Vertex, WindowDescriptor,
};
use std::time::Duration;

fn main() {
    let mut engine = Engine::new(WindowDescriptor {
        width: 1600,
        height: 1200,
        title: "diffuse light test",
        decorations: true,
        resizable: true,
        power_preference: PowerPreference::HighPerformance,
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
    let cube = uv_sphere(Some("CUBEE"), &mut engine, (18, 36, 1f32)).unwrap();
    cube.scale(0.6, 0.6, 0.6);
    //cube.scale(0.3, 0.3, 0.3);
    let cube_index = cube.object_index;

    let sphere_1 = uv_sphere(Some("SPHERE1"), &mut engine, (18, 36, 1f32)).unwrap();
    sphere_1.translate(2f32, 1f32, 0f32);
    sphere_1.set_color(1.0f32, 0.5f32, 0.31f32, 1f32);
    let sphere_2 = uv_sphere(Some("SPHERE2"), &mut engine, (18, 36, 1f32)).unwrap();
    sphere_2.translate(-2f32, 1f32, 0f32);
    sphere_2.set_color(1.0f32, 0.5f32, 0.31f32, 1f32);
    let sphere_3 = uv_sphere(Some("SPHERE3"), &mut engine, (18, 36, 1f32)).unwrap();
    sphere_3.translate(2f32, -1f32, 0f32);
    sphere_3.set_color(1.0f32, 0.5f32, 0.31f32, 1f32);
    let sphere_4 = uv_sphere(Some("SPHERE4"), &mut engine, (18, 36, 1f32)).unwrap();
    sphere_4.translate(-2f32, -1f32, 0f32);
    sphere_4.set_color(1.0f32, 0.5f32, 0.31f32, 1f32);

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
    let mut has_border = false;
    let mut val = 0f32;

    let mut lm = LightManager::new();
    lm.set_object_as_light(cube_index);

    engine
        .update_loop(move |renderer, window, objects, (event, input), camera| {
            fly_camera.update(camera, window, event, input);
            lm.update(objects, renderer).expect("Couldn't add light");

            //cube.translate(1f32, 1f32, 1f32);

            let sprite = objects.get_mut(cube_index).unwrap();

            if input.key_held(blue_engine::VirtualKeyCode::Up) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1 + speed,
                    sprite.position.2,
                );
                //lm.ambient_color.data = [1f32, 1f32, 1f32, 1f32];
            }
            if input.key_held(blue_engine::VirtualKeyCode::Down) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1 - speed,
                    sprite.position.2,
                );
                //lm.ambient_color.data = [0.1f32, 0.1f32, 0.1f32, 1f32];
            }

            if input.key_held(blue_engine::VirtualKeyCode::Left) {
                sprite.position(
                    sprite.position.0 - speed,
                    sprite.position.1,
                    sprite.position.2,
                );
            }
            if input.key_held(blue_engine::VirtualKeyCode::Right) {
                sprite.position(
                    sprite.position.0 + speed,
                    sprite.position.1,
                    sprite.position.2,
                );
            }

            if input.key_held(blue_engine::VirtualKeyCode::E) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1,
                    sprite.position.2 - speed,
                );
            }
            if input.key_held(blue_engine::VirtualKeyCode::Q) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1,
                    sprite.position.2 + speed,
                );
            }
        })
        .expect("Error during update loop");
}

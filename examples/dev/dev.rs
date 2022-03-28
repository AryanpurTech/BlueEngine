use std::{ops::Mul, result};

use blue_engine::{
    header::{
        uniform_type::Matrix, Engine, ObjectSettings, RotateAxis, ShaderSettings, TextureData,
        Vertex, WindowDescriptor,
    },
    objects::two_dimensions::{square, triangle},
    utils::{default_resources::DEFAULT_MATRIX_4, text::Text},
};
use legion::system;

#[system]
fn test(#[resource] t: &mut u8) {
    if *t != u8::MAX {
        *t += 1;
        //println!("{}", t);
    }
}

/*
    Good new structure idea:

    Pipeline: Entity with 4 component entity which later on render pass is queried.
    so e.g. shader is an entity with it's id saved to pipeline. and pipeline is an entity itself.

    OR:

    Pipeline includes all the built data, with the raw saved on the objects. This does have easier use but bigger in runtime size.
    A good fix for size can be referencing. although that might open to some borrow check errors not allowing same memory borrowed by many...


*/

struct haha {
    data: String,
}

fn main() {
    let mut engine = Engine::new(WindowDescriptor {
        width: 800,
        height: 600,
        title: "title",
        decorations: true,
        resizable: true,
    })
    .expect("win");

    let mut world = legion::World::default();

    let entity = world.push((
        Vertex {
            position: [-1f32, -1f32, -1f32],
            texture: [1f32, 1f32],
        },
        0f32,
    ));

    match world.entry(entity) {
        Some(data) => {
            //let t = data.get_component::<f32>();
            //println!("{:?}", t);
        }
        None => {}
    };
    let mut scheduler = legion::Schedule::builder()
        .add_system(test_system())
        .build();
    let mut res = legion::Resources::default();
    res.insert(5u8);

    // ===============================

    let mut font = Text::new(
        include_bytes!("resource/JetBrainsMono-Medium.ttf"),
        25f32,
        &mut engine.renderer,
    )
    .unwrap();

    //let triangle_id = triangle(Some("Triangleee"), &mut engine, camera).unwrap();
    let square_id = square(
        ObjectSettings {
            name: Some("SQUAREEE"),
            shader_settings: ShaderSettings {
                cull_mode: None,
                ..Default::default()
            },
            scale: (0.1, 0.1, 0.1),
            ..Default::default()
        },
        &mut engine,
    )
    .unwrap();
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
    font.draw("Hello_World", (-100, 50), &mut engine).unwrap();

    let radius = 2f32;
    let start = std::time::SystemTime::now();
    let mut rotation = 0f32;
    let speed = 0.5;

    engine
        .update_loop(move |renderer, window, objects, events, camera| {
            scheduler.execute(&mut world, &mut res);

            let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;
            //camera.set_eye([camx, 0.0, camz]);
            if events.key_pressed(blue_engine::header::KeyboardKeys::S) {
                let result = camera.position - camera.target * speed;
                camera.set_position(result.x, result.y, result.z);
            }
            if events.key_pressed(blue_engine::header::KeyboardKeys::W) {
                let result = camera.position + camera.target * speed;
                camera.set_position(result.x, result.y, result.z);
            }
        })
        .expect("Error during update loop");
}

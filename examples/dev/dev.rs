use blue_engine::{
    header::{
        uniform_type::Matrix, Engine, ObjectSettings, RotateAxis, TextureData, WindowDescriptor,
    },
    objects::two_dimensions::{square, triangle},
    utils::text::Text,
};

fn main() {
    let mut engine = Engine::new(WindowDescriptor {
        width: 800,
        height: 600,
        title: "title",
        decorations: true,
        resizable: true,
    })
    .expect("win");

    let mut font = Text::new(
        include_bytes!("resource/JetBrainsMono-Medium.ttf"),
        25f32,
        &mut engine.renderer,
    )
    .unwrap();

    //let triangle_id = triangle(Some("Triangleee"), &mut engine, camera).unwrap();
    /*let square_id = square(
        ObjectSettings {
            name: Some("SQUAREEE"),
            ..Default::default()
        },
        &mut engine,
    )
    .unwrap();*/
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

    engine
        .update_loop(move |renderer, window, objects, events, camera| {
            let camx = glm::sin(start.elapsed().unwrap().as_secs_f32()) * radius;
            let camz = glm::cos(start.elapsed().unwrap().as_secs_f32()) * radius;
            //camera.set_eye([camx, 0.0, camz]);
        })
        .expect("Error during update loop");
}

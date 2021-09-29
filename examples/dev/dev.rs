use blue_engine::{
    header::{uniform_type::Matrix, Engine, RotateAxis, WindowDescriptor},
    objects::{square, triangle},
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
        13f32,
        &mut engine.renderer
    )
    .unwrap();

    //let triangle_id = triangle(Some("Triangleee"), &mut engine, camera).unwrap();
    let square_id = square(Some("SQUAREEE"), &mut engine).unwrap();
    let window_size = engine.window.inner_size();
    let square = engine.get_object(square_id).unwrap();
    square.change_color(0.0, 0.0, 1.0, 0.7).unwrap();
    //square.resize(100.0, 100.0, 0.0, window_size);

    //let square = engine.objects.get_mut(square_id).unwrap();

    //square.no_stretch_update(&mut engine.renderer, engine.window.inner_size()).unwrap();
    font.draw("H", (50, 50), &mut engine).unwrap();

    let radius = 2f32;
    let start = std::time::SystemTime::now();

    engine
        .update_loop(move |renderer, window, objects, events, camera| {
            let camx = glm::sin(start.elapsed().unwrap().as_secs_f32()) * radius;
            let camz = glm::cos(start.elapsed().unwrap().as_secs_f32()) * radius;
            //camera.set_eye([camx, 0.0, camz]);

            if events.mouse_pressed(0) {
                let square = objects.get_mut(square_id).unwrap();
                square.position(10.0, 10.0, 0.0, window.inner_size());
                square.change_color(0.0, 1.0, 0.0, 0.7).unwrap();
            }
            if events.mouse_pressed(1) {
                let square = objects.get_mut(square_id).unwrap();
                square.position(100.0, 100.0, 0.0, window.inner_size());
                square.change_color(1.0, 0.0, 0.0, 0.7).unwrap();
            }
            if events.mouse_pressed(2) {
                let square = objects.get_mut(square_id).unwrap();
                square.position(0.0, 0.0, 0.0, window.inner_size());
                square.change_color(0.0, 0.0, 1.0, 0.7).unwrap();
            }
        })
        .expect("Error during update loop");
}

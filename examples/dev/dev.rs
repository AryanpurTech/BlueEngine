use blue_engine::{
    header::{uniform_type::Matrix, Engine, RotateAxis, WindowDescriptor},
    objects::{square, triangle},
    utils::text::Text,
};

fn main() {
    let mut engine = Engine::new(WindowDescriptor {
        width: 800.0,
        height: 600.0,
        title: "title",
        decorations: true,
        resizable: true,
    })
    .expect("win");

    /*let mut font = Text::new(
        std::fs::read(
            std::env::current_dir()
                .unwrap()
                .join("resource")
                .join("JetBrainsMono-Medium.ttf"),
        )
        .unwrap(),
        13f32,
    )
    .unwrap();*/

    //let triangle_id = triangle(Some("Triangleee"), &mut engine, camera).unwrap();
    let square_id = square(Some("SQUAREEE"), &mut engine).unwrap();
    {
        let a = engine.objects.get_mut(square_id).unwrap();
        //a.resize(800.0, 600.0, 1.0);
    }
    //let square = engine.objects.get_mut(square_id).unwrap();

    //square.no_stretch_update(&mut engine.renderer, engine.window.inner_size()).unwrap();
    //font.draw(
    //    "Hello World!",
    //    (50, 50),
    //    &mut engine,
    //    camera,
    //)
    //.unwrap();

    let radius = 2f32;
    let start = std::time::SystemTime::now();

    engine
        .update_loop(move |renderer, window, objects, events, camera| {
            let camx = glm::sin(start.elapsed().unwrap().as_secs_f32()) * radius;
            let camz = glm::cos(start.elapsed().unwrap().as_secs_f32()) * radius;
            //camera.set_eye([camx, 0.0, camz]);

            if events.mouse_pressed(0) {
                //camera.set_eye([1.0, 0.0, 1.0]);
                //let a = objects.get_mut(square_id).unwrap();
                //a.resize(50.0, 50.0, 0.0);
                objects
                    .get_mut(square_id)
                    .unwrap()
                    .translate(0.1, -0.1, 0.0);
            }
            if events.mouse_pressed(1) {
                objects
                    .get_mut(square_id)
                    .unwrap()
                    .rotate(25.0, RotateAxis::X);
            }
        })
        .expect("Error during update loop");
}

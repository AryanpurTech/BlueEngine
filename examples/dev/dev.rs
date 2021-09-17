use blue_engine::{
    definitions::{Engine, RotateAxis, WindowDescriptor},
    objects::{square, triangle},
    utils::text::Text,
};

fn main() {
    let mut engine = Engine::new(WindowDescriptor {
        width: 500.0,
        height: 500.0,
        title: "title",
        decorations: true,
        resizable: false,
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
    //let square = engine.objects.get_mut(square_id).unwrap();

    //square.no_stretch_update(&mut engine.renderer, engine.window.inner_size()).unwrap();
    //font.draw(
    //    "Hello World!",
    //    (50, 50),
    //    &mut engine,
    //    camera,
    //)
    //.unwrap();

    engine
        .update_loop(move |renderer, window, objects, events| {
            if events.mouse_pressed(0) {
                let a = objects.get_mut(square_id).unwrap();
                a.resize(50.0, 50.0, 0.0);
            }
            if events.mouse_pressed(1) {
                objects
                    .get_mut(square_id)
                    .unwrap()
                    .rotate(25.0, RotateAxis::Y);
            }
            // warning: Haswell Vulkan support is incomplete
        })
        .expect("Error during update loop");
}

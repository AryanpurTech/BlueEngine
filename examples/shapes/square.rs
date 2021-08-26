use blue_engine::{
    definitions::{Engine, WindowDescriptor},
    objects::square,
};

fn main() {
    let mut engine = Engine::new(WindowDescriptor {
        width: 500.0,
        height: 500.0,
        title: "square",
        decorations: true,
        resizable: false,
    })
    .expect("win");

    let camera = blue_engine::utils::camera::Camera::new(&mut engine.renderer)
        .expect("Couldn't create a camera");

    let _ = square(Some("Square"), &mut engine, camera).unwrap();

    engine
        .update_loop(move |_, _, _, _| {})
        .expect("Error during update loop");
}

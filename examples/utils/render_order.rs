use blue_engine::{primitive_shapes::square, Engine, ObjectSettings};

fn main() {
    let mut engine = Engine::new().expect("win");

    square(
        "layer1",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .expect("failed to create square");

    square(
        "layer2",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .expect("failed to create square");

    let layer1 = engine
        .objects
        .get_mut("layer1")
        .expect("failed to gete object");
    layer1
        .set_uniform_color(1f32, 0.5, 0f32, 1f32)
        .expect("failed to set color");
    layer1.set_position(-0.5, 0f32, 0f32);

    layer1.set_render_order(0).unwrap();

    let layer2 = engine
        .objects
        .get_mut("layer2")
        .expect("failed to gete object");
    layer2
        .set_uniform_color(0f32, 0f32, 1f32, 1f32)
        .expect("failed to set color");
    layer2.set_position(0.5, 0f32, 0f32);

    layer2.set_render_order(1).unwrap();

    let start = std::time::SystemTime::now();

    engine
        .update_loop(move |_, _, object_storage, _, _, _| {
            let target = object_storage.get_mut("layer1").unwrap();

            if start.elapsed().unwrap().as_secs() % 2 == 0 {
                target.set_render_order(2).unwrap();
            } else {
                target.set_render_order(0).unwrap();
            }
        })
        .expect("Error during update loop");
}

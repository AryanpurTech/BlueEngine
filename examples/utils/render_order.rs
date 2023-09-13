/*
* Render Order example by https://github.com/akowi-sknobloch which shows which object is rendered on top.
*
* The license is same as the one on the root.
*/

// imports needed
use blue_engine::{primitive_shapes::square, Engine, ObjectSettings};

fn main() {
    // initialize the engine
    let mut engine = Engine::new().expect("couldn't initialize engine");

    // make the first layer
    square(
        "layer1",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .expect("failed to create square");

    // make the second layer
    square(
        "layer2",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .expect("failed to create square");

    // Get layer 1 object
    let layer1 = engine
        .objects
        .get_mut("layer1")
        .expect("failed to gete object");
    // set a color to differenciate it
    layer1
        .set_uniform_color(1f32, 0.5, 0f32, 1f32)
        .expect("failed to set color");
    // move it to left a bit
    layer1.set_position(-0.5, 0f32, 0f32);
    // set render order to 0th
    layer1.set_render_order(0).unwrap();

    // Get layer 2 object
    let layer2 = engine
        .objects
        .get_mut("layer2")
        .expect("failed to gete object");
    // set a color to differenciate it
    layer2
        .set_uniform_color(0f32, 0f32, 1f32, 1f32)
        .expect("failed to set color");
    // move it to right a bit
    layer2.set_position(0.5, 0f32, 0f32);
    // set render order to 1st
    layer2.set_render_order(1).unwrap();

    // get a timer for order change
    let start = std::time::SystemTime::now();

    // start the update loop
    engine
        .update_loop(move |_, _, object_storage, _, _, _| {
            // get the target layer to change order of
            let target = object_storage.get_mut("layer1").unwrap();

            // on ever 2 seconds change order
            if start.elapsed().unwrap().as_secs() % 2 == 0 {
                target.set_render_order(2).unwrap();
            } else {
                // change back to default
                target.set_render_order(0).unwrap();
            }
        })
        .expect("Error during update loop");
}

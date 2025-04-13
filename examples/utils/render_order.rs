/*
* Render Order example by https://github.com/akowi-sknobloch which shows which object is rendered on top.
*
* The license is same as the one on the root.
*/

// imports needed
use blue_engine::{Engine, ObjectSettings, Vector3, primitive_shapes::square};

fn main() -> Result<(), blue_engine::error::Error> {
    // initialize the engine
    let mut engine = Engine::new()?;

    // make the first layer
    square(
        "layer1",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    // make the second layer
    square(
        "layer2",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    // Get layer 1 object
    engine
        .objects
        .get_mut("layer1")
        .expect("failed to gete object")
        .set_color(1f32, 0.5, 0f32, 1f32) // set a color to differentiate it
        .set_position(Vector3::new(-0.5, 0f32, 0f32)) // move it to left a bit
        .set_render_order(0); // set render order to 0th

    // Get layer 2 object
    engine
        .objects
        .get_mut("layer2")
        .expect("failed to gete object")
        .set_color(0f32, 0f32, 1f32, 1f32) // set a color to differentiate it
        .set_position(Vector3::new(0.5, 0f32, 0f32)) // move it to right a bit
        .set_render_order(1); // set render order to 1st

    // get a timer for order change
    let start = std::time::Instant::now();

    // start the update loop
    engine.update_loop(move |engine| {
        // get the target layer to change order of
        let target = engine.objects.get_mut("layer1").unwrap();

        // on ever 2 seconds change order
        if start.elapsed().as_secs() % 2 == 0 {
            target.set_render_order(2);
        } else {
            // change back to default
            target.set_render_order(0);
        }
    })?;

    Ok(())
}

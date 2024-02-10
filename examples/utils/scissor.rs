/*
 * Blue Engine by Elham Aryanpur
 *
 * Triangle example using pre-defined shapes
 *
 * The license is same as the one on the root.
*/

use blue_engine::{primitive_shapes::triangle, Engine, ObjectSettings};

pub fn main() {
    let mut engine = Engine::new().expect("win");

    triangle(
        "Triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .unwrap();

    // set scissor rect
    engine.renderer.scissor_rect = Some((0, 0, 450, 350));

    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}

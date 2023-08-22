/*
 * Blue Engine by Elham Aryanpur
 *
 * Triangle example using pre-defined shapes
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    header::{Engine, ObjectSettings},
    primitive_shapes::triangle,
};

pub fn main() {
    let mut engine = Engine::new().expect("win");

    triangle(
        "Triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .unwrap();

    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}

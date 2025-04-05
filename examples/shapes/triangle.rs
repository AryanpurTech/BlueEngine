/*
 * Blue Engine by Elham Aryanpur
 *
 * Triangle example using pre-defined shapes
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    prelude::{Engine, ObjectSettings},
    primitive_shapes::triangle,
};

fn main() -> Result<(), blue_engine::error::Error> {
    // initialize the engine
    let mut engine = Engine::new()?;

    // create a triangle
    triangle("my triangle", ObjectSettings::default(), &mut engine.renderer, &mut engine.objects)?;

    // run the engine
    engine
        .update_loop(move |_, _, _, _, _, _| {})?;

    Ok(())
}
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

pub fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new()?;

    triangle(
        "Triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    engine.update_loop(move |_, _, _, _, _, _| {})?;

    Ok(())
}

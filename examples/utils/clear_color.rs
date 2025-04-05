/*
 * Blue Engine by Elham Aryanpur
 *
 * Clear color example using pre-defined shapes
 *
 * The license is same as the one on the root.
*/

use blue_engine::{Engine, ObjectSettings, primitive_shapes::triangle, wgpu};

pub fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new()?;

    triangle(
        "Triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    engine.renderer.clear_color = wgpu::Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    engine.update_loop(move |_, _, _, _, _, _| {})?;

    Ok(())
}

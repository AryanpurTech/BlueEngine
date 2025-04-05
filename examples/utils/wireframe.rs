/*
 * Blue Engine by Elham Aryanpur
 *
 * Wireframe example using pre-defined shapes
 *
 * The license is same as the one on the root.
*/

use blue_engine::{Engine, ObjectSettings, ShaderSettings, primitive_shapes::triangle, wgpu};

pub fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new()?;

    triangle(
        "Triangle",
        ObjectSettings {
            shader_settings: ShaderSettings {
                polygon_mode: wgpu::PolygonMode::Line,
                ..Default::default()
            },
            ..Default::default()
        },
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    engine.update_loop(move |_, _, _, _, _, _| {})?;

    Ok(())
}

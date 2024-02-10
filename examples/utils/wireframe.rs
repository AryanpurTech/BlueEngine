/*
 * Blue Engine by Elham Aryanpur
 *
 * Wireframe example using pre-defined shapes
 *
 * The license is same as the one on the root.
*/

use blue_engine::{primitive_shapes::triangle, Engine, ObjectSettings, ShaderSettings};

pub fn main() {
    let mut engine = Engine::new().expect("win");

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
    )
    .unwrap();

    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}

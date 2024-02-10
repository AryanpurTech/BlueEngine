/*
 * Blue Engine by Elham Aryanpur
 *
 * Square example by defining custom vertices and indices
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    header::{Engine, ObjectSettings, Vertex},
    StringBuffer,
};

pub fn square(name: impl StringBuffer, engine: &mut Engine) -> color_eyre::Result<()> {
    engine.objects.new_object(
        name,
        vec![
            Vertex {
                position: [1.0, 1.0, 0.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, 1.0, 0.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
        ],
        vec![2, 1, 0, 2, 0, 3],
        ObjectSettings {
            camera_effect: false,
            ..Default::default()
        },
        &mut engine.renderer,
    )?;

    Ok(())
}

fn main() {
    let mut engine = Engine::new().expect("win");

    let _ = square("Square", &mut engine).unwrap();

    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}

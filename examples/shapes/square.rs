/*
 * Blue Engine by Elham Aryanpur
 *
 * Square example by defining custom vertices and indices
 *
 * The license is same as the one on the root.
*/

use blue_engine::header::{Engine, Object, ObjectSettings, Vertex, WindowDescriptor};

pub fn square<'a>(
    name: Option<&'static str>,
    engine: &'a mut Engine,
) -> Result<&'a mut Object, anyhow::Error> {
    let new_square = engine.new_object(
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
            name: name,
            camera_effect: false,
            ..Default::default()
        },
    )?;

    Ok(new_square)
}

fn main() {
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    let _ = square(Some("Square"), &mut engine).unwrap();

    engine
        .update_loop(move |_, _, _, _, _| {})
        .expect("Error during update loop");
}

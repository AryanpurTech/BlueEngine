/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::{Engine, ObjectSettings, Vertex};

/// Creates a 2D triangle
pub fn triangle<T: crate::UpdateEvents + 'static>(
    name: &'static str,
    settings: ObjectSettings,
    engine: &mut Engine<T>,
) -> anyhow::Result<()> {
    engine.new_object(
        name,
        vec![
            Vertex {
                position: [0.0, 1.0, 0.0],
                uv: [0.5, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
        ],
        vec![0, 1, 2],
        settings,
    )?;

    Ok(())
}

/// Creates a 2D square
pub fn square<T: crate::UpdateEvents + 'static>(
    name: &'static str,
    settings: ObjectSettings,
    engine: &mut Engine<T>,
) -> anyhow::Result<()> {
    engine.new_object(
        name,
        vec![
            Vertex {
                position: [1.0, 1.0, 0.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                uv: [1.0, 1.0],
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
        settings,
    )?;

    Ok(())
}

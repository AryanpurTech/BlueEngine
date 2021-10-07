/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::{Engine, ObjectSettings, Vertex};

/// Creates a 2D triangle
pub fn triangle(settings: ObjectSettings, engine: &mut Engine) -> Result<usize, anyhow::Error> {
    let new_triangle = engine.new_object(
        vec![
            Vertex {
                position: [0.0, 1.0, 0.0],
                texture: [0.5, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                texture: [1.0, 1.0],
            },
        ],
        vec![0, 1, 2],
        settings,
    )?;

    Ok(new_triangle)
}

/// Creates a 2D square
pub fn square(settings: ObjectSettings, engine: &mut Engine) -> Result<usize, anyhow::Error> {
    let new_square = engine.new_object(
        vec![
            Vertex {
                position: [1.0, 1.0, 0.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0, 0.0],
                texture: [0.0, 0.0],
            },
        ],
        vec![2, 1, 0, 2, 0, 3],
        settings,
    )?;

    Ok(new_square)
}

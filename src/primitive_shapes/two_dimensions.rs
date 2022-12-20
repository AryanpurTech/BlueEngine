/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::{header::{ObjectSettings, Vertex}, Renderer, Object};

/// Creates a 2D triangle
pub fn triangle(
    name: &'static str,
    settings: ObjectSettings,
    renderer: &mut Renderer,
    objects: &mut std::collections::HashMap<&'static str, Object>,
) -> anyhow::Result<()> {
    let object = renderer.build_object(
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
    objects.insert(name, object);

    Ok(())
}

/// Creates a 2D square
pub fn square(
    name: &'static str,
    settings: ObjectSettings,
    renderer: &mut Renderer,
    objects: &mut std::collections::HashMap<&'static str, Object>,
) -> anyhow::Result<()> {
    let object = renderer.build_object(
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
    objects.insert(name, object);

    Ok(())
}

/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::{
    header::{ObjectSettings, Vertex},
    ObjectStorage, Renderer, StringBuffer,
};

/// Creates a 2D triangle
pub fn triangle(
    name: impl StringBuffer,
    settings: ObjectSettings,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) -> color_eyre::Result<()> {
    objects.new_object(
        name.clone(),
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
        renderer,
    )?;

    Ok(())
}

/// Creates a 2D square
pub fn square(
    name: impl StringBuffer,
    settings: ObjectSettings,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) -> color_eyre::Result<()> {
    objects.new_object(
        name.clone(),
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
        renderer,
    )?;

    Ok(())
}

/// Create a 2D rectangle based on a width and height
pub fn rectangle(
    width: f32,
    height: f32,
    name: impl StringBuffer,
    settings: ObjectSettings,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) -> color_eyre::Result<()> {
    objects.new_object(
        name.clone(),
        vec![
            Vertex {
                position: [width / 2.0, height / 2.0, 0.0],
                uv: [1.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [width / 2.0, -height / 2.0, 0.0],
                uv: [1.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-width / 2.0, -height / 2.0, 0.0],
                uv: [0.0, 1.0],
                normal: [0f32, 0f32, 0f32],
            },
            Vertex {
                position: [-width / 2.0, height / 2.0, 0.0],
                uv: [0.0, 0.0],
                normal: [0f32, 0f32, 0f32],
            },
        ],
        vec![2, 1, 0, 2, 0, 3],
        settings,
        renderer,
    )?;

    Ok(())
}

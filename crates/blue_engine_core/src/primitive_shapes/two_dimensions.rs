/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
 */

/*
 * For the sake of example we never use Vector3::default() or Vector3::x_axis() or any axis.
 */

use crate::{
    ObjectStorage, Renderer, StringBuffer, Vector2, Vector3,
    prelude::{ObjectSettings, Vertex},
};

/// Creates a 2D triangle
pub fn triangle(
    name: impl StringBuffer,
    settings: ObjectSettings,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) {
    objects.new_object(
        name.clone(),
        vec![
            Vertex {
                position: Vector3::new(0.0, 1.0, 0.0),
                uv: Vector2::new(0.5, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, 0.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, 0.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
        ],
        vec![0, 1, 2],
        settings,
        renderer,
    );
}

/// Creates a 2D square
pub fn square(
    name: impl StringBuffer,
    settings: ObjectSettings,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) {
    objects.new_object(
        name.clone(),
        vec![
            Vertex {
                position: Vector3::new(1.0, 1.0, 0.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, 0.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, -1.0, 0.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-1.0, 1.0, 0.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
        ],
        vec![2, 1, 0, 2, 0, 3],
        settings,
        renderer,
    );
}

/// Create a 2D rectangle based on a width and height
pub fn rectangle(
    width: f32,
    height: f32,
    name: impl StringBuffer,
    settings: ObjectSettings,
    renderer: &mut Renderer,
    objects: &mut ObjectStorage,
) {
    objects.new_object(
        name.clone(),
        vec![
            Vertex {
                position: Vector3::new(width / 2.0, height / 2.0, 0.0),
                uv: Vector2::new(1.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(width / 2.0, -height / 2.0, 0.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-width / 2.0, -height / 2.0, 0.0),
                uv: Vector2::new(0.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-width / 2.0, height / 2.0, 0.0),
                uv: Vector2::new(0.0, 0.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
        ],
        vec![2, 1, 0, 2, 0, 3],
        settings,
        renderer,
    );
}

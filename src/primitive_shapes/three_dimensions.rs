use crate::{Engine, Object, ObjectSettings, Vertex};

pub fn cube<'a>(
    name: Option<&'static str>,
    engine: &'a mut Engine,
) -> Result<&'a mut Object, anyhow::Error> {
    let new_cube = engine.new_object(
        vec![
            // Front Face
            Vertex {
                position: [-1.0, -1.0, 1.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                texture: [0.0, 0.0],
            },
            // Back Face
            Vertex {
                position: [-1.0, 1.0, -1.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, -1.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                texture: [1.0, 1.0],
            },
            // Right face
            Vertex {
                position: [1.0, -1.0, -1.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                texture: [0.0, 1.0],
            },
            // Left Face
            Vertex {
                position: [-1.0, -1.0, 1.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, -1.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                texture: [0.0, 1.0],
            },
            // Top Face
            Vertex {
                position: [1.0, 1.0, -1.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, -1.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                texture: [1.0, 1.0],
            },
            // Bottom Face
            Vertex {
                position: [1.0, -1.0, 1.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, 1.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, -1.0],
                texture: [1.0, 1.0],
            },
        ],
        vec![
            0, 1, 2, 2, 3, 0, // top
            4, 5, 6, 6, 7, 4, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        ],
        ObjectSettings {
            name,
            ..Default::default()
        },
    )?;

    Ok(new_cube)
}

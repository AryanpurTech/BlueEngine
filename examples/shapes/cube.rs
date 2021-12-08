/*
 * Blue Engine by Elham Aryanpur
 *
 * Cube Example by Noswad#0001
 *
 * The license is same as the one on the root.
*/

use blue_engine::header::{Engine, ObjectSettings, Vertex, WindowDescriptor};

pub fn cube(name: Option<&'static str>, engine: &mut Engine) -> Result<usize, anyhow::Error> {
    let new_cube = engine.new_object(
        vec![
            // Top Face
            Vertex {
                position: [-1.0, -1.0, 1.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                texture: [0.0, 1.0],
            },
            // Botom Face
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
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                texture: [0.0, 1.0],
            },
            // Left Face
            Vertex {
                position: [-1.0, -1.0, 1.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, -1.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                texture: [1.0, 1.0],
            },
            // Front Face
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
            // Back Face
            Vertex {
                position: [1.0, -1.0, 1.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, 1.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, -1.0],
                texture: [0.0, 1.0],
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

fn main() {
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    let _ = cube(Some("Cube"), &mut engine).unwrap();

    let radius = 2f32;
    let start = std::time::SystemTime::now();
    engine
        .update_loop(move |_, _, _, _, camera| {
            let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;
            camera
                .set_position(camx, camy, camz)
                .expect("Couldn't update the camera eye");
        })
        .expect("Error during update loop");
}

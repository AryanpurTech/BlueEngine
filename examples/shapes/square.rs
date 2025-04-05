/*
 * Blue Engine by Elham Aryanpur
 *
 * Square example by defining custom vertices and indices
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    Object, StringBuffer,
    prelude::{Engine, ObjectSettings, Vertex},
};

pub fn square(
    name: impl StringBuffer,
    engine: &mut Engine,
) -> Result<(), blue_engine::error::Error> {
    let vertices = vec![
        Vertex {
            position: [1.0, 1.0, 0.0],
            uv: [1.0, 1.0],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0, 0.0],
            uv: [1.0, 0.0],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [-1.0, -1.0, 0.0],
            uv: [0.0, 1.0],
            normal: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [-1.0, 1.0, 0.0],
            uv: [0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
        },
    ];

    engine.objects.insert(
        name.as_string(),
        Object::new(
            name,
            vertices,
            vec![2, 1, 0, 2, 0, 3],
            ObjectSettings {
                camera_effect: None,
                ..Default::default()
            },
            &mut engine.renderer,
        )?,
    );

    Ok(())
}

fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new()?;

    square("Square", &mut engine)?;

    engine.update_loop(move |_, _, _, _, _, _| {})?;

    Ok(())
}

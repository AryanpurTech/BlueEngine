/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    header::{Engine, WindowDescriptor, Vertex},
};

pub fn square(
    name: Option<&'static str>,
    engine: &mut Engine,
) -> Result<usize, anyhow::Error> {
    let new_square = engine.new_object(
        name,
        vec![
            Vertex {
                position: [1.0, 1.0, 0.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                texture: [1.0, 0.0],
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
    )?;

    Ok(new_square)
}

fn main() {
    let mut engine = Engine::new(WindowDescriptor::default())
    .expect("win");

    let _ = square(Some("Square"), &mut engine).unwrap();

    engine
        .update_loop(move |_, _, _, _, _| {})
        .expect("Error during update loop");
}

/*
 * Blue Engine by Elham Aryanpur
 *
 * Square example by defining custom vertices and indices
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    header::{Engine, ObjectSettings, Vertex},
    StringBuffer, Vector2, Vector3,
};

pub fn square(name: impl StringBuffer, engine: &mut Engine) {
    engine.objects.new_object(
        name,
        vec![
            Vertex {
                position: Vector3::new(1.0, 1.0, 0.0),
                uv: Vector2::new(1.0, 1.0),
                normal: Vector3::new(0.0, 0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(1.0, -1.0, 0.0),
                uv: Vector2::new(1.0, 0.0),
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
        ObjectSettings {
            camera_effect: None,
            ..Default::default()
        },
        &mut engine.renderer,
    );
}

fn main() {
    let mut engine = Engine::new().expect("win");

    square("Square", &mut engine);

    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}

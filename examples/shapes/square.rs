/*
 * Blue Engine by Elham Aryanpur
 *
 * Square example by defining custom vertices and indices
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    StringBuffer, Vector2, Vector3,
    header::{Engine, ObjectSettings, Vertex},
};

pub fn square(name: impl StringBuffer, engine: &mut Engine) {
    let vertices = vec![
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
    ];

    engine.objects.new_object(
        name,
        vertices,
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

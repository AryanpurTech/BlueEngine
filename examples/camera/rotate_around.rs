/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

extern crate nalgebra_glm as glm;
use blue_engine::{
    header::{Engine, ObjectSettings, ShaderSettings, WindowDescriptor},
    objects::two_dimensions::square,
};

fn main() {
    // Create the engine
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    // create a square
    let _ = square(
        ObjectSettings {
            // let's give it a name
            name: Some("Rotating Square"),
            // and set the size
            size: (500f32, 500f32, 1f32),
            // we need it to not cull it's back face so that it's visible on both side
            shader_settings: ShaderSettings {
                cull_mode: None,
                ..Default::default()
            },
            // and have default settings for the rest
            ..Default::default()
        },
        &mut engine,
    )
    .unwrap();

    let radius = 2f32;
    let start = std::time::SystemTime::now();

    engine
        .update_loop(move |_, _, _, _, camera| {
            let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;
            camera
                .set_eye(camx, 0.0, camz)
                .expect("Couldn't update the camera eye");
        })
        .expect("Error during update loop");
}

/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    header::{Engine, WindowDescriptor},
    objects::square,
};

fn main() {
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    let square_id = square(Some("Rotating Square"), &mut engine).unwrap();
    {
        let sq = engine.objects.get_mut(square_id).unwrap();
        sq.resize(800.0, 600.0, 1.0);
    }

    let radius = 2f32;
    let start = std::time::SystemTime::now();

    engine
        .update_loop(move |_, _, _, _, camera| {
            let camx = glm::sin(start.elapsed().unwrap().as_secs_f32()) * radius;
            let camz = glm::cos(start.elapsed().unwrap().as_secs_f32()) * radius;
            camera.set_eye([camx, 0.0, camz]).expect("Couldn't update the camera eye");
        })
        .expect("Error during update loop");
}

/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use blue_engine::{header::Engine, primitive_shapes::cube};

fn main() {
    let mut engine = Engine::new().expect("win");

    cube("Cube", &mut engine.renderer, &mut engine.objects).unwrap();
    engine
        .objects
        .get_mut("Cube")
        .unwrap()
        .set_color(0f32, 0f32, 1f32, 1f32)
        .unwrap();

    let radius = 5f32;
    let start = std::time::SystemTime::now();
    engine
        .update_loop(move |_, _, _, _, camera, _| {
            let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;
            camera
                .set_position(camx, camy, camz)
                .expect("Couldn't update the camera eye");
        })
        .expect("Error during update loop");
}

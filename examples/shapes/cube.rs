/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use blue_engine::{header::Engine, primitive_shapes::cube, PowerPreference, WindowDescriptor};

fn main() {
    let mut engine = Engine::new_config(WindowDescriptor {
        width: 1920,
        height: 1080,
        power_preference: blue_engine::PowerPreference::HighPerformance,
        ..Default::default()
    })
    .expect("win");
    engine
        .window
        .set_fullscreen(Some(blue_engine::winit::window::Fullscreen::Exclusive(
            engine
                .window
                .current_monitor()
                .unwrap()
                .video_modes()
                .next()
                .unwrap(),
        )));

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

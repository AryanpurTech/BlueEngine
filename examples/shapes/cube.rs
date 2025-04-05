/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use blue_engine::{prelude::Engine, primitive_shapes::cube};

fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new()?;

    cube("Cube", &mut engine.renderer, &mut engine.objects)?;
    engine
        .objects
        .get_mut("Cube")
        .unwrap()
        .set_color(0f32, 0f32, 1f32, 1f32);

    let radius = 5f32;
    let start = std::time::Instant::now();
    engine.update_loop(move |_, _, _, _, camera, _| {
        let camx = start.elapsed().as_secs_f32().sin() * radius;
        let camy = start.elapsed().as_secs_f32().sin() * radius;
        let camz = start.elapsed().as_secs_f32().cos() * radius;
        camera.set_position((camx, camy, camz));
    })?;

    Ok(())
}

use blue_engine::{Engine, WindowDescriptor, primitive_shapes::cube};
use blue_engine_utilities::FlyCamera;

fn main() -> eyre::Result<()> {
    let mut engine = Engine::new_config(WindowDescriptor {
        width: 1500,
        height: 1000,
        title: "Fly Camera",
        ..Default::default()
    })?;

    let texture_data = include_bytes!("../resources/BlueLogoDiscord.png").to_vec();
    let texture = engine.renderer.build_texture(
        "crate texture",
        blue_engine::TextureData::Bytes(texture_data),
        blue_engine::TextureMode::Clamp,
    )?;
    cube("floor", &mut engine.renderer, &mut engine.objects);
    engine
        .objects
        .get_mut("floor")
        .unwrap()
        .set_texture(texture);

    // camera
    let fly_camera = FlyCamera::new(&mut engine.camera);

    // Add fly camera to the engine as plugin
    engine.signals.add_signal("flycamera", Box::new(fly_camera));

    let timer = std::time::SystemTime::now();
    let mut tick: u64 = 0;
    let mut fps: i32 = 0;

    engine.update_loop(move |_, _, _, _, _, _| {
        let now = timer.elapsed().unwrap().as_secs();
        if tick < now {
            tick = now;
            println!("FPS: {}", fps);
            fps = 0;
        } else {
            fps += 1;
        }
    })?;

    Ok(())
}

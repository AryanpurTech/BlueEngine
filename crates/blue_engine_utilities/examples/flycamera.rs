use blue_engine::{Engine, EngineSettings, ObjectSettings, primitive_shapes::cube};
use blue_engine_utilities::FlyCamera;

fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new_config(EngineSettings {
        width: 1500,
        height: 1000,
        title: "Fly Camera",
        present_mode: egui_wgpu::wgpu::PresentMode::Fifo,
        ..Default::default()
    })?;

    cube(
        "floor",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;
    engine.objects.get_mut("floor").unwrap().set_texture(
        "crate texture",
        blue_engine::TextureData::Path("./resources/BlueLogoDiscord.png".to_string()),
        blue_engine::TextureMode::Clamp,
        &mut engine.renderer,
    )?;

    // camera
    let fly_camera = FlyCamera::new(&mut engine.camera);

    // Add fly camera to the engine as plugin
    engine.signals.add_signal("flycamera", Box::new(fly_camera));

    let timer = std::time::SystemTime::now();
    let mut tick: u64 = 0;
    let mut fps: i32 = 0;

    engine.update_loop(move |_| {
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

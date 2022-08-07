use blue_engine::{
    header::{Engine, WindowDescriptor},
    primitive_shapes::cube,
    utils::flycamera::FlyCamera,
};

fn main() -> anyhow::Result<()> {
    let mut engine = Engine::new(WindowDescriptor {
        width: 1500,
        height: 1000,
        title: "Fly Camera",
        ..Default::default()
    })?;

    //let trig = triangle(ObjectSettings::default(), &mut engine)?;
    let texture_data = include_bytes!("BlueLogoDiscord.png").to_vec();
    let texture = engine.renderer.build_texture(
        "crate texture",
        blue_engine::header::TextureData::Bytes(texture_data),
        blue_engine::header::TextureMode::Clamp,
    )?;
    let floor = cube(Some("floor"), &mut engine)?;
    floor.set_texture(texture)?;

    // camera
    let mut fly_camera = FlyCamera::new(&mut engine.camera);

    let timer = std::time::SystemTime::now();
    let mut tick: u64 = 0;
    let mut fps: i32 = 0;

    engine.update_loop(move |_, window, _, (event, input), camera| {
        let now = timer.elapsed().unwrap().as_secs();
        if tick < now {
            tick = now;
            println!("FPS: {}", fps);
            fps = 0;
        } else {
            fps = fps + 1;
        }

        fly_camera.update(camera, window, event, input);
    })?;

    Ok(())
}

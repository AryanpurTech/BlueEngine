use blue_engine::{
    header::{Engine, WindowDescriptor},
    primitive_shapes::cube,
    utils::{
        animation::{Animation, Operation},
        flycamera::FlyCamera,
    },
};

fn main() -> anyhow::Result<()> {
    let mut engine = Engine::new(WindowDescriptor {
        width: 1500,
        height: 1000,
        title: "Fly Camera",
        ..Default::default()
    })?;

    //let trig = triangle(ObjectSettings::default(), &mut engine)?;
    let texture_data = include_bytes!("../../resources/BlueLogoDiscord.png").to_vec();
    let texture = engine.renderer.build_texture(
        "crate texture",
        blue_engine::header::TextureData::Bytes(texture_data),
        blue_engine::header::TextureMode::Clamp,
    )?;
    cube("floor", &mut engine)?;
    engine
        .objects
        .get_mut("floor")
        .unwrap()
        .set_texture(texture)?;

    // camera
    let fly_camera = FlyCamera::new(&mut engine.camera);

    // Add fly camera to the engine as plugin
    engine.plugins.push(Box::new(fly_camera));

    let timer = std::time::SystemTime::now();
    let mut tick: u64 = 0;
    let mut fps: i32 = 0;

    let now = std::time::Instant::now();
    let target_x = 1000f32;
    let target_time = std::time::Duration::from_secs(10).as_millis();
    let difference = target_x / target_time as f32;

    let mut animation = Animation::new("floor");
    animation.keyframes.push((
        2,
        Operation {
            translation: (20f32, 0f32, 0f32),
            rotation: (0f32, 0f32, 0f32),
        },
    ));

    animation.keyframes.push((
        5,
        Operation {
            translation: (-20f32, 0f32, 0f32),
            rotation: (0f32, 0f32, 0f32),
        },
    ));

    engine.update_loop(move |_, _, objects, input, _, _| {
        /*let cube = objects.get_mut("floor").unwrap();
        let elapsed = now.elapsed().as_millis();
        if elapsed <= target_time {
            let x_rotation = cube.rotation.0;
            cube.rotate(x_rotation * -1f32, blue_engine::RotateAxis::X);
            cube.translate(difference, 0f32, 0f32);
            cube.rotate(difference + x_rotation, blue_engine::RotateAxis::X)
        } */
        animation.animate(objects);

        if input.key_pressed(blue_engine::VirtualKeyCode::B) {
            println!("position: {:?}", objects.get("floor").unwrap().position);
        }

        let now = timer.elapsed().unwrap().as_secs();
        if tick < now {
            tick = now;
            //println!("FPS: {}", fps);
            fps = 0;
        } else {
            fps = fps + 1;
        }
    })?;

    Ok(())
}

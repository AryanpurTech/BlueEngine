use blue_engine::{
    primitive_shapes::{cube, square, triangle, uv_sphere},
    uniform_type::Matrix,
    utils::{default_resources::DEFAULT_MATRIX_4, flycamera::FlyCamera, loader::load_gltf},
    Engine, LightManager, ObjectSettings, PolygonMode, PowerPreference, RotateAxis, ShaderSettings,
    TextureData, Vertex, WindowDescriptor,
};

fn main() {
    let mut engine = Engine::new(WindowDescriptor {
        width: 1600,
        height: 1200,
        title: "diffuse light test",
        decorations: true,
        resizable: true,
        power_preference: PowerPreference::HighPerformance,
    })
    .expect("win");

    // ===============================

    //let triangle_id = triangle(Some("Triangleee"), &mut engine, camera).unwrap();
    let window_size = engine.window.inner_size();

    uv_sphere("cube", &mut engine, (18, 36, 1f32)).unwrap();
    engine.objects.get_mut("cube").unwrap().scale(0.6, 0.6, 0.6);
    engine
        .objects
        .get_mut("cube")
        .unwrap()
        .set_color(1f32, 0f32, 0f32, 1f32);
    //cube.scale(0.3, 0.3, 0.3);

    let test = load_gltf(
        "monke",
        "/home/elhamaryanpur/Desktop/Projects/Blue Engine/examples/shapes/monkey.glb",
        &mut engine,
    )
    .unwrap();

    engine
        .objects
        .get_mut("monke")
        .unwrap()
        .set_color(0.051f32, 0.533f32, 0.898f32, 1f32);
    //engine.objects[test].rotate(90f32, RotateAxis::Y);

    /*let sphere_1 = uv_sphere(Some("SPHERE1"), &mut engine, (18, 36, 1f32)).unwrap();
    engine.objects[sphere_1].scale(2f32, 2f32, 2f32);
    engine.objects[sphere_1].set_color(0.051f32, 0.533f32, 0.898f32, 1f32);

    let sphere_1 = uv_sphere(Some("SPHERE1"), &mut engine, (18, 36, 1f32)).unwrap();
    engine.objects[sphere_1].position(2f32, 1f32, 0f32);
    engine.objects[sphere_1].set_color(1.0f32, 0.5f32, 0.31f32, 1f32);
    let sphere_2 = uv_sphere(Some("SPHERE2"), &mut engine, (18, 36, 1f32)).unwrap();
    engine.objects[sphere_2].position(-2f32, 1f32, 0f32);
    engine.objects[sphere_2].set_color(1.0f32, 0.5f32, 0.31f32, 1f32);
    let sphere_3 = uv_sphere(Some("SPHERE3"), &mut engine, (18, 36, 1f32)).unwrap();
    engine.objects[sphere_3].position(2f32, -1f32, 0f32);
    engine.objects[sphere_3].set_color(1.0f32, 0.5f32, 0.31f32, 1f32);
    let sphere_4 = uv_sphere(Some("SPHERE4"), &mut engine, (18, 36, 1f32)).unwrap();
    engine.objects[sphere_4].position(-2f32, -1f32, 0f32);
    engine.objects[sphere_4].set_color(1.0f32, 0.5f32, 0.31f32, 1f32); */

    //let window_size = engine.window.inner_size();
    /*let change_texture = engine
    .renderer
    .build_and_append_texture(
        "name",
        TextureData::Bytes(include_bytes!("resource/BlueLogoDiscord.png").to_vec()),
        blue_engine::header::TextureMode::Clamp,
        //blue_engine::header::TextureFormat::PNG,
    )
    .unwrap();*/

    //let square = engine.get_object(square_id).unwrap();

    //square.change_color(0.0, 0.0, 1.0, 0.7).unwrap();
    //square.change_texture(change_texture);
    //square.resize(100.0, 100.0, 0.0, window_size);

    //let square = engine.objects.get_mut(square_id).unwrap();

    //square.no_stretch_update(&mut engine.renderer, engine.window.inner_size()).unwrap();
    //font.draw("Hello_World", (-100, 50), &mut engine).unwrap();

    let radius = 10f32;
    let start = std::time::SystemTime::now();
    let mut rotation = 0f32;
    let speed = -0.05;

    let mut fly_camera = FlyCamera::new(&mut engine.camera);
    let mut has_border = false;
    let mut val = 0f32;

    let mut lm = LightManager::new();
    lm.set_object_as_light("cube");

    engine.renderer.custom_render_pass = Some(Box::new(|encoder, view| {
        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        drop(render_pass);
    }));

    engine
        .update_loop(move |renderer, window, objects, (event, input), camera| {
            fly_camera.update(camera, window, event, input);
            lm.update(objects, renderer, &camera)
                .expect("Couldn't add light");

            let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
            let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;

            objects.get_mut("cube").unwrap().position(camx, camy, camz);

            //cube.translate(1f32, 1f32, 1f32);

            let sprite = objects.get_mut("cube").unwrap();

            if input.key_held(blue_engine::VirtualKeyCode::Up) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1 + speed,
                    sprite.position.2,
                );
                //lm.ambient_color.data = [1f32, 1f32, 1f32, 1f32];
            }
            if input.key_held(blue_engine::VirtualKeyCode::Down) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1 - speed,
                    sprite.position.2,
                );
                //lm.ambient_color.data = [0.1f32, 0.1f32, 0.1f32, 1f32];
            }

            if input.key_held(blue_engine::VirtualKeyCode::Left) {
                sprite.position(
                    sprite.position.0 - speed,
                    sprite.position.1,
                    sprite.position.2,
                );
            }
            if input.key_held(blue_engine::VirtualKeyCode::Right) {
                sprite.position(
                    sprite.position.0 + speed,
                    sprite.position.1,
                    sprite.position.2,
                );
            }

            if input.key_held(blue_engine::VirtualKeyCode::E) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1,
                    sprite.position.2 - speed,
                );
            }
            if input.key_held(blue_engine::VirtualKeyCode::Q) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1,
                    sprite.position.2 + speed,
                );
            }
        })
        .expect("Error during update loop");
}

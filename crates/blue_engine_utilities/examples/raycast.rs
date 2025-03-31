#[cfg(feature = "physics")]
use blue_engine::{Engine, WindowDescriptor, imports::glm, primitive_shapes::cube};
#[cfg(feature = "physics")]
use blue_engine_utilities::{FlyCamera, raycast::Raycast};

fn main() -> eyre::Result<()> {
    #[cfg(feature = "physics")]
    {
        let mut engine = Engine::new_config(WindowDescriptor {
            width: 1000,
            height: 1000,
            title: "Raycast",
            ..Default::default()
        })?;

        cube("cube1", &mut engine.renderer, &mut engine.objects);
        cube("cube2", &mut engine.renderer, &mut engine.objects);
        cube("cube3", &mut engine.renderer, &mut engine.objects);

        engine
            .objects
            .get_mut("cube1")
            .unwrap()
            .set_position([10f32, 1f32, -10f32]);
        engine
            .objects
            .get_mut("cube2")
            .unwrap()
            .set_position([-5f32, -5f32, -5f32]);
        engine
            .objects
            .get_mut("cube3")
            .unwrap()
            .set_position([0f32, 5f32, -7f32]);

        // camera
        let _fly_camera = FlyCamera::new(&mut engine.camera);

        // Add fly camera to the engine as plugin
        //engine.plugins.push(Box::new(fly_camera));

        let mut raycast = Raycast::new(engine.camera.get("main").unwrap());

        engine.update_loop(move |_, window, objects, input, camera, _| {
            raycast.update(
                camera.get("main").unwrap(),
                input,
                &window.as_ref().unwrap().inner_size(),
            );

            let obj = objects.get_mut("cube1").unwrap();

            //if input.mouse_pressed(0) {
            let _raycast_pos = raycast.get_current_ray();
            //cube("cube5", renderer, objects);
            //obj.position(raycast_pos.x, raycast_pos.y, raycast_pos.z);
            //}
            //println!("{:?}", raycast_pos);

            let transformation_matrix =
                obj.position_matrix * obj.rotation_matrix * obj.scale_matrix;

            raycast.ray_intersects_bounding_box(
                (
                    (transformation_matrix
                        * glm::vec4(obj.position.x, obj.position.y, obj.position.z, 1f32))
                    .xyz(),
                    (transformation_matrix * glm::vec4(obj.size.x, obj.size.y, obj.size.z, 1f32))
                        .xyz(),
                ),
                1f32,
                camera.get("main").unwrap(),
            );
        })?;
    }

    Ok(())
}

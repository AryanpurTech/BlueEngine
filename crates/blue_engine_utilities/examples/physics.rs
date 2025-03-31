/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

#[cfg(feature = "physics")]
use blue_engine::{
    Engine, WindowDescriptor,
    primitive_shapes::{cube, uv_sphere},
};
#[cfg(feature = "physics")]
use blue_engine_utilities::{FlyCamera, physics::Physics, raycast::Raycast};
#[cfg(feature = "physics")]
use rapier3d::prelude::*;

fn main() -> eyre::Result<()> {
    #[cfg(feature = "physics")]
    {
        let mut engine = Engine::new_config(WindowDescriptor {
            width: 1500,
            height: 1000,
            title: "Fly Camera",
            ..Default::default()
        })?;

        let mut physics = Physics::new();
        let fly_camera = FlyCamera::new(&mut engine.camera);

        cube("floor", &mut engine.renderer, &mut engine.objects);
        engine
            .objects
            .get_mut("floor")
            .unwrap()
            .set_scale([2f32, 0.3f32, 2f32])
            .set_position([0f32, 1f32, 0f32]);
        let collider = ColliderBuilder::cuboid(2.0, 0.3f32, 2.0)
            .translation([0f32, 1f32, 0f32].into())
            .build();
        physics.insert_collider("floor", collider);

        uv_sphere(
            "ball",
            (18, 46, 1f32),
            &mut engine.renderer,
            &mut engine.objects,
        );
        engine
            .objects
            .get_mut("ball")
            .unwrap()
            .set_color(0.3, 0.3, 0.6, 1f32)
            .set_position([0f32, -10f32, 0f32]);
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, -10f32, 0.0])
            .build();
        let collider = ColliderBuilder::ball(1f32).restitution(0.7).build();
        let ball_body_handle = physics.insert_rigid_body("ball", rigid_body);
        physics.insert_collider_with_parent("ball collider", collider, ball_body_handle);

        uv_sphere(
            "ball2",
            (18, 46, 1f32),
            &mut engine.renderer,
            &mut engine.objects,
        );
        engine
            .objects
            .get_mut("ball2")
            .unwrap()
            .set_color(0.3, 0.3, 0.6, 1f32);
        engine
            .objects
            .get_mut("ball2")
            .unwrap()
            .set_position([0.05f32, 13f32, 0.0f32]);
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.05, 13.0, 0.0])
            .build();
        let collider = ColliderBuilder::ball(1f32).restitution(0.7).build();
        let ball_body_handle = physics.insert_rigid_body("ball2", rigid_body);
        physics.insert_collider_with_parent("ball2 collider", collider, ball_body_handle);

        let mut raycast = Raycast::new(engine.camera.get("main").unwrap());

        engine.signals.add_signal("fly", Box::new(fly_camera));
        engine.signals.add_signal("physics", Box::new(physics));

        engine.update_loop(move |_, window, _, input, camera, signals| {
            let physics = signals.get_signal::<Physics>("physics").unwrap().unwrap();
            raycast.update(
                camera.get("main").unwrap(),
                input,
                &window.as_ref().unwrap().inner_size(),
            );

            let camera_pos = camera.get("main").unwrap().position;
            let camera_pos = blue_engine::glm::vec3(camera_pos.x, camera_pos.y, camera_pos.z);
            let ray = Ray::new(camera_pos.into(), raycast.get_current_ray());
            let max_toi = 4.0;
            let solid = true;
            let filter = QueryFilter::default();

            if let Some((handle, toi)) = physics.query_pipeline.cast_ray(
                &physics.rigid_body_set,
                &physics.collider_set,
                &ray,
                max_toi,
                solid,
                filter,
            ) {
                // The first collider hit has the handle `handle` and it hit after
                // the ray travelled a distance equal to `ray.dir * toi`.
                let hit_point = ray.point_at(toi); // Same as: `ray.origin + ray.dir * toi`
                println!("Collider {:?} hit at point {}", handle, hit_point);
            }
        })?;
    }

    Ok(())
}

/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

#[cfg(feature = "physics")]
use blue_engine::{
    Engine, EngineSettings, ObjectSettings,
    primitive_shapes::{cube, uv_sphere},
};
#[cfg(feature = "physics")]
use blue_engine_utilities::{FlyCamera, physics::Physics};
#[cfg(feature = "physics")]
use rapier3d::prelude::*;

fn main() -> Result<(), blue_engine::error::Error> {
    #[cfg(feature = "physics")]
    {
        let mut engine = Engine::new_config(EngineSettings {
            width: 1500,
            height: 1000,
            title: "Fly Camera",
            ..Default::default()
        })?;

        let mut physics = Physics::new();
        let fly_camera = FlyCamera::new(&mut engine.camera);

        cube(
            "floor",
            ObjectSettings::default(),
            &mut engine.renderer,
            &mut engine.objects,
        )?;
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
            ObjectSettings::default(),
            (18, 46, 1f32),
            &mut engine.renderer,
            &mut engine.objects,
        )?;
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
            ObjectSettings::default(),
            (18, 46, 1f32),
            &mut engine.renderer,
            &mut engine.objects,
        )?;
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

        engine.signals.add_signal("fly", Box::new(fly_camera));
        engine.signals.add_signal("physics", Box::new(physics));

        engine.update_loop(move |_| {})?;
    }

    Ok(())
}

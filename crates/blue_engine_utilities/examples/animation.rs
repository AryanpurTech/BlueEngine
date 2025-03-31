#[cfg(feature = "animation")]
use blue_engine::{Engine, WindowDescriptor, primitive_shapes::cube};
#[cfg(feature = "animation")]
use blue_engine_utilities::{AnimationKeyframe, animation::Animation};

fn main() -> eyre::Result<()> {
    #[cfg(feature = "animation")]
    {
        let mut engine = Engine::new_config(WindowDescriptor {
            width: 1280,
            height: 720,
            title: "Animation test",
            ..Default::default()
        })?;

        // make a cube
        cube("cube", &mut engine.renderer, &mut engine.objects)?;

        // initialize an animation sequence
        let mut animation = Animation::new("cube");

        // first frame, set the current data of the object
        animation
            .keyframes
            .push((0.0, AnimationKeyframe::default()));

        // second frame, where to go from first frame
        animation.keyframes.push((
            // how many seconds
            5.0,
            // the frame data. it uses Point3 type so, can use .into to turn from tuple to Point3
            AnimationKeyframe {
                position: (5f32, 0f32, 0f32).into(),
                rotation: (45f32, 45f32, 0f32).into(),
                size: (500f32, 100f32, 100f32).into(),
            },
        ));
        // third frame
        animation.keyframes.push((
            8.0,
            AnimationKeyframe {
                position: (-3f32, -3f32, 0f32).into(),
                rotation: (-45f32, -45f32, 0f32).into(),
                size: (100f32, 50f32, 50f32).into(),
            },
        ));
        // final frame
        animation.keyframes.push((
            10.0,
            AnimationKeyframe {
                position: (0f32, 0f32, 0f32).into(),
                rotation: (0f32, 0f32, 0f32).into(),
                ..Default::default()
            },
        ));

        // compile the animation sequence and start it.
        animation.start().expect("Couldn't compile the animation");

        engine.update_loop(move |_, window, objects, _, _, _| {
            // animate the object
            animation.animate(objects, window.as_ref().unwrap().inner_size());
        })?;
    }

    Ok(())
}

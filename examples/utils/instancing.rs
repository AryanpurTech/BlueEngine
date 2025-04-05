/*
 * Blue Engine by Elham Aryanpur
 *
 * Triangle example using pre-defined shapes
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    Instance, Vector3,
    prelude::{Engine, ObjectSettings},
    primitive_shapes::triangle,
};

pub fn main() -> Result<(), blue_engine::error::Error> {
    // start the engine
    let mut engine = Engine::new()?;

    // create a triangle
    triangle(
        "Triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    // update the triangle
    let shape = engine
        .objects
        .get_mut("Triangle")
        .expect("Couldn't get the triangle");

    // set the position of the main triangle
    shape.set_position(Vector3::new(0f32, 0f32, -3f32));

    // add an instance
    shape.add_instance(Instance {
        position: Vector3::new(2f32, 1f32, -2f32),
        ..Default::default()
    });
    shape.add_instance(Instance {
        position: Vector3::new(2f32, -1f32, -2f32),
        ..Default::default()
    });
    shape.add_instance(Instance {
        position: Vector3::new(-2f32, 1f32, -2f32),
        ..Default::default()
    });
    shape.add_instance(Instance {
        position: Vector3::new(-2f32, -1f32, -2f32),
        ..Default::default()
    });

    // we manually update the instance buffer before the next frame starts
    // this is due to the object updates happening after every frame, hence
    // for the first frame, we need to update it ourselves.
    shape.update_instance_buffer(&mut engine.renderer);

    // run the loop as normal
    engine.update_loop(move |_, _, _, _, _, _| {})?;

    Ok(())
}

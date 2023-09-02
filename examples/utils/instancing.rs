/*
 * Blue Engine by Elham Aryanpur
 *
 * Triangle example using pre-defined shapes
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    header::{Engine, ObjectSettings},
    primitive_shapes::triangle,
    Instance,
};

pub fn main() {
    // start the engine
    let mut engine = Engine::new().expect("window not created");

    // create a triangle
    triangle(
        "Triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .unwrap();

    // update the triangle
    engine.objects.update_object("Triangle", |object| {
        // set the position of the main triangle
        object.set_position(0f32, 0f32, -3f32);

        // a function to make instance creation easier
        let create_instance = |x: f32, y: f32, z: f32| {
            Instance::new(
                [x, y, z].into(),
                [0f32, 0f32, 0f32].into(),
                [1f32, 1f32, 1f32].into(),
            )
        };

        // add an instance
        object.add_instance(create_instance(2f32, 1f32, -2f32));
        object.add_instance(create_instance(2f32, -1f32, -2f32));
        object.add_instance(create_instance(-2f32, 1f32, -2f32));
        object.add_instance(create_instance(-2f32, -1f32, -2f32));
    });

    // we manually update the instance buffer before the next frame starts
    // this is due to the object updates happening after every frame, hence
    // for the first frame, we need to update it ourselves.
    engine
        .objects
        .get_mut("Triangle")
        .expect("Couldn't get the triangle")
        .update_instance_buffer(&mut engine.renderer)
        .expect("Couldn't update instance buffer");

    // run the loop as normal
    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}

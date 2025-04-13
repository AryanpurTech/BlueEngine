/*
 * Blue Engine by Elham Aryanpur
 *
 * Resource Sharing example using same resource in multiple objects
 *
 * The license is same as the one on the root.
*/
use blue_engine::{Engine, ObjectSettings, TextureData, Vector3, primitive_shapes::square};

fn main() -> Result<(), blue_engine::error::Error> {
    // Start the engine
    let mut engine = Engine::new()?;

    // build your main object with the texture
    square(
        "main",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    // add the texture to the main object as normally would
    engine
        .objects
        .get_mut("main")
        .unwrap()
        // build a texture as an example of resource to be shared
        .set_texture(
            "background",
            TextureData::Path("examples/resources/BlueLogoDiscord.png".to_string()),
            blue_engine::TextureMode::Clamp,
            &mut engine.renderer,
        )?
        .set_position(Vector3::new(-1.5f32, 0f32, 0f32)); // set position to make it visible

    // create another object where you want to get resources shared with
    square(
        "alt",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    // here you can use `reference_texture` to reference the texture from the main object
    engine
        .objects
        .get_mut("alt")
        .expect("Error during copying texture of the main square")
        // setting position again to make it visible
        .set_position([1.5f32, 0f32, 0f32])
        .reference_texture("main");

    engine.update_loop(move |_| {})?;

    Ok(())
}

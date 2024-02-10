/*
 * Blue Engine by Elham Aryanpur
 *
 * Resource Sharing example using same resource in multiple objects
 *
 * The license is same as the one on the root.
*/
use blue_engine::{primitive_shapes::square, Engine, ObjectSettings, TextureData};

fn main() {
    // Start the engine
    let mut engine = Engine::new().expect("window not initialized");

    // build a texture as an example of resource to be shared
    let texture = engine
        .renderer
        .build_texture(
            "background",
            TextureData::Path("resources/BlueLogoDiscord.png".to_string()),
            blue_engine::TextureMode::Clamp,
        )
        .unwrap();

    // build your main object with the texture
    square(
        "main",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .expect("Error during creation of main square");

    // add the texture to the main object as normally would
    engine
        .objects
        .get_mut("main")
        .unwrap()
        .set_texture(texture)
        .expect("Error during inserting texture to the main square");
    // set position to make it visible
    engine
        .objects
        .get_mut("main")
        .expect("Error during setting the position of the main square")
        .set_position(-1.5f32, 0f32, 0f32);

    // create another object where you want to get resources shared with
    square(
        "alt",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .expect("Error during creation of alt square");

    // here you can use `reference_texture` to reference the texture from the main object
    engine
        .objects
        .get_mut("alt")
        .expect("Error during copying texture of the main square")
        .reference_texture("main");
    // setting position again to make it visible
    engine
        .objects
        .get_mut("alt")
        .expect("Error during setting the position of the alt square")
        .set_position(1.5f32, 0f32, 0f32);

    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}

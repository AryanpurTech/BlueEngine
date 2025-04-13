/*
 * Headless mode example of the Blue Engine. In this mode, the frames are
 * rendered without a window being available. However, texture are being
 * presented during the runtime as bytes format, so that it can be used
 * however it is needed.
 *
 * To use this feature, you MUST disable the window feature, which is turned
 * on by default, and enable headless mode. This feature does not work for
 * android yet.
 *
 * example in Cargo.toml:
 *
 * blue_engine = { version = "*", default-features = false, features = ["static_link", "debug", "headless"] }
 */

use blue_engine::{
    Engine, EngineSettings, ObjectSettings, image::ImageEncoder, primitive_shapes::triangle,
};

pub fn output_image_native(image_data: Vec<u8>, texture_dims: (usize, usize), path: &str) {
    let writer = std::fs::File::create(path).unwrap();
    let encoder = blue_engine::image::codecs::png::PngEncoder::new(writer);
    encoder
        .write_image(
            &image_data,
            texture_dims.0 as u32,
            texture_dims.1 as u32,
            blue_engine::image::ExtendedColorType::Rgba8,
        )
        .unwrap();

    println!("generated a frame!");

    // to be able to view the image. Remove it if you're testing it in other ways.
    std::process::exit(0);
}

fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new_config(EngineSettings {
        // The width and height must be respecting of the 256 padding
        height: 1024,
        width: 1024,
        ..Default::default()
    })?;

    triangle(
        "trig",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    // let instant = std::time::Instant::now();
    engine.update_loop(move |engine| {
        output_image_native(
            engine.renderer.headless_texture_data.clone(),
            (
                // since we do not have a window, the width and
                // height is taken from the configuration of the renderer
                engine.renderer.config.width as usize,
                engine.renderer.config.height as usize,
            ),
            "img.png",
        );
    })?;
    Ok(())
}

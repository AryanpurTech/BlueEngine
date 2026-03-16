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
use std::ops::ControlFlow;
use blue_engine_core::{RotateAmount, RotateAxis};
use blue_engine::{
    Engine, EngineSettings, ObjectSettings, image::ImageEncoder, primitive_shapes::{triangle, cube},
};
use blue_engine::wgpu::Limits;

pub fn output_image_native(image_data: &Vec<u8>, texture_dims: (usize, usize), path: &str) {
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

    //std::process::exit(0);
    // ^^ NO CALL TO EXIT HERE!
}

fn main() -> Result<(), blue_engine::error::Error> {
    let mut engine = Engine::new_config(EngineSettings {
        // The width and height must be respecting of the 256 padding
        height: 1024,
        width: 1024,
        limits: Limits {
            max_texture_dimension_1d: 4096,
            max_texture_dimension_2d: 4096,
            // ^^^ Needed otherwise engine creation fails on Raspberry Pi 5
            ..Default::default()
        },
        ..Default::default()
    })?;


    triangle(
        "Triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    engine
        .objects
        .get_mut("Triangle")
        .unwrap()
        .set_position((0f32, 0f32, 1f32))
        .set_scale((0.5f32, 0.5f32, 0.5f32))
        .set_color(1f32, 0f32, 0f32, 1f32);

    // ^^^ Results in a BLUE triangle

    cube(
        "Cube",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    engine
        .objects
        .get_mut("Cube")
        .unwrap()
        .set_position((0f32, 0f32, 0f32))
        .set_scale((1f32, 1f32, 1.0f32))
        .set_color(0f32, 0f32, 1f32, 1f32);

    // ^^^ Results in a RED cube

    let steps = 30;

    let mut frame_index = 0;
    let start_at = std::time::Instant::now();
    let mut last_frame_at = start_at;

    let _ = engine.update_loop(move |engine| {
        frame_index += 1;

        let rotate = 360f32 * (frame_index as f32 / steps as f32) ;
        println!("rotation: {}deg", rotate);
        engine
            .objects
            .get_mut("Cube")
            .unwrap()
            .rotate(RotateAmount::Degrees(rotate), RotateAxis::Z);

        engine
            .objects
            .get_mut("Triangle")
            .unwrap()
            .rotate(RotateAmount::Degrees(-rotate), RotateAxis::Y);

        output_image_native(
            &engine.renderer.headless_texture_data,
            (
                // since we do not have a window, the width and
                // height is taken from the configuration of the renderer
                engine.renderer.config.width as usize,
                engine.renderer.config.height as usize,
            ),
            format!("frame_{}.png", frame_index).as_str(),
        );

        let now = std::time::Instant::now();
        let elapsed_since_start = now - start_at;
        let frame_duration = now - last_frame_at;
        last_frame_at = now;
        println!("frame {}, elapsed: {}ms, frame_duration: {}ms", frame_index, elapsed_since_start.as_millis(), frame_duration.as_millis());

        if frame_index >= steps {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
        // ^^^ added a system to allow exiting from the update loop and exiting gracefully.
        //     see API changes to `update_loop`
    });

    let elapsed_since_start = start_at.elapsed();

    println!("Done!, took {}ms", elapsed_since_start.as_millis());
    // ^^^ this MUST be printed

    Ok(())
}

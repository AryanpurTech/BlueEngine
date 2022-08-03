use std::{ops::Mul, result};

use blue_engine::{
    gui,
    header::{
        uniform_type::Matrix, Engine, ObjectSettings, RotateAxis, ShaderSettings, TextureData,
        Vertex, WindowDescriptor,
    },
    primitive_shapes::{cube, square, triangle},
    style_block, // text::Text},
    utils::{default_resources::DEFAULT_MATRIX_4, flycamera::FlyCamera},
    PolygonMode,
    PowerPreference,
    Style,
};
use std::time::Duration;

fn main() {
    let mut engine = Engine::new(WindowDescriptor {
        width: 800,
        height: 600,
        title: "title",
        decorations: true,
        resizable: true,
        power_preference: PowerPreference::HighPerformance,
    })
    .expect("win");

    // ===============================

    /*let mut font = Text::new(
        include_bytes!("resource/JetBrainsMono-Medium.ttf"),
        25f32,
        &mut engine.renderer,
    )
    .unwrap();*/

    //let triangle_id = triangle(Some("Triangleee"), &mut engine, camera).unwrap();
    let window_size = engine.window.inner_size();
    let cube = cube(Some("CUBEE"), &mut engine).unwrap();
    cube.scale(0.3, 0.3, 0.3);
    let cube_index = cube.object_index;

    //let window_size = engine.window.inner_size();
    /*let change_texture = engine
    .renderer
    .build_and_append_texture(
        "name",
        TextureData::Bytes(include_bytes!("resource/BlueLogoDiscord.png").to_vec()),
        blue_engine::header::TextureMode::Clamp,
        //blue_engine::header::TextureFormat::PNG,
    )
    .unwrap();*/

    //let square = engine.get_object(square_id).unwrap();

    //square.change_color(0.0, 0.0, 1.0, 0.7).unwrap();
    //square.change_texture(change_texture);
    //square.resize(100.0, 100.0, 0.0, window_size);

    //let square = engine.objects.get_mut(square_id).unwrap();

    //square.no_stretch_update(&mut engine.renderer, engine.window.inner_size()).unwrap();
    //font.draw("Hello_World", (-100, 50), &mut engine).unwrap();

    let radius = 2f32;
    let start = std::time::SystemTime::now();
    let mut rotation = 0f32;
    let speed = -0.05;

    let mut fly_camera = FlyCamera::new(&mut engine.camera);
    let mut has_border = false;
    let mut val = 0f32;

    engine
        .update_loop(move |_, window, objects, (event, input), camera, ui| {
            ui.show_demo_window(&mut true);
            ui.show_default_style_editor();
            gui::Window::new("name").build(&ui, || {
                style_block(
                    if has_border {
                        vec![
                            Style::Config(gui::StyleVar::FrameBorderSize(10f32)),
                            Style::Color(gui::StyleColor::Border, [0f32, 0f32, 1f32, 1f32]),
                        ]
                    } else {
                        vec![Style::Config(gui::StyleVar::FrameBorderSize(0f32))]
                    },
                    || {
                        ui.button(if has_border {
                            "Text has border"
                        } else {
                            "Text doesn't have border"
                        });
                    },
                    ui,
                );
                if ui.button("label2") {
                    has_border = true;
                }

                gui::Drag::new("label")
                    .speed(0.2)
                    .range(0f32, 5f32)
                    .build(&ui, &mut val);
            });

            //fly_camera.update(camera, window, event, input);

            //cube.translate(1f32, 1f32, 1f32);

            /*let sprite = objects.get_mut(cube_index).unwrap();

            if input.key_held(blue_engine::KeyboardKeys::Up) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1 + speed,
                    sprite.position.2,
                    window_size,
                );
            }
            if input.key_held(blue_engine::KeyboardKeys::Down) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1 - speed,
                    sprite.position.2,
                    window_size,
                );
            }

            if input.key_held(blue_engine::KeyboardKeys::Left) {
                sprite.position(
                    sprite.position.0 - speed,
                    sprite.position.1,
                    sprite.position.2,
                    window_size,
                );
            }
            if input.key_held(blue_engine::KeyboardKeys::Right) {
                sprite.position(
                    sprite.position.0 + speed,
                    sprite.position.1,
                    sprite.position.2,
                    window_size,
                );
            }

            if input.key_held(blue_engine::KeyboardKeys::E) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1,
                    sprite.position.2 - speed,
                    window_size,
                );
            }
            if input.key_held(blue_engine::KeyboardKeys::Q) {
                sprite.position(
                    sprite.position.0,
                    sprite.position.1,
                    sprite.position.2 + speed,
                    window_size,
                );
            } */
        })
        .expect("Error during update loop");
}

use blue_engine::{
    gui,
    gui::StyleColor, // for colors
    gui::StyleVar,   // for configs
    style_block,     // A function that let you style a certain amount of choosing elements
    Engine,
    Style, // lets you choose to change config or color
    WindowDescriptor,
};

fn main() {
    let engine = Engine::new(WindowDescriptor::default()).unwrap();

    engine
        .update_loop(|_, _, _, _, _, ui| {
            // Create a ui window
            gui::Window::new("Styling Buttons").build(&ui, || {
                ui.set_window_font_scale(1.5f32);
                // ========= BLUE ======== //
                style_block(
                    vec![Style::Color(
                        StyleColor::Button, // We choose what things to style, e.g. here we choose button
                        [0f32, 0f32, 1f32, 1f32], // And the color data.
                    )], // In this vec we add styling
                    || {
                        // Here we add elements that we want to style
                        ui.button("Blue Button");

                        style_block(
                            vec![
                                Style::Config(StyleVar::FrameBorderSize(2f32)), // Can apply configs as such
                                Style::Color(StyleColor::Border, [1f32, 1f32, 1f32, 1f32]),
                            ],
                            || {
                                ui.button("Blue Button With Border");
                                // You can also nest them
                                // ========= GREEN ======== //
                                style_block(
                                    vec![Style::Color(
                                        StyleColor::Button,
                                        [0f32, 1f32, 0f32, 1f32],
                                    )],
                                    || {
                                        ui.button("Green Button");
                                    },
                                    &ui,
                                );
                            },
                            &ui,
                        );
                    },
                    &ui,
                );
            });
        })
        .unwrap();
}

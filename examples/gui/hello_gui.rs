/*
 * Blue Engine by Elham Aryanpur
 *
 * GUI example. Run with `gui` feature.
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    gui,
    header::{Engine, ObjectSettings, WindowDescriptor},
    primitive_shapes::triangle,
};

fn main() {
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    let trig = triangle(ObjectSettings::default(), &mut engine)
        .unwrap()
        .object_index;

    let mut color = [1f32, 1f32, 1f32, 1f32];

    engine
        .update_loop(move |_, _, objects, _, _, ui| {
            gui::Window::new("Control Triangle")
                .resizable(false)
                .build(&ui, || {
                    gui::ColorEdit::new("Pick a color", gui::EditableColor::Float4(&mut color))
                        .inputs(false)
                        .alpha(true)
                        .alpha_bar(true)
                        .build(&ui);
                });

            objects[trig]
                .change_color(color[0], color[1], color[2], color[3])
                .unwrap();
        })
        .expect("Error during update loop");
}

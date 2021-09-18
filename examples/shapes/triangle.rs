/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    definitions::{Engine, WindowDescriptor},
    objects::triangle,
};

fn main() {
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    let _ = triangle(Some("Triangle"), &mut engine).unwrap();

    engine
        .update_loop(move |_, _, _, _, _| {})
        .expect("Error during update loop");
}

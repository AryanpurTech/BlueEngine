/*
 * Signal example (could be used as template too) for Blue Engine
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    header::{Engine, ObjectSettings},
    primitive_shapes::triangle,
    Signal,
};

/// An example plugin with custom fields and operations
struct MyPlugin {
    counter: u32,
}
impl MyPlugin {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }

    fn reset(&mut self) {
        self.counter = 0;
    }
}
/// implemented the signals you need from the engine
impl Signal for MyPlugin {
    fn frame(
        &mut self,
        _renderer: &mut blue_engine::Renderer,
        _window: &blue_engine::Window,
        _objects: &mut blue_engine::ObjectStorage,
        _camera: &mut blue_engine::CameraContainer,
        input: &blue_engine::InputHelper,
        _encoder: &mut blue_engine::CommandEncoder,
        _view: &blue_engine::TextureView,
    ) {
        // operations that relate to the engine, and can call the
        // internal fields and functions you need.
        if input.key_pressed(blue_engine::KeyCode::Enter) {
            self.increment();
            println!("New counter value: {}", self.counter);
        }
    }
}

pub fn main() {
    // initialize the engine
    let mut engine = Engine::new().expect("win");

    // add your objects and anything else you need
    triangle(
        "Triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )
    .unwrap();

    // initialize your plugin
    let mut myplugin = MyPlugin::new();
    // do any init operation you want
    myplugin.reset();

    // add it to the engine's signals
    engine.signals.add_signal("my plugin", Box::new(myplugin));

    // thats it, the engine will now run it on every frame or wherever you requested.
    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}

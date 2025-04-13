/*
 * Signal example (could be used as template too) for Blue Engine
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    Signal,
    prelude::{Engine, ObjectSettings},
    primitive_shapes::triangle,
};

/// An example plugin with custom fields and operations
struct MyPlugin {
    counter: u32,
}
impl MyPlugin {
    fn new() -> Self {
        Self { counter: 0 }
    }

    fn _increment(&mut self) {
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
        _engine: &mut blue_engine_core::Engine,
        _encoder: &mut blue_engine_core::CommandEncoder,
        _view: &blue_engine_core::TextureView,
    ) {
        // operations that relate to the engine, and can call the
        // internal fields and functions you need.
        #[cfg(feature = "window")]
        if _engine
            .simple_input
            .key_pressed(blue_engine::KeyCode::Enter)
        {
            self._increment();
            println!("New counter value: {}", self.counter);
        }
    }
}

pub fn main() -> Result<(), blue_engine::error::Error> {
    // initialize the engine
    let mut engine = Engine::new()?;

    // add your objects and anything else you need
    triangle(
        "Triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
    )?;

    // initialize your plugin
    let mut myplugin = MyPlugin::new();
    // do any init operation you want
    myplugin.reset();

    // add it to the engine's signals
    engine.signals.add_signal("my plugin", Box::new(myplugin));

    // thats it, the engine will now run it on every frame or wherever you requested.
    engine.update_loop(move |_| {})?;

    Ok(())
}

<img src="https://raw.githubusercontent.com/AryanpurTech/BlueEngineDocs/master/resources/logo_3d.gif" loop=infinite width="100%" />

[![Rust Linux](https://github.com/AryanpurTech/BlueEngine/actions/workflows/rust-linux.yml/badge.svg)](https://github.com/AryanpurTech/BlueEngine/actions/workflows/rust-linux.yml)
[![Rust Windows](https://github.com/AryanpurTech/BlueEngine/actions/workflows/rust-win.yml/badge.svg)](https://github.com/AryanpurTech/BlueEngine/actions/workflows/rust-win.yml)
[![Rust MacOS](https://github.com/AryanpurTech/BlueEngine/actions/workflows/rust-osx.yml/badge.svg)](https://github.com/AryanpurTech/BlueEngine/actions/workflows/rust-osx.yml)

Make sure to use latest Rust version, as the engine is always kept up to date.

## About

Blue Engine is a general-purpose, easy-to-use, extendable, and portable graphics engine written in rust. The engine can run on many popular back-end APIs including Vulkan, D3D-12, GL-ES 3, and Metal as well as Windows, Linux, Mobile, and OSX to ensure cross-platform compatibility.

Hello World:

```rust
use blue_engine::{
    header::{
        Engine, ObjectSettings, WindowDescriptor
    },
    primitive_shapes::triangle
};

fn main() {
    // initialize the engine
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    // create a triangle
    triangle("my triangle", ObjectSettings::default(), &mut engine.renderer, &mut engine.objects).unwrap();

    // run the engine
    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}
```

* [Join our discord server](https://discord.gg/s7xsj9q)

* [WIP] [Documentation](https://aryanpurtech.github.io/BlueEngineDocs/)

* Check out the [examples](https://github.com/AryanpurTech/BlueEngine/tree/master/examples) folder to get a sense of how things are done

* Check out the [utilities library](https://github.com/AryanpurTech/BlueEngineUtilities) for extra functionality with the engine

* Check out the [editor](https://github.com/rustylabs/blue_flame)

*the credits to the image on top: NotPB*

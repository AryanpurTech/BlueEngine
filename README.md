<img src="https://raw.githubusercontent.com/AryanpurTech/BlueEngineDocs/master/resources/logo_3d.gif" loop=infinite width="100%" />

[![Build](https://github.com/AryanpurTech/BlueEngine/actions/workflows/build.yml/badge.svg)](https://github.com/AryanpurTech/BlueEngine/actions/workflows/build.yml)
[![rust-clippy analyze](https://github.com/AryanpurTech/BlueEngine/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/AryanpurTech/BlueEngine/actions/workflows/rust-clippy.yml)
[![Static Badge](https://img.shields.io/badge/Join-The_Discord-blue?style=flat&logo=discord&color=blue)](https://discord.com/invite/s7xsj9q)
[![Static Badge](https://img.shields.io/badge/Read_The_Docs-blue?style=flat&logo=docsdotrs&color=%23000000)](https://docs.rs/blue_engine)

Make sure to use latest Rust version, as the engine is always kept up to date.

## About

Blue Engine is a general-purpose, easy-to-use, extendable, and portable graphics engine written in rust. The engine can run on many popular back-end APIs including Vulkan, D3D-12, GL-ES 3, and Metal as well as Windows, Linux, Mobile, and OSX to ensure cross-platform compatibility.

Hello World:

```rust
use blue_engine::{
    prelude::{ Engine, ObjectSettings },
    primitive_shapes::triangle
};

fn main() {
    // initialize the engine
    let mut engine = Engine::new().expect("engine couldn't be initialized");

    // create a triangle
    triangle("my triangle", ObjectSettings::default(), &mut engine.renderer, &mut engine.objects);

    // run the engine
    engine
        .update_loop(move |_, _, _, _, _, _| {})
        .expect("Error during update loop");
}
```

- [WIP] [Guide](https://aryanpurtech.github.io/BlueEngineDocs/)

- Check out the [examples](https://github.com/AryanpurTech/BlueEngine/tree/master/examples) folder to get a sense of how things are done

- Check out the [utilities library](https://github.com/AryanpurTech/BlueEngineUtilities) for extra functionality with the engine

_the credits to the image on top: NotPB_

_the development might seem slow sometimes, its due to multiple repositories being handled and due to my education taking a large chunk of my time. The project isn't dead, just slow._

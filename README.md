<img src="https://raw.githubusercontent.com/AryanpurTech/BlueEngineDocs/master/resources/logo_3d.gif" loop=infinite width="100%" />

[![Cross Platform Build](https://github.com/AryanpurTech/BlueEngine/actions/workflows/release.yml/badge.svg)](https://github.com/AryanpurTech/BlueEngine/actions/workflows/release.yml)
[![Static Badge](https://img.shields.io/badge/Join-The_Discord-blue?style=flat&logo=discord&color=blue)](https://discord.com/invite/s7xsj9q)
[![Static Badge](https://img.shields.io/badge/Read_The_Docs-blue?style=flat&logo=docsdotrs&color=%23000000)](https://docs.rs/blue_engine)

Make sure to use latest Rust version, as the engine is always kept up to date.

## About

Blue Engine is a general-purpose, easy-to-use, extendable, and portable graphics engine written in rust. The engine can run on many popular back-end APIs including Vulkan, D3D-12, GL-ES, and Metal as well as Windows, Linux, Mobile, and OSX to ensure cross-platform compatibility.

Hello World:

```rust
use blue_engine::{
    prelude::{ Engine, ObjectSettings },
    primitive_shapes::triangle
};

fn main() -> Result<(), blue_engine::error::Error> {
    // initialize the engine
    let mut engine = Engine::new()?;

    // create a triangle
    triangle("my triangle", ObjectSettings::default(), &mut engine.renderer, &mut engine.objects)?;

    // run the engine
    engine
        .update_loop(move |_| {})?;

    Ok(())
}
```

- [WIP] [Guide](https://docs.rs/blue_engine/latest/blue_engine/)

- Check out the [examples](https://github.com/AryanpurTech/BlueEngine/tree/master/examples) folder to get a sense of how things are done

- Check out the [utilities library](https://crates.io/crates/blue_engine_utilities) for extra functionality with the engine

_the credits to the image on top: NotPB_

_The project isn't dead, just the development might seem slow sometimes._

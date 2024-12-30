/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

#![warn(missing_docs)]
#![allow(clippy::needless_doctest_main)]

//! <img src="https://raw.githubusercontent.com/AryanpurTech/BlueEngineDocs/master/resources/logo_3d.gif" loop=infinite width="100%" />
//!
//! # Blue Engine
//!
//! Blue Engine is an easy to use, portable, and extendable/customizable graphics engine. Here
//! lives the documentation for the engine.
//!
//! ## Setup
//!
//! The setup and installation details live in the project's [guide](https://aryanpurtech.github.io/BlueEngineDocs/).
//! A basic program in Blue Engine is as follow:
//!
//! ## Example
//!
//! ```rust
//! use blue_engine::{
//!     header::{ Engine, ObjectSettings },
//!     primitive_shapes::triangle
//! };
//!
//! fn main() {
//!     // initialize the engine
//!     let mut engine = Engine::new().expect("engine couldn't be initialized");
//!
//!     // create a triangle
//!     triangle("my triangle", ObjectSettings::default(), &mut engine.renderer, &mut engine.objects).unwrap();
//!
//!    // run the engine
//!    engine
//!        .update_loop(move |_, _, _, _, _, _| {})
//!        .expect("Error during update loop");
//! }
//! ```
//!
//! ## Utilities
//!
//! This crate is the core of the engine, but there is also [utilities crate](https://github.com/AryanpurTech/BlueEngineUtilities)
//! which have a lot of utilities for the engine such as lighting, physics, etc.
//!
//! ## Guide for code navigation
//!
//! The code of the engine is organized in a rather different manner than traditional in the
//! language. There are inspirations from other languages to make it easier to navigate the
//! project.
//!
//! ## Older hardware
//!
//! The engine uses WGPU under the hood for rendering. WGPU by nature is designed for modern hardware, so if you have or want to
//! target older hardware, you might need to add a couple more things to WindowDescriptor during Engine::new_config:
//!
//! 1) set a backend that targets your older hardware, such as GL using the backends field: `backend: blue_engine::wgpu::Backends::GL`
//! 2) experiement with the limits field, which describes what features you need. `limits: blue_engine::wgpu::Limits::default()`. there
//!    are three options for limits: `default` for normal hardware, `downlevel_defaults` which are compatible with GLES-3 and D3D-11, or
//!    `downlevel_webgl2_defaults` which is also compatible with WebGL2, and the lowest level for limits and can support very old hardware.
//!
//! with these two changes, hopefully you can get Blue Engine to run on older hardware. If not, please let me know so I can help you further.

pub(crate) mod definition;
/// interal error definitions of the engine
pub mod error;
/// contains all the declarations such as structs, exports, enums, ...
pub mod header;
/// contains the definition for Object type, which is a type that make it easier to manage data for rendering.
pub mod objects;
/// contains definition for some 2D and 3D shapes. They are basic shapes and
/// can be used as examples of how to create your own content.
pub mod primitive_shapes;
/// contains definition for rendering part of the engine.
pub mod render;
/// Utilities for the engine (soon moving to it's own
/// [crate](https://github.com/AryanpurTech/BlueEngineUtilities)).
pub mod utils;
/// contains definition for creation of window and instance creation.
pub mod window;
#[doc(inline)]
pub use crate::header::*;

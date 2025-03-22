/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

#![warn(missing_docs)]
#![allow(clippy::needless_doctest_main)]

//!

pub(crate) mod definition;
/// interal error definitions of the engine
pub mod error;
/// contains the definition for Object type, which is a type that make it easier to manage data for rendering.
pub mod objects;
/// contains all the declarations such as structs, exports, enums, ...
pub mod prelude;
/// contains definition for some 2D and 3D shapes. They are basic shapes and
/// can be used as examples of how to create your own content.
pub mod primitive_shapes;
/// contains definition for rendering part of the engine.
pub mod render;
/// Utilities for the engine (soon moving to it's own
/// [crate](https://github.com/AryanpurTech/BlueEngineUtilities)).
pub mod utils;
/// contains definition for 2D and 3D vectors.
pub mod vector;
/// contains definition for creation of window and instance creation.
pub mod window;
#[doc(inline)]
pub use crate::prelude::*;

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
/// contains definition for rendering part of the engine.
pub mod render;
/// Utilities for the engine (soon moving to it's own
/// [crate](https://github.com/AryanpurTech/BlueEngineUtilities)).
pub mod utils;
pub use utils::*;
/// contains definition for creation of window and instance creation.
pub mod window;
#[doc(inline)]
pub use crate::prelude::*;
/// contains defintions of top level functionality of the Engine
pub mod engine;

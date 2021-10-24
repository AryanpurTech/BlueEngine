/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

extern crate nalgebra_glm as glm;

#[cfg(feature = "debug")]
pub fn debug<T: std::fmt::Debug>(_debug_text: T) {
    println!("{:?}", _debug_text);
}
#[cfg(not(feature = "debug"))]
pub fn debug<T: std::fmt::Debug>(_debug_text: _debug_text) {}

pub(crate) mod definition;
pub mod header;
pub mod objects;
pub mod render;
pub mod utils;
pub mod window;

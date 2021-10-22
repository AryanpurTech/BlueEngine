/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

extern crate nalgebra_glm as glm;

#[cfg(target_feature = "debug")]
pub fn debug<T>(debug_text: T) {
    println!("{:?}", T);
}
#[cfg(not(target_feature = "debug"))]
pub fn debug<T>(_debug_text: T) {}

pub(crate) mod definition;
pub mod header;
pub mod objects;
pub mod render;
pub mod utils;
pub mod window;

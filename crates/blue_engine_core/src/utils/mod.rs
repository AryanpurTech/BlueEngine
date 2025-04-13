/// The camera utilities
pub mod camera;
/// Input wrapping
#[cfg(all(feature = "window", not(feature = "headless")))]
mod current_input;
/// default resources used in the engine
pub mod default_resources;
/// input helper
#[cfg(all(feature = "window", not(feature = "headless")))]
pub mod winit_input_helper;

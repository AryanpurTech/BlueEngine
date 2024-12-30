#![allow(missing_docs)]

use thiserror::Error;

/// This enum contains errors that occurred in the engine.
#[derive(Error, Debug)]
pub enum Error {
    // ===== WINIT
    /// Errors that occur throughout the update_loop
    #[error("An error occurred with the update loop: {0}")]
    UpdateLoopError(#[from] winit::error::EventLoopError),

    // ===== WGPU
    #[error("Failed to find an appropriate adapter")]
    AdapterNotFound,
    #[error("Failed to find an appropriate device for rendering")]
    DeviceNotFound(#[from] wgpu::RequestDeviceError),

    // ===== Image
    #[error("Failed to load the texture data from given source")]
    LoadingTextureDataError(#[from] image::error::ImageError),

    #[error("{0}")]
    Custom(String),
}

pub use downcast;
pub use glam;
pub use image;
pub use wgpu;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit;

pub use wgpu::Backends;
pub use wgpu::CommandEncoder;
pub use wgpu::LoadOp;
pub use wgpu::MemoryHints;
pub use wgpu::Operations;
pub use wgpu::RenderPassColorAttachment;
pub use wgpu::RenderPassDescriptor;
pub use wgpu::TextureView;

#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::dpi::*;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::event::DeviceEvent;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::event::ElementState;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::event::Event;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::event::KeyEvent;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::event::MouseButton;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::event::WindowEvent;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::event_loop::EventLoop;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::keyboard::Key;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::keyboard::KeyCode;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use winit::window::Fullscreen;

// Math types
/// A 2-dimensional vector.
pub type Vector2 = glam::Vec2;
/// A 3-dimensional vector.
pub type Vector3 = glam::Vec3;
/// A 4-dimensional vector.
pub type Vector4 = glam::Vec4;
/// A 2x2 column major matrix.
///
/// SIMD vector types are used for storage on supported platforms.
///
/// This type is 16 byte aligned.
pub type Matrix2 = glam::Mat2;
/// A 3x3 column major matrix.
///
/// This 3x3 matrix type features convenience methods for creating and using linear and
/// affine transformations.
pub type Matrix3 = glam::Mat3;
/// A 4x4 column major matrix.
///
/// This 4x4 matrix type features convenience methods for creating and using affine transforms and perspective projections.
pub type Matrix4 = glam::Mat4;
/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to floating point "error creep" which can occur when successive quaternion operations are applied.
///
/// SIMD vector types are used for storage on supported platforms.
///
/// This type is 16 byte aligned.
pub type Quaternion = glam::Quat;

/// Input helper
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use crate::utils::winit_input_helper::WinitInputHelper as InputHelper;
pub use bytemuck::Pod;
pub use bytemuck::Zeroable;

/// Depth Format
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
/// Shaders are programs that runs on the GPU
pub type Shaders = wgpu::RenderPipeline;
/// Uniform Buffers are small amount of data that are sent from CPU to GPU
pub type UniformBuffers = wgpu::BindGroup;
/// Textures are image data that are sent to GPU to be set to a surface
pub type Textures = wgpu::BindGroup;
/// Primitive type the input mesh is composed of.
pub type ShaderPrimitive = wgpu::PrimitiveTopology;
/// Format of indices used with pipeline.
pub type IndexFormat = wgpu::IndexFormat;
/// Vertex winding order which classifies the "front" face of a triangle.
pub type FrontFace = wgpu::FrontFace;
/// Face of a vertex.
pub type CullMode = wgpu::Face;
/// Type of drawing mode for polygons
pub type PolygonMode = wgpu::PolygonMode;
/// Power Preference when choosing a physical adapter.
pub type PowerPreference = wgpu::PowerPreference;

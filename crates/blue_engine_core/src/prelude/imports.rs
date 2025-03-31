pub use downcast;
pub use image;
pub use wgpu;
pub use winit;

pub use wgpu::Backends;
pub use wgpu::CommandEncoder;
pub use wgpu::LoadOp;
pub use wgpu::MemoryHints;
pub use wgpu::Operations;
pub use wgpu::RenderPassColorAttachment;
pub use wgpu::RenderPassDescriptor;
pub use wgpu::TextureView;

pub use winit::dpi::*;
pub use winit::event::DeviceEvent;
pub use winit::event::ElementState;
pub use winit::event::Event;
pub use winit::event::KeyEvent;
pub use winit::event::MouseButton;
pub use winit::event::WindowEvent;
pub use winit::event_loop::EventLoop;
pub use winit::keyboard::Key;
pub use winit::keyboard::KeyCode;
pub use winit::window::Fullscreen;

// Math types
pub type Vector2 = glam::Vec2;
pub type Vector3 = glam::Vec3;
pub type Vector4 = glam::Vec4;
pub type Matrix2 = glam::Mat2;
pub type Matrix3 = glam::Mat3;
pub type Matrix4 = glam::Mat4;
pub type Quaternion = glam::Quat;

/// Input helper
pub use crate::utils::winit_input_helper::WinitInputHelper as InputHelper;
pub use bytemuck::Pod;
pub use bytemuck::Zeroable;

pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub type Shaders = wgpu::RenderPipeline;
pub type UniformBuffers = wgpu::BindGroup;
pub type Textures = wgpu::BindGroup;
pub type ShaderPrimitive = wgpu::PrimitiveTopology;
pub type IndexFormat = wgpu::IndexFormat;
pub type FrontFace = wgpu::FrontFace;
pub type CullMode = wgpu::Face;
pub type PolygonMode = wgpu::PolygonMode;
pub type PowerPreference = wgpu::PowerPreference;

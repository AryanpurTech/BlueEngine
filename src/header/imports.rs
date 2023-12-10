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

/// Pod trait for custom uniform buffer structure
pub use bytemuck::Pod;
/// Zeroable trait for custom uniform buffer structure
pub use bytemuck::Zeroable;

/// Backends
pub use wgpu::Backends;
/// Encoder from wgpu
pub use wgpu::CommandEncoder;
pub use wgpu::LoadOp;
pub use wgpu::Operations;
pub use wgpu::RenderPassColorAttachment;
pub use wgpu::RenderPassDescriptor;
/// Surface Texture
pub use wgpu::TextureView;

/// all of downcast
pub use downcast;
/// all of image
pub use image;
/// all of nalgebra_glm
pub use nalgebra_glm as glm;
/// all of wgpu
pub use wgpu;
/// all of winit
pub use winit;
/// WindowSize
pub use winit::dpi::*;
/// Device Events
pub use winit::event::DeviceEvent;
/// Element State
pub use winit::event::ElementState;
/// Winit Events
pub use winit::event::Event;
/// Keyboard input identifier
pub use winit::event::KeyEvent;
/// The mouse button identifier
pub use winit::event::MouseButton;
/// WindowEvents
pub use winit::event::WindowEvent;
/// Event Loop
pub use winit::event_loop::EventLoop;
/// Keyboard keys identifier
pub use winit::keyboard::Key;
pub use winit::keyboard::KeyCode;
/// Fullscreen enum
pub use winit::window::Fullscreen;
/// Window export from winit
pub use winit::window::Window;
/// Input helper
pub use winit_input_helper::WinitInputHelper as InputHelper;

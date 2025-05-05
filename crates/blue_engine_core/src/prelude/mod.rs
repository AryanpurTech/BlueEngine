use downcast::{Any, downcast};
/// re-exports from dependencies that are useful
pub mod imports;
pub use imports::*;
/// contains definition for some 2D and 3D shapes. They are basic shapes and
/// can be used as examples of how to create your own content.
pub mod primitive_shapes;
pub use crate::camera::{Camera, CameraContainer, Projection};
pub use crate::definition::{
    Pipeline, PipelineData, ShaderSettings, TextureData, TextureMode, VertexBuffers,
    pixel_to_cartesian,
};
pub use crate::engine::{Engine, EngineSettings};
pub use crate::objects::{
    Instance, InstanceRaw, Object, ObjectSettings, ObjectStorage, RotateAmount, RotateAxis,
};
pub use crate::render::Renderer;
#[cfg(all(feature = "window", not(feature = "headless")))]
pub use crate::window::Window;

/// The uint type used for indices and more
#[cfg(not(feature = "u32"))]
pub type UnsignedIntType = u16;
#[cfg(feature = "u32")]
pub type UnsignedIntType = u32;

///
pub mod macros {
    macro_rules! impl_deref {
        ($struct:ty,$type:ty) => {
            impl std::ops::Deref for $struct {
                type Target = $type;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl std::ops::DerefMut for $struct {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        };
    }

    macro_rules! impl_deref_field {
        ($struct:ty,$type:ty,$field:ident) => {
            impl std::ops::Deref for $struct {
                type Target = $type;

                fn deref(&self) -> &Self::Target {
                    &self.$field
                }
            }
            impl std::ops::DerefMut for $struct {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.$field
                }
            }
        };
    }

    pub(crate) use impl_deref;
    pub(crate) use impl_deref_field;
}

/// Will contain all details about a vertex and will be sent to GPU
// Will be turned to C code and sent to GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// Contains position data for the vertex in 3D space
    pub position: [f32; 3],
    /// Contains uv position data for the vertex
    pub uv: [f32; 2],
    /// Contains the normal face of the vertex
    pub normal: [f32; 3],
}
impl Vertex {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    // This should be replaced with `std::mem::size_of::<Vector3>() as wgpu::BufferAddress`
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
unsafe impl Send for Vertex {}
unsafe impl Sync for Vertex {}

/// Allows all events to be fetched directly, making it easier to add custom additions to the engine.
pub trait Signal: Any {
    /// This is ran as soon as the engine is properly initialized and all components are ready
    #[allow(clippy::too_many_arguments)]
    fn init(&mut self, _engine: &mut crate::Engine) {}

    /// This is ran at the device events when available
    #[allow(clippy::too_many_arguments)]
    #[cfg(all(feature = "window", not(feature = "headless")))]
    fn device_events(&mut self, _engine: &mut crate::Engine, _events: &crate::DeviceEvent) {}

    /// This is ran at the window events when available
    #[allow(clippy::too_many_arguments)]
    #[cfg(all(feature = "window", not(feature = "headless")))]
    fn window_events(&mut self, _engine: &mut crate::Engine, _events: &crate::WindowEvent) {}

    /// ran before the frame is rendered
    #[allow(clippy::too_many_arguments)]
    fn frame(
        &mut self,
        _engine: &mut crate::Engine,
        _encoder: &mut crate::CommandEncoder,
        _view: &crate::TextureView,
    ) {
    }
}
// The engine needs to know the functions of Signal to do things internally,
// so we use downcast and not the std::any::Any
downcast!(dyn Signal);

/// Handles the live events in the engine
pub struct SignalStorage {
    /// list of events with key and the event
    pub events: Vec<(String, Box<dyn Signal>)>,
}

/// To hold the width and height of the engine frames
pub type WindowSize = (u32, u32);

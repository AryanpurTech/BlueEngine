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
pub use crate::objects::{
    Instance, InstanceRaw, Object, ObjectSettings, ObjectStorage, RotateAmount, RotateAxis,
};
pub use crate::render::Renderer;
pub use crate::window::{EngineSettings, Window};

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

/// The engine is the main starting point of using the Blue Engine.
/// Everything that runs on Blue Engine will be under this struct.
/// The structure of engine is monolithic, but the underlying data and the way it works is not.
/// It gives a set of default data to work with,
/// but also allow you to go beyond that and work as low level as you wish to.
///
/// You can also use the Engine to build you own custom structure the way you wish for it to be.
/// Possibilities are endless!
///
/// To start using the Blue Engine, you can start by creating a new Engine like follows:
/// ```
/// use blue_engine::prelude::{Engine, EngineSettings};
///
/// fn main() {
///     let engine = Engine::new().expect("Couldn't create the engine");
/// }
/// ```
/// The EngineSettings simply holds what features you would like for your window.
/// If you are reading this on later version of
/// the engine, you might be able to even run the engine in headless mode
/// meaning there would not be a need for a window and the
/// renders would come as image files.
///
/// If you so wish to have a window, you would need to start a window update loop.
/// The update loop of window runs a frame every few millisecond,
/// and gives you details of what is happening during this time, like input events.
/// You can also modify existing parts of the engine during
/// this update loop, such as changing camera to look differently,
/// or creating a new object on the scene, or even changing window details!
///
/// The update loop is just a method of the Engine struct
/// that have one argument which is a callback function.
/// ```
///
/// ```
/// [THE DATA HERE IS WORK IN PROGRESS!]
pub struct Engine {
    /// The renderer does exactly what it is called.
    /// It works with the GPU to render frames according to the data you gave it.
    pub renderer: Renderer,
    /// The event_loop handles the events of the window and inputs.
    ///
    /// #### USED INTERNALLY
    pub event_loop_control_flow: crate::winit::event_loop::ControlFlow,
    /// The window handles everything about window and inputs.
    /// This includes ability to modify window and listen toinput devices for changes.
    ///
    /// ### The window is not available before update_loop.
    pub window: Window,
    /// The object system is a way to make it easier to work with the engine.
    /// Obviously you can work without it, but it's for those who
    /// do not have the know-how, or wish to handle all the work of rendering data manually.
    pub objects: ObjectStorage,
    /// The camera handles the way the scene looks when rendered.
    /// You can modify everything there is to camera through this.
    pub camera: CameraContainer,
    /// Handles all engine plugins
    pub signals: SignalStorage,

    /// holds the update_loop function
    ///
    /// #### USED INTERNALLY
    pub update_loop: Option<
        Box<
            dyn 'static
                + FnMut(
                    // Core
                    &mut Engine,
                ),
        >,
    >,

    /// input events
    ///
    /// #### USED INTERNALLY
    pub simple_input: crate::utils::winit_input_helper::WinitInputHelper,
}
unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

/// Allows all events to be fetched directly, making it easier to add custom additions to the engine.
pub trait Signal: Any {
    /// This is ran as soon as the engine is properly initialized and all components are ready
    #[allow(clippy::too_many_arguments)]
    fn init(&mut self, _engine: &mut crate::Engine) {}

    /// This is ran at the device events when available
    #[allow(clippy::too_many_arguments)]
    fn device_events(&mut self, _engine: &mut crate::Engine, _events: &crate::DeviceEvent) {}

    /// This is ran at the window events when available
    #[allow(clippy::too_many_arguments)]
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

/// A unified way to handle strings
pub trait StringBuffer: StringBufferTrait + Clone {}
/// A trait for [StringBuffer]
pub trait StringBufferTrait {
    /// Returns the string as &[`str`]
    fn as_str(&self) -> &str;
    /// Returns the string as [`String`]
    fn as_string(&self) -> String;
    /// Returns Arc<str> for ease of computation
    fn as_arc(&self) -> std::sync::Arc<str>;
}

impl StringBufferTrait for String {
    fn as_str(&self) -> &str {
        self.as_ref()
    }
    fn as_string(&self) -> String {
        self.clone()
    }
    fn as_arc(&self) -> std::sync::Arc<str> {
        self.as_str().into()
    }
}
impl StringBuffer for String {}
impl StringBufferTrait for &str {
    fn as_str(&self) -> &str {
        self
    }
    fn as_string(&self) -> String {
        self.to_string()
    }
    fn as_arc(&self) -> std::sync::Arc<str> {
        self.as_str().into()
    }
}
impl StringBuffer for &str {}

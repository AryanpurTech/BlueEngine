/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is Apache-2.0
*/

/// re-exports from dependencies that are useful
pub mod imports;
/// few commonly used uniform buffer structures
pub mod uniform_buffer;
pub use imports::*;
pub use uniform_buffer::*;

use downcast::{downcast, Any};

/// The uint type used for indices and more
#[cfg(feature = "u16")]
pub type UnsignedIntType = u16;
#[cfg(feature = "u32")]
pub type UnsignedIntType = u32;

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

/// Objects make it easier to work with Blue Engine, it automates most of work needed for
/// creating 3D objects and showing them on screen. A range of default objects are available
/// as well as ability to customize each of them and even create your own! You can also
/// customize almost everything there is about them!
pub struct Object {
    /// Give your object a name, which can help later on for debugging.
    pub name: std::sync::Arc<str>,
    /// A list of Vertex
    pub vertices: Vec<Vertex>,
    /// A list of indices that dictates the order that vertices appear
    pub indices: Vec<UnsignedIntType>,
    /// Describes how to uniform buffer is structures
    pub uniform_layout: wgpu::BindGroupLayout,
    /// Pipeline holds all the data that is sent to GPU, including shaders and textures
    pub pipeline: Pipeline,
    /// List of instances of this object
    pub instances: Vec<Instance>,
    /// instance buffer
    pub instance_buffer: wgpu::Buffer,
    /// Dictates the size of your object in relation to the world
    pub size: glm::Vec3,
    /// Dictates the position of your object in pixels
    pub position: glm::Vec3,
    /// Dictates the rotation of your object
    pub rotation: glm::Vec3,
    // flags the object to be updated until next frame
    pub(crate) changed: bool,
    /// Transformation matrices helps to apply changes to your object, including position, orientation, ...
    /// Best choice is to let the Object system handle it
    pub position_matrix: nalgebra_glm::Mat4,
    /// Transformation matrices helps to apply changes to your object, including position, orientation, ...
    /// Best choice is to let the Object system handle it
    pub scale_matrix: nalgebra_glm::Mat4,
    /// Transformation matrices helps to apply changes to your object, including position, orientation, ...
    /// Best choice is to let the Object system handle it
    pub rotation_matrix: nalgebra_glm::Mat4,
    /// Transformation matrix, but inversed
    pub inverse_transformation_matrix: crate::uniform_type::Matrix,
    /// The main color of your object
    pub color: crate::uniform_type::Array4,
    /// A struct making it easier to manipulate specific parts of shader
    pub shader_builder: crate::objects::ShaderBuilder,
    /// Shader settings
    pub shader_settings: ShaderSettings,
    /// Camera have any effect on the object?
    pub camera_effect: Option<std::sync::Arc<str>>,
    /// Uniform Buffers to be sent to GPU. These are raw and not compiled for GPU yet
    pub uniform_buffers: Vec<wgpu::Buffer>,
    /// Should be rendered or not
    pub is_visible: bool,
    /// Objects with higher number get rendered later and appear "on top" when occupying the same space
    pub render_order: usize,
}
unsafe impl Send for Object {}
unsafe impl Sync for Object {}

/// Extra settings to customize objects on time of creation
#[derive(Debug, Clone)]
pub struct ObjectSettings {
    /// Should it be affected by camera?
    pub camera_effect: Option<std::sync::Arc<str>>,
    /// Shader Settings
    pub shader_settings: ShaderSettings,
}
impl Default for ObjectSettings {
    fn default() -> Self {
        Self {
            camera_effect: Some("main".into()),
            shader_settings: ShaderSettings::default(),
        }
    }
}
unsafe impl Send for ObjectSettings {}
unsafe impl Sync for ObjectSettings {}

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
/// use blue_engine::header::{Engine, WindowDescriptor};
///
/// fn main() {
///     let engine = Engine::new().expect("Couldn't create the engine");
/// }
/// ```
/// The WindowDescriptor simply holds what features you would like for your window.
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
    #[allow(clippy::type_complexity)]
    pub update_loop: Option<
        Box<
            dyn 'static
                + FnMut(
                    // Core
                    &mut Renderer,
                    &mut Window,
                    &mut ObjectStorage,
                    &crate::utils::winit_input_helper::WinitInputHelper,
                    &mut CameraContainer,
                    &mut crate::SignalStorage,
                ),
        >,
    >,

    /// input events
    ///
    /// #### USED INTERNALLY
    pub input_events: crate::utils::winit_input_helper::WinitInputHelper,
}
unsafe impl Send for Engine {}
unsafe impl Sync for Engine {}

/// Container for pipeline values. Each pipeline takes only 1 vertex shader,
/// 1 fragment shader, 1 texture data, and optionally a vector of uniform data.
#[derive(Debug)]
pub struct Pipeline {
    /// the shader buffer that's sent to the gpu
    pub shader: PipelineData<crate::Shaders>,
    /// The vertex buffer that's sent to the gpu. This includes indices as well
    pub vertex_buffer: PipelineData<VertexBuffers>,
    /// The texture that's sent to the gpu.
    pub texture: PipelineData<crate::Textures>,
    /// the Uniform buffers that are sent to the gpu
    pub uniform: PipelineData<Option<crate::UniformBuffers>>,
}
unsafe impl Send for Pipeline {}
unsafe impl Sync for Pipeline {}

/// Container for pipeline data. Allows for sharing resources with other objects
#[derive(Debug)]
pub enum PipelineData<T> {
    /// No data, just a reference to a buffer
    Copy(String),
    /// The actual data
    Data(T),
}

/// Container for vertex and index buffer
#[derive(Debug)]
pub struct VertexBuffers {
    /// An array of vertices. A vertex is a point in 3D space containing
    /// an X, Y, and a Z coordinate between -1 and +1
    pub vertex_buffer: wgpu::Buffer,
    /// An array of indices. Indices are a way to reuse vertices,
    /// this in turn helps greatly in reduction of amount of vertices needed to be sent to the GPU
    pub index_buffer: wgpu::Buffer,
    /// The length of the vertex buffer
    pub length: u32,
}
unsafe impl Send for VertexBuffers {}
unsafe impl Sync for VertexBuffers {}

/// Main renderer class. this will contain all methods and data related to the renderer
#[derive(Debug)]
pub struct Renderer {
    /// A [`wgpu::Surface`] represents a platform-specific surface
    /// (e.g. a window) onto which rendered images may be presented.
    pub surface: Option<wgpu::Surface<'static>>,
    /// Context for all of the gpu objects
    pub instance: wgpu::Instance,
    /// Handle to a physical graphics and/or compute device.
    #[allow(unused)]
    pub adapter: wgpu::Adapter,
    /// Open connection to a graphics and/or compute device.
    pub device: wgpu::Device,
    /// Handle to a command queue on a device.
    pub queue: wgpu::Queue,
    /// Describes a [`wgpu::Surface`]
    pub config: wgpu::SurfaceConfiguration,
    /// The size of the window
    pub size: winit::dpi::PhysicalSize<u32>,
    /// The texture bind group layout
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    /// The uniform bind group layout
    pub default_uniform_bind_group_layout: wgpu::BindGroupLayout,
    /// The depth buffer, used to render object depth
    pub depth_buffer: (wgpu::Texture, wgpu::TextureView, wgpu::Sampler),
    /// The default data used within the renderer
    pub default_data: Option<(crate::Textures, crate::Shaders, crate::UniformBuffers)>,
    /// The camera used in the engine
    pub camera: Option<crate::UniformBuffers>,
    /// Background clear color
    pub clear_color: wgpu::Color,
    /// Scissor cut section of the screen to render to
    /// (x, y, width, height)
    pub scissor_rect: Option<(u32, u32, u32, u32)>,
}
unsafe impl Sync for Renderer {}
unsafe impl Send for Renderer {}

/// Descriptor and settings for a window.
#[derive(Debug, Clone)]
pub struct WindowDescriptor {
    /// The width of the window
    pub width: u32,
    /// The height of the window
    pub height: u32,
    /// The title of the window
    pub title: &'static str,
    /// Should the window contain the keys like minimize, maximize, or resize?
    pub decorations: bool,
    /// Should the window be resizable
    pub resizable: bool,
    /// Define how much power should the app ask for
    pub power_preference: crate::PowerPreference,
    /// The backend to use for the draw
    pub backends: crate::Backends,
    /// The features to be enabled on a backend
    ///
    /// read more at [wgpu::Features]
    pub features: crate::wgpu::Features,
    /// Controls how the events are processed
    ///
    /// read more at [winit::event_loop::ControlFlow]
    pub control_flow: crate::winit::event_loop::ControlFlow,
    /// The presentation mode of renderer for things like VSync
    ///
    /// read more at [wgpu::PresentMode]
    pub present_mode: crate::wgpu::PresentMode,
    /// Limits to be required based on the generation of the GPU and the API.
    ///
    /// read more at [wgpu::Limits]
    pub limits: crate::wgpu::Limits,
    /// The alpha mode which specifies how the alpha channel of
    /// the textures should be handled during compositing.
    pub alpha_mode: crate::wgpu::CompositeAlphaMode,
    /// The desired frame latency.
    ///
    /// read more at [wgpu::SurfaceConfiguration::desired_maximum_frame_latency]
    pub desired_maximum_frame_latency: u32,
    /// How the memory should be utilized
    ///
    /// read more at [wgpu::MemoryHints]
    pub memory_hints: crate::wgpu::MemoryHints,
}
impl std::default::Default for WindowDescriptor {
    /// Will quickly create a window with default settings
    fn default() -> Self {
        let backends = crate::Backends::all();
        Self {
            width: 800,
            height: 600,
            title: "Blue Engine",
            decorations: true,
            resizable: true,
            power_preference: crate::PowerPreference::LowPower,
            backends,
            features: if backends == wgpu::Backends::VULKAN {
                wgpu::Features::POLYGON_MODE_LINE | wgpu::Features::POLYGON_MODE_POINT
            } else if backends
                .contains(wgpu::Backends::VULKAN | wgpu::Backends::METAL | wgpu::Backends::DX12)
            {
                wgpu::Features::POLYGON_MODE_LINE
            } else {
                wgpu::Features::empty()
            },
            control_flow: crate::winit::event_loop::ControlFlow::Poll,
            present_mode: crate::wgpu::PresentMode::AutoNoVsync,
            limits: crate::wgpu::Limits::default(),
            alpha_mode: crate::wgpu::CompositeAlphaMode::Auto,
            desired_maximum_frame_latency: 2,
            memory_hints: crate::MemoryHints::Performance,
        }
    }
}
unsafe impl Send for WindowDescriptor {}
unsafe impl Sync for WindowDescriptor {}

/// Container for the projection used by the camera
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Projection {
    /// Perspective projection
    ///
    /// This is the default project used by the video games and majority of graphics
    Perspective {
        /// The field of view
        fov: f32,
    },
    /// Orthographic projection
    ///
    /// This projection gives you a 2D view of the scene
    Orthographic {
        /// The size of the view
        zoom: f32,
    },
}

/// Container for the camera feature. The settings here are needed for
/// algebra equations needed for camera vision and movement. Please leave it to the renderer to handle
#[derive(Debug)]
pub struct Camera {
    /// The position of the camera in 3D space
    pub position: nalgebra_glm::Vec3,
    /// The target at which the camera should be looking
    pub target: nalgebra_glm::Vec3,
    /// The up vector of the camera. This defines the elevation of the camera
    pub up: nalgebra_glm::Vec3,
    /// The resolution of the camera view
    pub resolution: (f32, f32),
    /// The projection of the camera
    pub projection: Projection,
    /// The closest view of camera
    pub near: f32,
    /// The furthest view of camera
    pub far: f32,
    /// The final data that will be sent to GPU
    pub view_data: nalgebra_glm::Mat4,
    // For checking and rebuilding it's uniform buffer
    pub(crate) changed: bool,
    /// The uniform data of the camera to be sent to the gpu
    pub uniform_data: UniformBuffers,
    /// The position and target of the camera
    pub(crate) add_position_and_target: bool,
}
unsafe impl Send for Camera {}
unsafe impl Sync for Camera {}

/// Container for Cameras
///
/// This allows for different objects have a different camera perspective.
#[derive(Debug)]
pub struct CameraContainer {
    /// The list of cameras
    // Arc<str> is used instead of String for performance
    pub cameras: std::collections::HashMap<std::sync::Arc<str>, Camera>,
}
impl_deref_field!(
    CameraContainer,
    std::collections::HashMap<std::sync::Arc<str>, Camera>,
    cameras
);

/// These definitions are taken from wgpu API docs
#[derive(Debug, Clone, Copy)]
pub struct ShaderSettings {
    // ===== PRIMITIVE ===== //
    /// The primitive topology used to interpret vertices
    pub topology: crate::ShaderPrimitive,
    /// When drawing strip topologies with indices, this is the
    /// required format for the index buffer. This has no effect
    /// on non-indexed or non-strip draws.
    pub strip_index_format: Option<crate::IndexFormat>,
    /// The face to consider the front for the purpose of
    /// culling and stencil operations.
    pub front_face: crate::FrontFace,
    /// The face culling mode
    pub cull_mode: Option<crate::CullMode>,
    /// Controls the way each polygon is rasterized. Can be
    /// either `Fill` (default), `Line` or `Point`
    ///
    /// Setting this to something other than `Fill` requires
    /// `NON_FILL_POLYGON_MODE` feature to be enabled
    pub polygon_mode: crate::PolygonMode,
    /// If set to true, the polygon depth is clamped to 0-1
    /// range instead of being clipped.
    ///
    /// Enabling this requires the `DEPTH_CLAMPING` feature
    /// to be enabled
    pub clamp_depth: bool,
    /// If set to true, the primitives are rendered with
    /// conservative overestimation. I.e. any rastered
    /// pixel touched by it is filled. Only valid for PolygonMode::Fill!
    ///
    /// Enabling this requires `CONSERVATIVE_RASTERIZATION`
    /// features to be enabled.
    pub conservative: bool,

    // ===== Multisample ===== //
    /// The number of samples calculated per pixel (for MSAA).
    /// For non-multisampled textures, this should be `1`
    pub count: u32,
    /// Bitmask that restricts the samples of a pixel modified
    /// by this pipeline. All samples can be enabled using the
    /// value `!0`
    pub mask: u64,
    /// When enabled, produces another sample mask per pixel
    /// based on the alpha output value, that is ANDead with the
    /// sample_mask and the primitive coverage to restrict the
    /// set of samples affected by a primitive.

    /// The implicit mask produced for alpha of zero is guaranteed
    /// to be zero, and for alpha of one is guaranteed to be all
    /// 1-s.
    pub alpha_to_coverage_enabled: bool,
}
impl Default for ShaderSettings {
    fn default() -> Self {
        Self {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            clamp_depth: false,
            conservative: false,
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: true,
        }
    }
}
unsafe impl Send for ShaderSettings {}
unsafe impl Sync for ShaderSettings {}

/// Instance buffer data that is sent to GPU
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    /// The transformation matrix of the instance
    pub model: uniform_type::Matrix,
}

/// Instance buffer data storage
#[derive(Debug, Clone, Copy)]
pub struct Instance {
    /// The position of the instance
    pub position: nalgebra_glm::Vec3,
    /// The rotation of the instance
    pub rotation: nalgebra_glm::Vec3,
    /// The scale of the instance
    pub scale: nalgebra_glm::Vec3,
}

/// Allows all events to be fetched directly, making it easier to add custom additions to the engine.
pub trait Signal: Any {
    /// This is ran as soon as the engine is properly initialized and all components are ready
    #[allow(clippy::too_many_arguments)]
    fn init(
        &mut self,
        _renderer: &mut crate::Renderer,
        _window: &crate::Window,
        _objects: &mut ObjectStorage,
        _camera: &mut crate::CameraContainer,
    ) {
    }

    /// This is ran at the device events when available
    #[allow(clippy::too_many_arguments)]
    fn device_events(
        &mut self,
        _renderer: &mut crate::Renderer,
        _window: &crate::Window,
        _objects: &mut ObjectStorage,
        _events: &crate::DeviceEvent,
        _input: &crate::InputHelper,
        _camera: &mut crate::CameraContainer,
    ) {
    }

    /// This is ran at the window events when available
    #[allow(clippy::too_many_arguments)]
    fn window_events(
        &mut self,
        _renderer: &mut crate::Renderer,
        _window: &crate::Window,
        _objects: &mut ObjectStorage,
        _events: &crate::WindowEvent,
        _input: &crate::InputHelper,
        _camera: &mut crate::CameraContainer,
    ) {
    }

    /// ran before the frame is rendered
    #[allow(clippy::too_many_arguments)]
    fn frame(
        &mut self,
        _renderer: &mut crate::Renderer,
        _window: &crate::Window,
        _objects: &mut ObjectStorage,
        _camera: &mut crate::CameraContainer,
        _input: &crate::InputHelper,
        _encoder: &mut crate::CommandEncoder,
        _view: &crate::TextureView,
    ) {
    }
}
// The engine needs to know the functions of Signal to do things internally,
// so we use downcast and not the std::any::Any
downcast!(dyn Signal);

/// Defines how the rotation axis is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateAxis {
    #[doc(hidden)]
    X,
    #[doc(hidden)]
    Y,
    #[doc(hidden)]
    Z,
}
unsafe impl Send for RotateAxis {}
unsafe impl Sync for RotateAxis {}

/// Defines how the rotation amount is
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RotateAmount {
    #[doc(hidden)]
    Radians(f32),
    #[doc(hidden)]
    Degrees(f32),
}
unsafe impl Send for RotateAmount {}
unsafe impl Sync for RotateAmount {}

/// Defines how the texture data is
#[derive(Debug, Clone)]
pub enum TextureData {
    /// the texture file bytes directly
    Bytes(Vec<u8>),
    /// the texture as a [`image::DynamicImage`]
    Image(image::DynamicImage),
    /// path to a texture file to load
    Path(String),
}
unsafe impl Send for TextureData {}
unsafe impl Sync for TextureData {}

/// Defines how the borders of texture would look like
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureMode {
    /// Expands the texture to fit the object
    Clamp,
    /// Repeats the texture instead of stretching
    Repeat,
    /// Repeats the texture, but mirrors it on edges
    MirrorRepeat,
}
unsafe impl Send for TextureMode {}
unsafe impl Sync for TextureMode {}

/// This function helps in converting pixel value to the value that is between -1 and +1
pub fn pixel_to_cartesian(value: f32, max: u32) -> f32 {
    let mut result = value / max as f32;

    if value == max as f32 {
        result = 0.0;
    } else if result < max as f32 / 2.0 {
    }

    if result > -1.0 {
        result
    } else {
        -1.0
    }
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

/// A unified way to handle objects
///
/// This is a container for objects that is used to apply different operations on the objects at the same time.
/// It can deref to the object hashmap itself when needed.
pub struct ObjectStorage(std::collections::HashMap<String, Object>);
impl ObjectStorage {
    /// Creates a new object storage
    pub fn new() -> Self {
        ObjectStorage(std::collections::HashMap::new())
    }
}
impl Default for ObjectStorage {
    fn default() -> Self {
        Self::new()
    }
}
unsafe impl Send for ObjectStorage {}
unsafe impl Sync for ObjectStorage {}

impl_deref!(ObjectStorage, std::collections::HashMap<String, Object>);

/// Depth format
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

/// Handles the live events in the engine
pub struct SignalStorage {
    /// list of events with key and the event
    pub events: Vec<(String, Box<dyn Signal>)>,
}

/// Handles the order in which a functionality in the engine should be executed
pub enum ExecuteOrder {
    /// The main function that is the update_loop
    UpdateLoopFunction,
}

/// A wrapper for winit window to make it easier to use and more ergonomic.
#[derive(Debug)]
pub struct Window {
    /// The winit window itself.
    pub window: Option<std::sync::Arc<crate::winit::window::Window>>,
    /// Default attributes of the window
    pub default_attributes: winit::window::WindowAttributes,
    /// Whether the engine should close.
    pub should_close: bool,
}
impl_deref_field!(
    Window,
    Option<std::sync::Arc<crate::winit::window::Window>>,
    window
);

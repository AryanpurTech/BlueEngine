/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

#[cfg(feature = "gui")]
pub use imgui as gui;

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

#[cfg(feature = "gui")]
pub enum Style {
    Config(gui::StyleVar),
    Color(gui::StyleColor, [f32; 4]),
}

#[cfg(feature = "gui")]
pub fn style_block<F: FnMut()>(styles: Vec<Style>, mut ui_block: F, ui: &gui::Ui) {
    let mut stack = Vec::<gui::StyleStackToken>::new();
    let mut color = Vec::<gui::ColorStackToken>::new();

    for i in styles {
        match i {
            Style::Config(data) => stack.push(ui.push_style_var(data)),
            Style::Color(data, hue) => color.push(ui.push_style_color(data, hue)),
        }
    }
    ui_block();
    for i in stack {
        i.end();
    }
    for i in color {
        i.end();
    }
}

/// A container for uniform buffer types
pub mod uniform_type {

    /// 4 by 4, 32 bit float matrix uniform buffer
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Matrix {
        pub data: [[f32; 4]; 4],
    }
    impl Matrix {
        /// Replaces it's values by the new values provided
        pub fn update(&mut self, uniform: Matrix) {
            self.data = uniform.data;
        }

        /// Converts internal matrix to the uniform matrix
        pub fn from_im(matrix: nalgebra_glm::Mat4) -> Self {
            let mtx = matrix.as_slice();

            Self {
                data: [
                    [mtx[0], mtx[1], mtx[2], mtx[3]],
                    [mtx[4], mtx[5], mtx[6], mtx[7]],
                    [mtx[8], mtx[9], mtx[10], mtx[11]],
                    [mtx[12], mtx[13], mtx[14], mtx[15]],
                ],
            }
        }

        /// Converts uniform matrix to internal matrix
        pub fn to_im(&self) -> nalgebra_glm::Mat4 {
            let mtx = self.data;

            nalgebra_glm::mat4(
                mtx[0][0], mtx[0][1], mtx[0][2], mtx[0][3], mtx[1][0], mtx[1][1], mtx[1][2],
                mtx[1][3], mtx[2][0], mtx[2][1], mtx[2][2], mtx[2][3], mtx[3][0], mtx[3][1],
                mtx[3][2], mtx[3][3],
            )
        }
    }
    impl std::ops::Mul for Matrix {
        type Output = Matrix;

        fn mul(self, rhs: Self) -> Self::Output {
            let a = self.data;
            let b = rhs.data;
            Matrix {
                data: [
                    [
                        a[0][0] * b[0][0],
                        a[0][1] * b[1][0],
                        a[0][2] * b[2][0],
                        a[0][3] * b[3][0],
                    ],
                    [
                        a[1][0] * b[0][1],
                        a[1][1] * b[1][1],
                        a[1][2] * b[2][1],
                        a[1][3] * b[3][1],
                    ],
                    [
                        a[2][0] * b[0][2],
                        a[2][1] * b[1][2],
                        a[2][2] * b[2][2],
                        a[2][3] * b[3][2],
                    ],
                    [
                        a[3][0] * b[0][3],
                        a[3][1] * b[1][3],
                        a[3][2] * b[2][3],
                        a[3][3] * b[3][3],
                    ],
                ],
            }
        }
    }

    /// An array with length 3, each 32 bit float value, uniform buffer
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Array3 {
        pub data: [f32; 3],
    }
    impl std::ops::Mul<Array3> for Array3 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Self {
                data: [
                    self.data[0] * rhs.data[0],
                    self.data[1] * rhs.data[1],
                    self.data[2] * rhs.data[2],
                ],
            }
        }
    }
    impl std::ops::Mul<f32> for Array3 {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self::Output {
            Self {
                data: [self.data[0] * rhs, self.data[1] * rhs, self.data[2] * rhs],
            }
        }
    }

    /// An array with length 4, each 32 bit float value, uniform buffer
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Array4 {
        pub data: [f32; 4],
    }
    impl Array4 {
        pub fn update(&mut self, uniform: Array4) {
            self.data = uniform.data;
        }
    }
    impl std::ops::Mul<Array4> for Array4 {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            Array4 {
                data: [
                    self.data[0] * rhs.data[0],
                    self.data[1] * rhs.data[1],
                    self.data[2] * rhs.data[2],
                    self.data[3] * rhs.data[3],
                ],
            }
        }
    }
    impl std::ops::Mul<f32> for Array4 {
        type Output = Array4;

        fn mul(self, rhs: f32) -> Self::Output {
            Array4 {
                data: [
                    self.data[0] * rhs,
                    self.data[1] * rhs,
                    self.data[2] * rhs,
                    self.data[3] * rhs,
                ],
            }
        }
    }

    /// A 32 bit float uniform buffer
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Float {
        pub data: f32,
    }
    impl Float {
        pub fn update(&mut self, uniform: Float) {
            self.data = uniform.data;
        }
    }
}

pub use bytemuck::Pod;
pub use bytemuck::Zeroable;

/// Objects make it easier to work with Blue Engine, it automates most of work needed for
/// creating 3D objects and showing them on screen. A range of default objects are available
/// as well as ability to customize each of them and even create your own! You can also
/// customize almost everything there is about them!
pub struct Object {
    /// Give your object a name, which can help later on for debugging.
    pub name: Option<&'static str>,
    /// A list of Vertex
    pub vertices: Vec<Vertex>,
    /// A list of indices that dictates the order that vertices appear
    pub indices: Vec<u16>,
    pub uniform_layout: wgpu::BindGroupLayout,
    /// Pipeline holds all the data that is sent to GPU, including shaders and textures
    pub pipeline: Pipeline,
    /// Dictates the size of your object in pixels
    pub size: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    /// Dictates the position of your object in pixels
    pub position: (f32, f32, f32),
    // flags the object to be updated until next frame
    pub(crate) changed: bool,
    /// Transformation matrix helps to apply changes to your object, including position, orientation, ...
    /// Best choice is to let the Object system handle it
    pub transformation_matrix: nalgebra_glm::Mat4,
    /// The main color of your object
    pub uniform_color: uniform_type::Array4,
    /// The color of your object that is sent to gpu
    pub color: uniform_type::Array4,
    /// The index of the object in the queue
    pub object_index: usize,
    /// A struct making it easier to manipulate specific parts of shader
    pub shader_builder: crate::objects::ShaderBuilder,
    /// Shader settings
    pub shader_settings: ShaderSettings,
    /// Camera have any effect on the object?
    pub camera_effect: bool,
    /// Uniform Buffers to be sent to GPU
    pub uniform_buffers: Vec<wgpu::Buffer>,
}

/// Extra settings to customize objects on time of creation
#[derive(Debug, Clone, Copy)]
pub struct ObjectSettings {
    /// Give your object a name, which can help later on for debugging.
    pub name: Option<&'static str>,
    /// Dictates the size of your object in pixels
    pub size: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    /// Dictates the position of your object in pixels
    pub position: (f32, f32, f32),
    /// The color of your object, A.K.A. albedo sometimes
    pub color: uniform_type::Array4,
    /// Should it be affected by camera?
    pub camera_effect: bool,
    /// Shader Settings
    pub shader_settings: ShaderSettings,
}
impl Default for ObjectSettings {
    fn default() -> Self {
        Self {
            name: Some("Object!"),
            size: (100f32, 100f32, 100f32),
            scale: (1f32, 1f32, 1f32),
            position: (0f32, 0f32, 0f32),
            color: uniform_type::Array4 {
                data: crate::utils::default_resources::DEFAULT_COLOR,
            },
            camera_effect: true,
            shader_settings: ShaderSettings::default(),
        }
    }
}

pub struct Geometry {
    /// A list of Vertex
    pub vertices: Vec<Vertex>,
    /// A list of indices that dictates the order that vertices appear
    pub indices: Vec<u16>,
}

/// The engine is the main starting point of using the Blue Engine. Everything that runs on Blue Engine will be under this struct.
/// The structure of engine is monolithic, but the underlying data and the way it works is not.
/// It gives a set of default data to work with, but also allow you to go beyond that and work as low level as you wish to.
///
/// You can also use the Engine to build you own custom structure the way you wish for it to be. Possibilities are endless!
///
/// To start using the Blue Engine, you can start by creating a new Engine like follows:
/// ```
/// use blue_engine::header::{Engine, WindowDescriptor};
///
/// fn main() {
///     let engine = Engine::new(WindowDescriptor::default()).expect("Couldn't create the engine");
/// }
/// ```
/// The WindowDescriptor simply holds what features you would like for your window. If you are reading this on later version of
/// the engine, you might be able to even run the engine in headless mode meaning there would not be a need for a window and the
/// renders would come as image files.
///
/// If you so wish to have a window, you would need to start a window update loop. The update loop of window runs a frame every few milisecond,
/// and gives you details of what is happening during this time, like input events. You can also modify existing parts of the engine during
/// this update loop, such as changing camera to look differently, or creating a new object on the scene, or even changing window details!
///
/// The update loop is just a method of the Engine struct that have one argument which is a callback function.
/// ```
///
/// ```
/// [THE DATA HERE IS WORK IN PROGRESS!]
pub struct Engine {
    /// The renderer does exactly what it is called. It works with the GPU to render frames according to the data you gave it.
    pub renderer: Renderer,
    // The event_loop handles the events of the window and inputs, so it's used internally
    pub(crate) event_loop: winit::event_loop::EventLoop<()>,
    /// The window handles everything about window and inputs. This includes ability to modify window and listen to input devices for changes.
    pub window: winit::window::Window,
    /// The object system is a way to make it easier to work with the engine. Obviously you can work without it, but it's for those who
    /// do not have the know-how, or wish to handle all the work of rendering data manually.
    pub objects: Vec<Object>,
    /// The camera handles the way the scene looks when rendered. You can modify everything there is to camera through this.
    pub camera: Camera,
}

/// Container for pipeline values. Each pipeline takes only 1 vertex shader, 1 fragment shader, 1 texture data, and optionally a vector of uniform data.
pub struct Pipeline {
    pub shader: Shaders,
    pub vertex_buffer: VertexBuffers,
    pub texture: Textures,
    pub uniform: Option<UniformBuffers>,
}

/// Container for vertex and index buffer
pub struct VertexBuffers {
    /// An array of vertices. A vertex is a point in 3D space containing an X, Y, and a Z coordinate between -1 and +1
    pub vertex_buffer: wgpu::Buffer,
    /// An array of indices. Indices are a way to reuse vertices, this in turn helps greatly in reduction of amount of vertices needed to be sent to the GPU
    pub index_buffer: wgpu::Buffer,
    pub length: u32,
}

/// Shaders are programs that runs on the GPU
pub type Shaders = wgpu::RenderPipeline;
/// Uniform Buffers are small amount of data that are sent from CPU to GPU
pub type UniformBuffers = wgpu::BindGroup;
/// Textures are image data that are sent to GPU to be set to a surface
pub type Textures = wgpu::BindGroup;

// Main renderer class. this will contain all methods and data related to the renderer
pub struct Renderer {
    pub(crate) surface: wgpu::Surface,
    #[cfg(feature = "gui")]
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    pub(crate) texture_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) default_uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) depth_buffer: (wgpu::Texture, wgpu::TextureView, wgpu::Sampler),
    pub default_data: Option<(Textures, Shaders, UniformBuffers)>,
    pub camera: Option<UniformBuffers>,
}

/// Descriptor and settings for a window.
#[derive(Debug, Clone, Copy)]
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
    pub power_preference: PowerPreference,
}
impl std::default::Default for WindowDescriptor {
    /// Will quickly create a window with default settings
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            title: "Blue Engine",
            decorations: true,
            resizable: true,
            power_preference: PowerPreference::LowPower,
        }
    }
}

/// Container for the camera feature. The settings here are needed for
/// algebra equations needed for camera vision and movement. Please leave it to the renderer to handle
#[derive(Debug)]
pub struct Camera {
    /// The position of the camera in 3D space
    pub position: nalgebra_glm::Vec3,
    /// The target at which the camera should be looking
    pub target: nalgebra_glm::Vec3,
    pub up: nalgebra_glm::Vec3,
    pub resolution: (f32, f32),
    /// The field of view of the camera
    pub fov: f32,
    /// The closest view of camera
    pub near: f32,
    /// The furthest view of camera
    pub far: f32,
    /// The final data that will be sent to GPU
    pub view_data: nalgebra_glm::Mat4,
    // For checking and rebuilding it's uniform buffer
    pub(crate) changed: bool,
    pub(crate) uniform_data: UniformBuffers,
    pub(crate) add_position_and_target: bool,
}

pub struct LightManager {
    pub ambient_color: uniform_type::Array4,
    pub ambient_strength: f32,
    pub affected_objects: Vec<usize>,
    pub light_objects: std::collections::BTreeMap<usize, [f32; 3]>,
}

/// Device Events
pub use winit::event::DeviceEvent;
/// The mouse button identifier
pub use winit::event::MouseButton;
/// Keyboard keys identifier
pub use winit::event::VirtualKeyCode;

/// This function helps in converting pixel value to the value that is between -1 and +1
pub fn normalize(value: f32, max: u32) -> f32 {
    let mut result = value / max as f32;

    if value == max as f32 {
        result = 0.0;
    } else if result < max as f32 / 2.0 {
    }

    if result > -1.0 {
        return result as f32;
    } else {
        return -1.0;
    }
}

/// Returns
pub fn percentage(amount: f32, of: f32) -> f32 {
    let result = amount / of;

    return result;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateAxis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
pub enum TextureData {
    Bytes(Vec<u8>),
    Image(image::DynamicImage),
}

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

/// Defines file format of the texture to load
pub enum TextureFormat {
    PNG,
    BMP,
    JPEG,
    PNM,
}

pub type ShaderPrimitive = wgpu::PrimitiveTopology;
pub type IndexFormat = wgpu::IndexFormat;
pub type FrontFace = wgpu::FrontFace;
pub type CullMode = wgpu::Face;
pub type PolygonMode = wgpu::PolygonMode;
pub type PowerPreference = wgpu::PowerPreference;

// ? These definitions are taken from wgpu API docs
#[derive(Debug, Clone, Copy)]
pub struct ShaderSettings {
    // ===== PRIMITIVE ===== //
    /// The primitive topology used to interpret vertices
    pub topology: ShaderPrimitive,
    /// When drawing strip topologies with indices, this is the
    /// required format for the index buffer. This has no effect
    /// on non-indexed or non-strip draws.
    pub strip_index_format: Option<IndexFormat>,
    /// The face to consider the front for the purpose of
    /// culling and stencil operations.
    pub front_face: FrontFace,
    /// The face culling mode
    pub cull_mode: Option<CullMode>,
    /// Controls the way each polygon is rasterized. Can be
    /// either `Fill` (default), `Line` or `Point`
    ///
    /// Setting this to something other than `Fill` requires
    /// `NON_FILL_POLYGON_MODE` feature to be enabled
    pub polygon_mode: PolygonMode,
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
    /// based on the alpha output value, that is ANDed with the
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
            alpha_to_coverage_enabled: false,
        }
    }
}

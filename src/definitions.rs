/// Will contain all details about a vertex and will be sent to GPU
// Will be turned to C code and sent to GPU
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// Contains position data for the vertex in 3D space
    pub position: [f32; 3],
    /// Contains texture position data for the vertex
    pub texture: [f32; 2],
}
// implementing some structure data for the layout configuration later on pipeline
impl Vertex {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
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
        pub fn update(&mut self, uniform: Matrix) {
            self.data = uniform.data;
        }
    }

    /// An array with length 4, each 32 bit float value, uniform buffer
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Array {
        pub data: [f32; 4],
    }
    impl Array {
        pub fn update(&mut self, uniform: Array) {
            self.data = uniform.data;
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

// Container for pipeline values. Each pipeline takes only 1 vertex shader, 1 fragment shader, 1 texture data, and optionally a vector of uniform data.
pub struct Pipeline {
    pub shader_index: usize,
    pub buffer_index: usize,
    pub texture_index: Option<usize>,
    pub uniform_buffer: Option<usize>,
}

// Container for vertex and index buffer
pub struct Buffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub length: u32,
    pub instances: std::ops::Range<u32>,
}

// Main renderer class. this will contain all methods and data related to the renderer
pub struct Renderer {
    pub(crate) surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) sc_desc: wgpu::SwapChainDescriptor,
    pub(crate) swap_chain: wgpu::SwapChain,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    pub(crate) texture_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) shaders: Vec<wgpu::RenderPipeline>,
    pub(crate) buffers: Vec<Buffers>,
    pub(crate) texture_bind_group: Vec<wgpu::BindGroup>,
    pub(crate) uniform_bind_group: Vec<wgpu::BindGroup>,
    pub(crate) render_pipeline: Vec<Pipeline>,
}

// Callbacks for renderloop
pub enum WindowCallbackEvents<'a> {
    Before,                                           // This will run before the loop
    During(&'a winit_input_helper::WinitInputHelper), // will be called during the loop
    After, // will be called after the loop ended. Maybe for cleanup?
}

/// Descriptor and settings for a window.
pub struct WindowDescriptor {
    /// The width of the window
    pub width: f64,
    /// The height of the window
    pub height: f64,
    /// The title of the window
    pub title: &'static str,
    /// Should the window contain the keys like minimize, maximize, or resize?
    pub decorations: bool,
    /// Should the window be resizable
    pub resizable: bool,
}
impl WindowDescriptor {
    /// Will quickly create a window with default settings
    pub fn default() -> Result<Self, anyhow::Error> {
        Ok(Self {
            width: 800.0,
            height: 600.0,
            title: "Blue Engine by Blue Mazar",
            decorations: true,
            resizable: true,
        })
    }
}

/// Container for the camera feature. The settings here are needed for 
/// algebra equations needed for camera vision and movement. Please leave it to the renderer to handle
pub struct Camera {
    /// The position of the camera in 3D space
    pub eye: cgmath::Point3<f32>,
    /// The target at which the camera should be looking
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    /// The field of view of the camera
    pub fovy: f32,
    /// The closest view of camera
    pub znear: f32,
    /// The furthest view of camera
    pub zfar: f32,
}

/// The mouse button identifier
pub use winit::event::MouseButton;
/// Keyboard keys identifier
pub use winit::event::VirtualKeyCode as KeyboardKeys;

/// Buffer type enum, allowing for multiple types to be sent
#[derive(Clone, Debug)]
pub enum UniformBuffer {
    Matrix(&'static str, uniform_type::Matrix),
    Array(&'static str, uniform_type::Array),
    Float(&'static str, uniform_type::Float),
}

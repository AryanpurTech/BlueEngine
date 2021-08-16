/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

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
impl Vertex {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
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

pub struct Object {
    pub name: Option<&'static str>,
    pub verticies: Vec<Vertex>,
    pub indicies: Vec<u16>,
    pub pipeline: Pipeline,
    pub pipeline_id: Option<usize>,
    pub window_size: winit::dpi::PhysicalSize<u32>,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub changed: bool
}

pub struct Engine {
    pub renderer: Renderer,
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub window: winit::window::Window,
    pub objects: Vec<Object>,
}

// Container for pipeline values. Each pipeline takes only 1 vertex shader, 1 fragment shader, 1 texture data, and optionally a vector of uniform data.
#[derive(Debug, Clone, Copy)]
pub struct Pipeline {
    pub shader_index: usize,
    pub vertex_buffer_index: usize,
    pub texture_index: usize,
    pub uniform_index: Option<usize>,
}

// Container for vertex and index buffer
pub struct VertexBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub length: u32,
}

pub type Shaders = wgpu::RenderPipeline;
pub type UniformBuffers = wgpu::BindGroup;
pub type Textures = wgpu::BindGroup;

// Main renderer class. this will contain all methods and data related to the renderer
pub struct Renderer {
    pub(crate) surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) sc_desc: wgpu::SwapChainDescriptor,
    pub(crate) swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub(crate) texture_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) shaders: Vec<Shaders>,
    pub(crate) vertex_buffers: Vec<VertexBuffers>,
    pub(crate) texture_bind_group: Vec<Textures>,
    pub(crate) uniform_bind_group: Vec<UniformBuffers>,
    pub(crate) render_pipelines: Vec<Pipeline>,
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

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0
);

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

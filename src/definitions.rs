#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture: [f32; 2],
}
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
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

pub mod uniform_type {
    pub trait Types {}

    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Matrix {
        pub data: [[f32; 4]; 4],
    }
    impl Types for Matrix {}
    impl Matrix {
        pub fn update(&mut self, uniform: Matrix) {
            self.data = uniform.data;
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Array {
        pub data: [f32; 4],
    }
    impl Types for Array {}
    impl Array {
        pub fn update(&mut self, uniform: Array) {
            self.data = uniform.data;
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct Float {
        pub data: f32,
    }
    impl Types for Float {}
    impl Float {
        pub fn update(&mut self, uniform: Float) {
            self.data = uniform.data;
        }
    }
}

pub type Shaders = wgpu::RenderPipeline;

pub struct Pipeline {
    pub shader_index: usize,
    pub buffer_index: usize,
    pub texture_index: Option<usize>,
    pub uniform_buffer: Option<usize>,
}

pub struct Buffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub length: u32,
    pub instances: std::ops::Range<u32>,
}

pub struct Renderer {
    pub(crate) surface: wgpu::Surface,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) sc_desc: wgpu::SwapChainDescriptor,
    pub(crate) swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub(crate) texture_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) uniform_bind_group_layout: wgpu::BindGroupLayout,
    pub shaders: Vec<Shaders>,
    pub buffers: Vec<Buffers>,
    pub texture_bind_group: Vec<wgpu::BindGroup>,
    pub uniform_bind_group: Vec<wgpu::BindGroup>,
    pub render_pipeline: Vec<Pipeline>,
}

pub enum WindowCallbackEvents<'a> {
    Before,
    During(&'a winit_input_helper::WinitInputHelper),
    After,
}

pub struct WindowDescriptor {
    pub width: f64,
    pub height: f64,
    pub title: &'static str,
    pub decorations: bool,
    pub resizable: bool,
}

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

use std::usize;

pub use winit::event::MouseButton;
pub use winit::event::VirtualKeyCode as KeyboardKeys;

#[derive(Clone, Debug)]
pub enum UniformBuffer {
    Matrix(&'static str, uniform_type::Matrix),
    Array(&'static str, uniform_type::Array),
    Float(&'static str, uniform_type::Float),
}

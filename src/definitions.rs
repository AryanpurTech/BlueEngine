#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub texture: [f32; 2],
}
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct _UniformsM {
    pub data: [[f32; 4]; 4],
}
unsafe impl bytemuck::Pod for _UniformsM {}
unsafe impl bytemuck::Zeroable for _UniformsM {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct _UniformsV {
    pub data: [f32; 4],
}
unsafe impl bytemuck::Pod for _UniformsV {}
unsafe impl bytemuck::Zeroable for _UniformsV {}

pub enum UniformBuffer {
    Matrix(_UniformsM),
    Array(_UniformsV),
}

pub enum UniformBindGroup {
    Matrix(wgpu::BindGroup),
    Array(wgpu::BindGroup),
}

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

pub type Shaders = wgpu::RenderPipeline;

pub struct Pipeline {
    pub shader_index: usize,
    pub buffers: Buffers,
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
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_dynamic_bind_group_layout: wgpu::BindGroupLayout,
    pub uniform_static_bind_group_layout: wgpu::BindGroupLayout,
    pub render_pipeline_layout: wgpu::PipelineLayout,
    pub shaders: Vec<Shaders>,
    pub texture_bind_group: Vec<wgpu::BindGroup>,
    pub uniform_bind_group: Vec<UniformBindGroup>,
    pub render_pipeline: Vec<Pipeline>,
}

pub struct ShadersData {
    pub name: &'static str,
    pub vertex_shader: Vec<u8>,
    pub fragment_shader: Vec<u8>,
}

pub struct BuffersData {
    pub verticies: Vec<Vertex>,
    pub indicies: Vec<u16>,
    pub instances: std::ops::Range<u32>,
}

pub type Callback = Option<fn(renderer: &mut Renderer)>;

pub struct WindowDescriptor {
    pub width: f64,
    pub height: f64,
    pub title: &'static str,
    pub decorations: bool,
    pub resizable: bool,
    pub before: Callback,
    pub during: Callback,
    pub after: Callback,
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

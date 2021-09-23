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

        pub fn from_glm(matrix: glm::Matrix4<f32>) -> Matrix {
            let mtx = matrix.as_array();
            Matrix {
                data: [
                    [mtx[0][0], mtx[0][1], mtx[0][2], mtx[0][3]],
                    [mtx[1][0], mtx[1][1], mtx[1][2], mtx[1][3]],
                    [mtx[2][0], mtx[2][1], mtx[2][2], mtx[2][3]],
                    [mtx[3][0], mtx[3][1], mtx[3][2], mtx[3][3]],
                ],
            }
        }

        pub fn to_glm(matrix: Self) -> glm::Matrix4<f32> {
            let mtx = matrix.data;
            glm::Matrix4::new(
                glm::vec4(mtx[0][0], mtx[0][1], mtx[0][2], mtx[0][3]),
                glm::vec4(mtx[1][0], mtx[1][1], mtx[1][2], mtx[1][3]),
                glm::vec4(mtx[2][0], mtx[2][1], mtx[2][2], mtx[2][3]),
                glm::vec4(mtx[3][0], mtx[3][1], mtx[3][2], mtx[3][3]),
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
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub pipeline: Pipeline,
    pub pipeline_id: Option<usize>,
    pub window_size: winit::dpi::PhysicalSize<u32>,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub changed: bool,
    pub transformation_matrix: glm::Matrix4<f32>,
    pub color: uniform_type::Array,
}

pub struct Engine {
    pub renderer: Renderer,
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub window: winit::window::Window,
    pub objects: Vec<Object>,
    pub camera: Camera,
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
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub(crate) texture_bind_group_layout: wgpu::BindGroupLayout,
    pub(crate) default_uniform_bind_group_layout: wgpu::BindGroupLayout,
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
impl std::default::Default for WindowDescriptor {
    /// Will quickly create a window with default settings
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            title: "Blue Engine",
            decorations: true,
            resizable: true,
        }
    }
}

/// Container for the camera feature. The settings here are needed for
/// algebra equations needed for camera vision and movement. Please leave it to the renderer to handle
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// The position of the camera in 3D space
    pub eye: glm::Vector3<f32>,
    /// The target at which the camera should be looking
    pub target: glm::Vector3<f32>,
    pub up: glm::Vector3<f32>,
    pub aspect: f32,
    /// The field of view of the camera
    pub fovy: f32,
    /// The closest view of camera
    pub znear: f32,
    /// The furthest view of camera
    pub zfar: f32,
    /// The final data that will be sent to GPU
    pub view_data: glm::Matrix4<f32>,
    // For checking and rebuilding it's uniform buffer
    pub(crate) changed: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotateAxis {
    X,
    Y,
    Z,
}

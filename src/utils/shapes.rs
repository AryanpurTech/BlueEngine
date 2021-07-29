use crate::definitions::{normalize, uniform_type, Pipeline, UniformBuffer, Vertex};

const DEFAULT_SHADER: &str = r#"
// Vertex Stage

[[block]]
struct VertexUniforms {
    camera_matrix: mat4x4<f32>;
};
[[group(1), binding(0)]]
var<uniform> vertex_uniforms: VertexUniforms;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] texture_coordinates: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] texture_coordinates: vec2<f32>;
};

[[stage(vertex)]]
fn main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vertex_uniforms.camera_matrix * vec4<f32>(input.position, 1.0);
    out.texture_coordinates = input.texture_coordinates;
    return out;
}

// Fragment Stage

[[block]]
struct FragmentUniforms {
    color: vec4<f32>;
};
[[group(1), binding(1)]]
var<uniform> fragment_uniforms: FragmentUniforms;

[[group(0), binding(0)]]
var texture_diffuse: texture_2d<f32>;

[[group(0), binding(1)]]
var sampler_diffuse: sampler;

[[stage(fragment)]]
fn main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates) * fragment_uniforms.color;
}
"#;

const DEFAULT_TEXTURE: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0,
    0, 0, 31, 21, 196, 137, 0, 0, 1, 130, 105, 67, 67, 80, 73, 67, 67, 32, 112, 114, 111, 102, 105,
    108, 101, 0, 0, 40, 207, 149, 145, 75, 40, 68, 81, 28, 198, 127, 238, 16, 121, 46, 76, 145,
    164, 187, 192, 138, 18, 146, 165, 134, 72, 81, 26, 163, 188, 22, 238, 189, 99, 134, 154, 123,
    77, 247, 142, 108, 44, 149, 237, 148, 133, 199, 198, 107, 97, 99, 205, 214, 194, 86, 41, 229,
    81, 178, 179, 179, 34, 54, 210, 245, 63, 119, 212, 76, 106, 148, 83, 167, 243, 235, 59, 231,
    251, 58, 231, 59, 160, 29, 164, 44, 219, 43, 237, 2, 219, 201, 184, 209, 145, 136, 62, 61, 51,
    171, 151, 63, 163, 209, 64, 21, 45, 212, 26, 150, 151, 30, 159, 28, 142, 81, 116, 124, 220, 82,
    162, 214, 155, 78, 149, 197, 255, 70, 77, 124, 209, 179, 160, 68, 23, 30, 176, 210, 110, 70,
    120, 65, 184, 111, 45, 147, 86, 188, 35, 28, 182, 150, 140, 184, 240, 169, 112, 135, 43, 23,
    20, 190, 87, 186, 153, 227, 23, 197, 201, 128, 53, 149, 25, 118, 99, 209, 65, 225, 176, 176,
    158, 44, 96, 179, 128, 173, 37, 215, 22, 238, 21, 110, 141, 219, 142, 228, 107, 211, 57, 142,
    43, 94, 87, 108, 167, 86, 173, 159, 123, 170, 23, 86, 47, 58, 83, 147, 74, 151, 217, 204, 8,
    163, 140, 51, 129, 142, 201, 42, 203, 164, 200, 208, 41, 171, 35, 138, 71, 84, 246, 35, 69,
    252, 77, 129, 127, 66, 92, 166, 184, 150, 177, 196, 49, 196, 10, 54, 70, 224, 71, 253, 193,
    239, 110, 189, 68, 79, 119, 46, 169, 58, 2, 101, 79, 190, 255, 214, 6, 229, 91, 240, 149, 245,
    253, 207, 67, 223, 255, 58, 130, 208, 35, 92, 56, 121, 255, 202, 1, 244, 191, 139, 158, 205,
    107, 173, 251, 80, 183, 1, 103, 151, 121, 205, 220, 134, 243, 77, 104, 124, 72, 27, 174, 17,
    72, 33, 153, 90, 34, 1, 175, 39, 242, 77, 51, 80, 127, 13, 149, 115, 185, 222, 126, 246, 57,
    190, 131, 152, 116, 53, 118, 5, 187, 123, 208, 158, 148, 236, 249, 34, 239, 174, 40, 236, 237,
    207, 51, 65, 127, 68, 190, 1, 84, 201, 114, 155, 16, 187, 186, 109, 0, 0, 0, 9, 112, 72, 89,
    115, 0, 0, 46, 34, 0, 0, 46, 34, 1, 170, 226, 221, 146, 0, 0, 0, 13, 73, 68, 65, 84, 24, 87,
    99, 248, 255, 255, 255, 127, 0, 9, 251, 3, 253, 5, 67, 69, 202, 0, 0, 0, 0, 73, 69, 78, 68,
    174, 66, 96, 130,
];

const DEFAULT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Object {
    pub verticies: Vec<Vertex>,
    pub indicies: Vec<u16>,
    pub window_size: winit::dpi::PhysicalSize<u32>,
    pub pipeline: Pipeline,
    pub pipeline_id: Option<usize>,
    pub width: f32,
    pub height: f32,
}

// ? change to Object struct and globalize stuff.
// ? Also this way add automation of pipeline creation
// ? using default values for everything unless needed, including WGSL shaders for stuff
// ? this should also help out in customization of pipeline, like way of rendering, wether to include textures, ...

impl Object {
    pub fn new(
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
        renderer: &mut crate::definitions::Renderer,
        window_size: winit::dpi::PhysicalSize<u32>,
        camera: crate::utils::camera::Camera,
    ) -> anyhow::Result<Self> {
        let uniform_index = Some(renderer.build_and_append_uniform_buffers(Vec::from([
            UniformBuffer::Matrix("Camera", camera.new_camera_uniform_buffer()?),
            UniformBuffer::Array(
                "Color",
                uniform_type::Array {
                    data: DEFAULT_COLOR,
                },
            ),
        ]))?);

        println!("YOOOOOOOOOOOOOOOO{:?}", renderer.uniform_bind_group.len());
        println!("YOOO 1");
        let shader_index =
            renderer.build_and_append_shaders("Default Shader", DEFAULT_SHADER.to_string())?;
        println!("YOOO 2");
        let vertex_buffer_index =
            renderer.build_and_append_vertex_buffers(verticies.clone(), indicies.clone())?;
        println!("YOOO 3");
        let texture_index = renderer.build_and_append_texture(
            "Default Texture",
            Vec::from(DEFAULT_TEXTURE),
            "clamp",
        )?;
        println!("YOOO 4");

        Ok(Object {
            verticies,
            indicies,
            window_size,
            pipeline: Pipeline {
                shader_index,
                vertex_buffer_index,
                texture_index,
                uniform_index,
            },
            pipeline_id: None,
            width: 100.0,
            height: 100.0,
        })
    }

    pub fn scale(&mut self, width: f32, height: f32) {
        for i in self.verticies.iter_mut() {
            i.position[0] *= width;
            i.position[1] *= height;
        }

        self.width *= width;
        self.height *= height;
        println!("a{} | {}", self.width, self.height);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        let difference_in_width = normalize(self.width, self.window_size.width)
            - normalize(width, self.window_size.width);
        let difference_in_height = normalize(self.height, self.window_size.height)
            - normalize(height, self.window_size.height);

        for i in self.verticies.iter_mut() {
            i.position[0] *= difference_in_width;
            i.position[1] *= difference_in_height;
        }

        self.width = width;
        self.height = height;
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        for i in self.verticies.iter_mut() {
            i.position[0] += x;
            i.position[1] += y;
        }
    }

    pub fn update(&mut self, window_size: winit::dpi::PhysicalSize<u32>) {
        println!("b{} | {}", self.width, self.height);
        let normalized_width = normalize(self.width, window_size.width);
        let normalized_height = normalize(self.height, window_size.height);

        for i in self.verticies.iter_mut() {
            i.position[0] *= normalized_width;
            i.position[1] *= normalized_height;
        }

        self.width *= normalized_width;
        self.height *= normalized_height;
        self.window_size = window_size;
        println!("v{} | {}", self.width, self.height);
    }

    pub fn draw(&mut self, renderer: &mut crate::definitions::Renderer) -> anyhow::Result<()> {
        self.pipeline_id = Some(renderer.append_pipeline(self.pipeline)?);

        Ok(())
    }
}

pub fn triangle(
    renderer: &mut crate::definitions::Renderer,
    window_size: winit::dpi::PhysicalSize<u32>,
    camera: crate::utils::camera::Camera,
) -> Result<Object, anyhow::Error> {
    let mut new_triangle = Object::new(
        vec![
            Vertex {
                position: [0.0, 1.0, 0.0],
                texture: [0.5, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                texture: [1.0, 1.0],
            },
        ],
        vec![0, 1, 2],
        renderer,
        window_size,
        camera,
    )?;
    new_triangle.update(window_size);

    Ok(new_triangle)
}

pub fn square(
    renderer: &mut crate::definitions::Renderer,
    window_size: winit::dpi::PhysicalSize<u32>,
    camera: crate::utils::camera::Camera,
) -> Result<Object, anyhow::Error> {
    let mut new_square = Object::new(
        vec![
            Vertex {
                position: [-1.0, 1.0, 0.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, 0.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                texture: [0.0, 0.0],
            },
        ],
        vec![0, 1, 3, 1, 2, 3],
        renderer,
        window_size,
        camera,
    )?;
    new_square.update(window_size);

    Ok(new_square)
}

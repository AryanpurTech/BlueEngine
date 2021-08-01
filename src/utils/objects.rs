use crate::definitions::{Pipeline, Renderer, UniformBuffer, Vertex, normalize, uniform_type};
use crate::utils::default_resources::{DEFAULT_COLOR, DEFAULT_SHADER, DEFAULT_TEXTURE};

pub struct Object {
    pub name: Option<&'static str>,
    pub verticies: Vec<Vertex>,
    pub indicies: Vec<u16>,
    pub window_size: winit::dpi::PhysicalSize<u32>,
    pub pipeline: Pipeline,
    pub pipeline_id: Option<usize>,
    pub width: f32,
    pub height: f32,
}

impl Object {
    pub fn new(
        name: Option<&'static str>,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
        renderer: &mut Renderer,
        window_size: winit::dpi::PhysicalSize<u32>,
        camera: crate::utils::camera::Camera,
    ) -> anyhow::Result<Self> {
        let mut normalized_verticies = verticies;
        let normalized_width = normalize(100.0, window_size.width);
        let normalized_height = normalize(100.0, window_size.height);
        for i in normalized_verticies.iter_mut() {
            i.position[0] *= normalized_width;
            i.position[1] *= normalized_height;
        }

        let uniform_index = Some(renderer.build_and_append_uniform_buffers(Vec::from([
            UniformBuffer::Matrix("Camera", camera.new_camera_uniform_buffer()?),
            UniformBuffer::Array(
                "Color",
                uniform_type::Array {
                    data: DEFAULT_COLOR,
                },
            ),
        ]))?);
        let shader_index =
            renderer.build_and_append_shaders("Default Shader", DEFAULT_SHADER.to_string())?;
        let vertex_buffer_index =
            renderer.build_and_append_vertex_buffers(normalized_verticies.clone(), indicies.clone())?;
        let texture_index = renderer.build_and_append_texture(
            "Default Texture",
            Vec::from(DEFAULT_TEXTURE),
            "clamp",
        )?;

        Ok(Object {
            name,
            verticies: normalized_verticies,
            indicies,
            window_size,
            pipeline: Pipeline {
                shader_index,
                vertex_buffer_index,
                texture_index,
                uniform_index,
            },
            pipeline_id: None,
            width: 100.0 * normalized_width,
            height: 100.0 * normalized_height,
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

    pub fn no_stretch_update(&mut self, renderer: &mut Renderer, window_size: winit::dpi::PhysicalSize<u32>) {
        let normalized_width = normalize(self.width, window_size.width);
        let normalized_height = normalize(self.height, window_size.height);

        for i in self.verticies.iter_mut() {
            i.position[0] *= normalized_width;
            i.position[1] *= normalized_height;
        }

        self.width *= normalized_width;
        self.height *= normalized_height;
        self.window_size = window_size;
        
        //let pipeline = renderer.
    }

    pub fn draw(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        self.pipeline_id = Some(renderer.append_pipeline(self.pipeline)?);

        Ok(())
    }
}

pub fn triangle(
    name: Option<&'static str>,
    renderer: &mut Renderer,
    window_size: winit::dpi::PhysicalSize<u32>,
    camera: crate::utils::camera::Camera,
) -> Result<Object, anyhow::Error> {
    let new_triangle = Object::new(
        name,
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

    Ok(new_triangle)
}

pub fn square(
    name: Option<&'static str>,
    renderer: &mut Renderer,
    window_size: winit::dpi::PhysicalSize<u32>,
    camera: crate::utils::camera::Camera,
) -> Result<Object, anyhow::Error> {
    let new_square = Object::new(
        name,
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

    Ok(new_square)
}

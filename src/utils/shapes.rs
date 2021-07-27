use crate::definitions::{normalize, Vertex};

pub struct Object {
    pub verticies: Vec<Vertex>,
    pub indicies: Vec<u16>,
    window_size: winit::dpi::PhysicalSize<u32>,
    pipeline_queue: Option<usize>,
    width: f32,
    height: f32,
}

const DEFAULT_SHADER: &[u8] = b"
[[stage(vertex)]]
fn vertex()
";

// ? change to Object struct and globalize stuff.
// ? Also this way add automation of pipeline creation
// ? using default values for everything unless needed, including WGSL shaders for stuff
// ? this should also help out in customization of pipeline, like way of rendering, wether to include textures, ...

impl Object {
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
}

pub fn triangle(window_size: winit::dpi::PhysicalSize<u32>) -> Result<Object, anyhow::Error> {
    Ok(Object {
        verticies: vec![
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
        indicies: vec![0, 1, 2],
        window_size,
        pipeline_queue: None,
        width: 100.0,
        height: 100.0,
    })
}

pub fn square(window_size: winit::dpi::PhysicalSize<u32>) -> Result<Object, anyhow::Error> {
    Ok(Object {
        verticies: vec![
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
        indicies: vec![0, 1, 3, 1, 2, 3],
        window_size,
        pipeline_queue: None,
        width: 100.0,
        height: 100.0,
    })
}

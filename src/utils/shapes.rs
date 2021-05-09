use crate::definitions::{normalize, Vertex};

pub struct Shape2D {
    pub verticies: Vec<Vertex>,
    pub indicies: Vec<u16>,
    width: f32,
    height: f32,
}

impl Shape2D {
    pub fn scale(&mut self, width: f32, height: f32) {
        for i in self.verticies.iter_mut() {
            i.position[0] *= width;
            i.position[1] *= height;
        }

        self.width *= width;
        self.height *= height;
        println!("a{} | {}", self.width, self.height);
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
        println!("v{} | {}", self.width, self.height);
    }
}

pub fn triangle() -> Result<Shape2D, anyhow::Error> {
    Ok(Shape2D {
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
        width: 100.0,
        height: 100.0,
    })
}

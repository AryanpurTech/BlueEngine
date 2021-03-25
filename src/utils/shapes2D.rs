use winit::dpi::PhysicalSize;

use crate::definitions::{normalize, size_normalize, Vertex};

pub struct Shape2D {
    pub verticies: Vec<Vertex>,
    pub indecies: Vec<u16>,
    pub width: u32,
    pub height: u32
}

impl Shape2D {
    pub fn resize(&mut self, width: u32, height: u32) {
        let normalized_width = size_normalize(width, self.width);
        let normalized_height = size_normalize(height, self.height);

        for i in self.verticies.iter_mut() {
            i.position[0] *= normalized_width;
            i.position[1] *= normalized_height;
        }

        self.width = width;
        self.height = height;
    }
}

pub fn triangle(window_size: PhysicalSize<u32>) -> Result<Shape2D, anyhow::Error> {
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
        indecies: vec![0, 1, 2],
        width: window_size.width,
        height: window_size.height
    })
}

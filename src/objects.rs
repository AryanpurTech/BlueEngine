/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::{
    normalize, uniform_type, Engine, Object, Pipeline, Renderer, RotateAxis, UniformBuffer, Vertex,
};
use crate::utils::default_resources::{DEFAULT_COLOR, DEFAULT_MATRIX_4, DEFAULT_SHADER, DEFAULT_TEXTURE};

impl Engine {
    pub fn new_object(
        &mut self,
        name: Option<&'static str>,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
    ) -> anyhow::Result<usize> {
        let mut normalized_verticies = verticies;

        let normalized_width = normalize(100.0, self.window.inner_size().width);
        let normalized_height = normalize(100.0, self.window.inner_size().height);
        let normalized_depth = normalize(100.0, self.window.inner_size().width);

        for i in normalized_verticies.iter_mut() {
            i.position[0] *= normalized_width;
            i.position[1] *= normalized_height;
            i.position[2] *= normalized_depth;
        }
        let vertex_buffer_index = self
            .renderer
            .build_and_append_vertex_buffers(normalized_verticies.clone(), indicies.clone())?;
        
        let uniform_index =
            self.renderer
                .build_and_append_uniform_buffers(vec![UniformBuffer::Matrix(
                    "Transformation Matrix",
                    uniform_type::Matrix::from_glm(DEFAULT_MATRIX_4),
                )])?;

        let shader_index = self.renderer.build_and_append_shaders(
            name.unwrap_or("Object"),
            DEFAULT_SHADER.to_string(),
            Some(&uniform_index.1),
        )?;

        let index = self.objects.len();
        self.objects.push(Object {
            name,
            vertices: normalized_verticies,
            indices: indicies,
            pipeline: Pipeline {
                vertex_buffer_index,
                shader_index: shader_index,
                texture_index: 0,
                uniform_index: Some(uniform_index.0),
            },
            window_size: self.window.inner_size(),
            pipeline_id: None,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
            changed: false,
            transformation_matrix: DEFAULT_MATRIX_4,
            color: uniform_type::Array {
                data: DEFAULT_COLOR,
            },
        });
        let item = self.objects.get_mut(index).unwrap();
        item.pipeline_id = Some(self.renderer.append_pipeline(item.pipeline)?);

        Ok(index)
    }

    pub fn get_object(&mut self, index: usize) -> anyhow::Result<&mut Object> {
        Ok(self.objects.get_mut(index).unwrap())
    }
}
impl Object {
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        for i in self.vertices.iter_mut() {
            i.position[0] *= x;
            i.position[1] *= y;
            i.position[2] *= z;
        }

        self.width *= x;
        self.height *= y;
        self.depth *= z;

        self.changed = true;
    }

    pub fn resize(&mut self, width: f32, height: f32, depth: f32) {
        let difference_in_width = if self.width != 0.0 && width != 0.0 {
            normalize(width, self.window_size.width) / normalize(self.width, self.window_size.width)
        } else {
            0.0
        };
        let difference_in_height = if self.height != 0.0 && height != 0.0 {
            normalize(height, self.window_size.height)
                / normalize(self.height, self.window_size.height)
        } else {
            0.0
        };
        let difference_in_depth = if self.depth != 0.0 && depth != 0.0 {
            normalize(depth, self.window_size.width) / normalize(self.depth, self.window_size.width)
        } else {
            0.0
        };

        self.scale(
            difference_in_width,
            difference_in_height,
            difference_in_depth,
        );
    }

    pub fn rotate(&mut self, angle: f32, axis: RotateAxis) {
        let mut rotation_matrix = self.transformation_matrix;
        rotation_matrix = glm::ext::rotate(
            &rotation_matrix,
            angle,
            match axis {
                RotateAxis::X => glm::vec3(1.0, 0.0, 0.0),
                RotateAxis::Y => glm::vec3(0.0, 1.0, 0.0),
                RotateAxis::Z => glm::vec3(0.0, 0.0, 1.0),
            },
        );
        

        self.changed = true;
    }

    pub fn position(&mut self, x: f32, y: f32, z: f32) {
        todo!();
    }

    pub fn update(
        &mut self,
        renderer: &mut Renderer,
        window_size: winit::dpi::PhysicalSize<u32>,
    ) -> anyhow::Result<()> {
        self.update_vertex_buffer(renderer)?;

        self.window_size = window_size;
        self.changed = false;

        Ok(())
    }

    fn update_vertex_buffer(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let updated_buffer =
            renderer.build_vertex_buffers(self.vertices.clone(), self.indices.clone())?;
        let _ = std::mem::replace(
            &mut renderer.vertex_buffers[self.pipeline.vertex_buffer_index],
            updated_buffer,
        );

        Ok(())
    }

    fn update_uniform_buffer(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let updated_buffer =
            renderer.build_vertex_buffers(self.vertices.clone(), self.indices.clone())?;
        let _ = std::mem::replace(
            &mut renderer.vertex_buffers[self.pipeline.vertex_buffer_index],
            updated_buffer,
        );

        Ok(())
    }
}

pub fn triangle(name: Option<&'static str>, engine: &mut Engine) -> Result<usize, anyhow::Error> {
    let new_triangle = engine.new_object(
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
    )?;

    Ok(new_triangle)
}

pub fn square(name: Option<&'static str>, engine: &mut Engine) -> Result<usize, anyhow::Error> {
    let new_square = engine.new_object(
        name,
        vec![
            Vertex {
                position: [1.0, 1.0, 0.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, 1.0, 0.0],
                texture: [0.0, 0.0],
            },
        ],
        vec![2, 1, 0, 2, 0, 3],
    )?;

    Ok(new_square)
}

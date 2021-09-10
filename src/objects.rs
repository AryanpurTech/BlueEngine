/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::definitions::{
    normalize, uniform_type, Engine, Object, Pipeline, Renderer, RotateAxis, UniformBuffer, Vertex,
};
use crate::utils::default_resources::{
    DEFAULT_COLOR, DEFAULT_MATRIX_4, DEFAULT_SHADER, DEFAULT_TEXTURE,
};

impl Engine {
    pub fn new_object(
        &mut self,
        name: Option<&'static str>,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
        camera: crate::utils::camera::Camera,
    ) -> anyhow::Result<usize> {
        let mut normalized_verticies = verticies;

        let normalized_width = normalize(100.0, self.window.inner_size().width);
        let normalized_height = normalize(100.0, self.window.inner_size().height);
        let normalized_depth = normalize(100.0, self.window.inner_size().width);

        for i in normalized_verticies.iter_mut() {
            i.position[0] *= normalized_width; // ! Fix the size of default not being the size intented
            i.position[1] *= normalized_height;
            i.position[2] *= normalized_depth;
        }

        let uniform_index = Some(self.renderer.build_and_append_uniform_buffers(Vec::from([
            UniformBuffer::Matrix("View", camera.new_camera_uniform_buffer()?),
            UniformBuffer::Array(
                "Color",
                uniform_type::Array {
                    data: DEFAULT_COLOR,
                },
            ),
        ]))?);
        let shader_index = self
            .renderer
            .build_and_append_shaders("Default Shader", DEFAULT_SHADER.to_string())?;
        let vertex_buffer_index = self
            .renderer
            .build_and_append_vertex_buffers(normalized_verticies.clone(), indicies.clone())?;
        let texture_index = self.renderer.build_and_append_texture(
            "Default Texture",
            Vec::from(DEFAULT_TEXTURE),
            "clamp",
        )?;

        let index = self.objects.len();
        self.objects.push(Object {
            name,
            verticies: normalized_verticies,
            indicies,
            pipeline: Pipeline {
                shader_index,
                vertex_buffer_index,
                texture_index,
                uniform_index,
            },
            window_size: self.window.inner_size(),
            pipeline_id: None,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
            changed: false,
            transformation_matrix: camera.new_camera_uniform_buffer()?,
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
        for i in self.verticies.iter_mut() {
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
        todo!();
        let mut rotation_matrix = DEFAULT_MATRIX_4;
        rotation_matrix = glm::ext::rotate(
            &rotation_matrix,
            angle,
            match axis {
                RotateAxis::X => glm::vec3(1.0, 0.0, 0.0),
                RotateAxis::Y => glm::vec3(0.0, 1.0, 0.0),
                RotateAxis::Z => glm::vec3(0.0, 0.0, 1.0),
            },
        );
        for i in self.verticies.iter_mut() {
            let vertex =
                rotation_matrix * glm::vec4(i.position[0], i.position[1], i.position[2], 1.0);
            i.position[0] = vertex.x;
            i.position[1] = vertex.y;
            i.position[2] = vertex.z;
        }

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
            renderer.build_vertex_buffers(self.verticies.clone(), self.indicies.clone())?;
        let _ = std::mem::replace(
            &mut renderer.vertex_buffers[self.pipeline.vertex_buffer_index],
            updated_buffer,
        );

        Ok(())
    }

    fn update_uniform_buffer(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let updated_buffer =
            renderer.build_vertex_buffers(self.verticies.clone(), self.indicies.clone())?;
        let _ = std::mem::replace(
            &mut renderer.vertex_buffers[self.pipeline.vertex_buffer_index],
            updated_buffer,
        );

        Ok(())
    }
}

pub fn triangle(
    name: Option<&'static str>,
    engine: &mut Engine,
    camera: crate::utils::camera::Camera,
) -> Result<usize, anyhow::Error> {
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
        camera,
    )?;

    Ok(new_triangle)
}

pub fn square<'a>(
    name: Option<&'static str>,
    engine: &mut Engine,
    camera: crate::utils::camera::Camera,
) -> Result<usize, anyhow::Error> {
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
        camera,
    )?;

    Ok(new_square)
}

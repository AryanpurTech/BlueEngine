/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::uniform_type::Array;
use crate::header::{Engine, Object, Pipeline, Renderer, RotateAxis, UniformBuffer, Vertex, normalize, uniform_type};
use crate::utils::default_resources::{DEFAULT_COLOR, DEFAULT_MATRIX_4, DEFAULT_SHADER};

impl Engine {
    /// Creates a new object
    pub fn new_object(
        &mut self,
        name: Option<&'static str>,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
    ) -> anyhow::Result<usize> {
        let vertex_buffer_index = self
            .renderer
            .build_and_append_vertex_buffers(verticies.clone(), indicies.clone())?;

        let uniform_index = self.renderer.build_and_append_uniform_buffers(vec![
            UniformBuffer::Matrix(
                "Transformation Matrix",
                uniform_type::Matrix::from_glm(DEFAULT_MATRIX_4),
            ),
            UniformBuffer::Array(
                "Color",
                uniform_type::Array {
                    data: DEFAULT_COLOR,
                },
            ),
        ])?;

        let shader_index = self.renderer.build_and_append_shaders(
            name.unwrap_or("Object"),
            DEFAULT_SHADER.to_string(),
            Some(&uniform_index.1),
        )?;

        let index = self.objects.len();
        self.objects.push(Object {
            name,
            vertices: verticies,
            indices: indicies,
            pipeline: (
                Pipeline {
                    vertex_buffer_index,
                    shader_index: shader_index,
                    texture_index: 0,
                    uniform_index: Some(uniform_index.0),
                },
                None,
            ),
            size: (self.window.inner_size().width as f32, self.window.inner_size().height as f32, 0f32),
            position: (0.0, 0.0, 0.0),
            changed: false,
            transformation_matrix: DEFAULT_MATRIX_4,
            color: uniform_type::Array {
                data: DEFAULT_COLOR,
            },
            object_index: self.objects.len()
        });
        let object = self.objects.get_mut(index).unwrap();
        object.pipeline = (
            object.pipeline.0,
            Some(self.renderer.append_pipeline(object.pipeline.0)?),
        );
        object.scale(0.1, 0.1, 0.1);

        Ok(index)
    }

    /// Returns mutable object
    pub fn get_object(&mut self, index: usize) -> anyhow::Result<&mut Object> {
        Ok(self.objects.get_mut(index).unwrap())
    }
}
impl Object {
    /// Scales an object. e.g. 2.0 doubles the size and 0.5 halves
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        for i in self.vertices.iter_mut() {
            i.position[0] *= x;
            i.position[1] *= y;
            i.position[2] *= z;
        }

        self.size.0 *= x;
        self.size.1 *= y;
        self.size.2 *= z;

        self.changed = true;
    }

    /// Resizes an object in pixels which are relative to the window
    pub fn resize(
        &mut self,
        width: f32,
        height: f32,
        depth: f32,
        window_size: winit::dpi::PhysicalSize<u32>,
    ) {
        let difference_in_width = if self.size.0 != 0.0 && width != 0.0 {
            normalize(width, window_size.width) / normalize(self.size.0, window_size.width)
        } else {
            0.0
        };
        let difference_in_height = if self.size.1 != 0.0 && height != 0.0 {
            normalize(height, window_size.height) / normalize(self.size.1, window_size.height)
        } else {
            0.0
        };
        let difference_in_depth = if self.size.2 != 0.0 && depth != 0.0 {
            normalize(depth, window_size.width) / normalize(self.size.2, window_size.width)
        } else {
            0.0
        };

        self.scale(
            difference_in_width,
            difference_in_height,
            difference_in_depth,
        );
    }

    /// Rotates the object in the axis you specify
    pub fn rotate(&mut self, angle: f32, axis: RotateAxis) {
        let mut rotation_matrix = self.transformation_matrix;
        rotation_matrix = glm::ext::rotate(
            &rotation_matrix,
            angle,
            match axis {
                RotateAxis::Z => glm::vec3(0.0, 0.0, 1.0),
                RotateAxis::X => glm::vec3(0.0, 1.0, 0.0),
                RotateAxis::Y => glm::vec3(1.0, 0.0, 0.0),
            },
        );
        self.transformation_matrix = rotation_matrix;

        self.changed = true;
    }

    /// Moves the object by the amount you specify in the axis you specify
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let mut position_matrix = self.transformation_matrix;
        position_matrix = glm::ext::translate(&position_matrix, glm::vec3(x, y, z));
        self.transformation_matrix = position_matrix;

        self.changed = true;
    }

    /// Sets the position of the object in 3D space relative to the window
    pub fn position(&mut self, x: f32, y: f32, z: f32, window_size: winit::dpi::PhysicalSize<u32>) {
        let difference = glm::sqrt(
            glm::pow(self.position.0 - x, 2.0)
                + glm::pow(self.position.1 - y, 2.0)
                + glm::pow(self.position.2 - z, 2.0),
        );

        let normalized_target_x = if (self.position.0 - x) == 0.0 {
            0.0
        } else {
            let new_difference = normalize(difference, window_size.width);
            if self.position.0 > x {
                new_difference * -1.0
            } else {
                new_difference
            }
        };
        let normalized_target_y = if (self.position.1 - y) == 0.0 {
            0.0
        } else {
            let new_difference = normalize(difference, window_size.height);
            if self.position.1 > y {
                new_difference * -1.0
            } else {
                new_difference
            }
        };
        let normalized_target_z = if (self.position.2 - z) == 0.0 {
            0.0
        } else {
            let new_difference = normalize(difference, window_size.width);
            if self.position.2 > z {
                new_difference * -1.0
            } else {
                new_difference
            }
        };

        self.position.0 = x;
        self.position.1 = y;
        self.position.2 = z;

        self.translate(
            normalized_target_x,
            normalized_target_y,
            normalized_target_z,
        );
    }

    /// Changes the color of the object. If textures exist, the color of textures will change
    pub fn change_color(
        &mut self,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) -> anyhow::Result<()> {
        self.color = Array {
            data: [red, green, blue, alpha],
        };
        self.changed = true;

        Ok(())
    }

    /// Replaces the object's texture with provided one
    pub fn change_texture(&mut self, texture_index: usize) -> anyhow::Result<()> {
        self.pipeline.0.texture_index = texture_index;
        self.changed = true;

        
        Ok(())
    }

    /// Update and apply changes done to an object
    pub fn update(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        self.update_vertex_buffer(renderer)?;
        self.update_uniform_buffer(renderer)?;
        self.update_pipeline(renderer)?;
        self.changed = false;
        Ok(())
    }

    pub(crate) fn update_pipeline(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        //let pipeline = renderer.get_pipeline(self.pipeline.1.unwrap())?;
        let _ = std::mem::replace(
            &mut renderer.render_pipelines[self.pipeline.1.unwrap()],
            self.pipeline.0,
        );

        Ok(())
    }

    pub(crate) fn update_vertex_buffer(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let updated_buffer =
            renderer.build_vertex_buffers(self.vertices.clone(), self.indices.clone())?;
        let _ = std::mem::replace(
            &mut renderer.vertex_buffers[self.pipeline.0.vertex_buffer_index],
            updated_buffer,
        );

        Ok(())
    }

    pub(crate) fn update_uniform_buffer(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let updated_buffer = renderer
            .build_uniform_buffer(vec![
                UniformBuffer::Matrix(
                    "Transformation Matrix",
                    uniform_type::Matrix::from_glm(self.transformation_matrix),
                ),
                UniformBuffer::Array("Color", self.color),
            ])?
            .0;

        let _ = std::mem::replace(
            &mut renderer.uniform_bind_group[self.pipeline.0.uniform_index.unwrap()],
            updated_buffer,
        );

        Ok(())
    }
}

/// Creates a 2D triangle
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

/// Creates a 2D square
pub fn square(name: Option<&'static str>, engine: &mut Engine) -> Result<usize, anyhow::Error> {
    let new_square = engine.new_object(
        name,
        vec![
            Vertex {
                position: [1.0, 1.0, 0.0],
                texture: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                texture: [1.0, 1.0],
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

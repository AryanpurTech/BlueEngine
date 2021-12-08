/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::uniform_type::Array;
use crate::header::{
    normalize, uniform_type, Engine, Object, ObjectSettings, Pipeline, Renderer, RotateAxis,
    UniformBuffer, Vertex,
};
use crate::utils::default_resources::{DEFAULT_MATRIX_4, DEFAULT_SHADER};
pub mod two_dimensions;

impl Engine {
    /// Creates a new object
    pub fn new_object(
        &mut self,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
        settings: ObjectSettings,
    ) -> anyhow::Result<usize> {
        let vertex_buffer_index = self
            .renderer
            .build_and_append_vertex_buffers(verticies.clone(), indicies.clone())?;

        let uniform_index = self.renderer.build_and_append_uniform_buffers(vec![
            UniformBuffer::Matrix("Transformation Matrix", DEFAULT_MATRIX_4),
            UniformBuffer::Array("Color", settings.color),
        ])?;

        let shader_index = self.renderer.build_and_append_shaders(
            settings.name.unwrap_or("Object"),
            DEFAULT_SHADER.to_string(),
            Some(&uniform_index.1),
            settings.shader_settings,
        )?;

        let index = self.objects.len();
        self.objects.push(Object {
            name: settings.name,
            vertices: verticies,
            indices: indicies,
            pipeline: (
                Pipeline {
                    vertex_buffer_index,
                    shader_index: shader_index,
                    texture_index: settings.texture_index,
                    uniform_index: Some(uniform_index.0),
                },
                None,
            ),
            uniform_layout: uniform_index.1,
            size: (
                self.window.inner_size().width as f32,
                self.window.inner_size().height as f32,
                0f32,
            ),
            position: (0f32, 0f32, 0f32),
            changed: false,
            transformation_matrix: DEFAULT_MATRIX_4.to_im(),
            color: settings.color,
            object_index: self.objects.len(),
            camera_effect: settings.camera_effect,
            shader_settings: settings.shader_settings,
        });
        let object = self.objects.get_mut(index).unwrap();
        object.pipeline = (
            object.pipeline.0,
            Some(self.renderer.append_pipeline(object.pipeline.0)?),
        );
        object.scale(0.1, 0.1, 0.1);
        object.resize(
            settings.size.0,
            settings.size.1,
            settings.size.2,
            self.window.inner_size(),
        );
        object.position(
            settings.position.0,
            settings.position.1,
            settings.position.2,
            self.window.inner_size(),
        );
        //object.update(&mut self.renderer)?;

        Ok(index)
    }

    /// Returns mutable object
    pub fn get_object(&mut self, index: usize) -> Option<&mut Object> {
        self.objects.get_mut(index)
    }
} // ? make the Shader Builder, add customizations to the objects, and fix bugs boi
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
        self.update_shader(renderer)?;
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

    pub(crate) fn update_shader(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let updated_shader = renderer.build_shaders(
            self.name.unwrap_or("Object"),
            self.build_shader(),
            Some(&self.uniform_layout),
            self.shader_settings,
        )?;
        let _ = std::mem::replace(
            &mut renderer.shaders[self.pipeline.0.shader_index],
            updated_shader,
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

    pub(crate) fn build_shader(&self) -> String {
        // step 1 define blocks
        let blocks = format!(
            "\n{}\n{}\n{}",
            r#"[[block]]
struct TransformationUniforms {
    transform_matrix: mat4x4<f32>;
};
[[group(2), binding(0)]]
var<uniform> transform_uniform: TransformationUniforms;"#,
            r#"[[block]]
struct FragmentUniforms {
    color: vec4<f32>;
};
[[group(2), binding(1)]]
var<uniform> fragment_uniforms: FragmentUniforms;"#,
            if self.camera_effect {
                r#"[[block]]
struct CameraUniforms {
    camera_matrix: mat4x4<f32>;
};
[[group(1), binding(0)]]
var<uniform> camera_uniform: CameraUniforms;"#
            } else {
                ""
            }
        );

        // step 2 define input and output for vertex
        let input_and_output = format!(
            "\n{}",
            r#"struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] texture_coordinates: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] texture_coordinates: vec2<f32>;
};"#
        );

        // step 3 define texture data
        let texture_data = format!(
            "\n{}",
            r#"[[group(0), binding(0)]]
var texture_diffuse: texture_2d<f32>;

[[group(0), binding(1)]]
var sampler_diffuse: sampler;"#
        );

        // step 4 vertex stage according to data before
        let vertex_stage = format!(
            "\n// ===== VERTEX STAGE ===== //\n{}\n{}\n{}",
            r#"[[stage(vertex)]]
fn main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.texture_coordinates = input.texture_coordinates;"#,
            if self.camera_effect {
                "out.position = camera_uniform.camera_matrix * (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0));"
            } else {
                "out.position = transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0);"
            },
            r#"return out;
}"#
        );

        // step 5 fragment stage
        let fragment_stage = format!(
            "\n// ===== Fragment STAGE ===== //\n{}",
            r#"[[stage(fragment)]]
fn main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates) * fragment_uniforms.color;
}"#
        );

        format!(
            "{}{}{}{}{}",
            blocks, input_and_output, texture_data, vertex_stage, fragment_stage
        )
    }
}

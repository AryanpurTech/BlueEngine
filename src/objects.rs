/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::{
    normalize, uniform_type, Engine, Object, ObjectSettings, Pipeline, Renderer, RotateAxis,
    TextureData, Textures, UniformBuffer, Vertex,
};
use crate::uniform_type::Array4;
use crate::utils::default_resources::{DEFAULT_MATRIX_4, DEFAULT_SHADER, DEFAULT_TEXTURE};

impl Engine {
    /// Creates a new object
    pub fn new_object(
        &mut self,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
        settings: ObjectSettings,
    ) -> anyhow::Result<&mut Object> {
        let vertex_buffer = self
            .renderer
            .build_vertex_buffer(verticies.clone(), indicies.clone())?;

        let uniform = self.renderer.build_uniform_buffer(vec![
            UniformBuffer::Matrix("Transformation Matrix", DEFAULT_MATRIX_4),
            UniformBuffer::Array4("Color", settings.color),
        ])?;

        let shader = self.renderer.build_shader(
            settings.name.unwrap_or("Object"),
            DEFAULT_SHADER.to_string(),
            Some(&uniform.1),
            settings.shader_settings,
        )?;

        let texture = self.renderer.build_texture(
            "Default Texture",
            TextureData::Bytes(DEFAULT_TEXTURE.to_vec()),
            crate::header::TextureMode::Clamp,
            //crate::header::TextureFormat::PNG
        )?;

        let index = self.objects.len();
        self.objects.push(Object {
            name: settings.name,
            vertices: verticies,
            indices: indicies,
            pipeline: Pipeline {
                vertex_buffer,
                shader: shader,
                texture: texture,
                uniform: Some(uniform.0),
            },
            uniform_layout: uniform.1,
            size: settings.size,
            scale: settings.scale,
            position: (
                settings.position.0,
                settings.position.1,
                settings.position.2,
            ),
            changed: false,
            transformation_matrix: DEFAULT_MATRIX_4.to_im(),
            main_color: settings.color,
            color: settings.color,
            object_index: self.objects.len(),
            shader_builder: ShaderBuilder::new(settings.camera_effect),
            shader_settings: settings.shader_settings,
            camera_effect: settings.camera_effect,
            uniform_buffers: vec![
                UniformBuffer::Matrix("Transformation Matrix", DEFAULT_MATRIX_4),
                UniformBuffer::Array4("Color", settings.color),
            ],
        });
        let object = self.objects.get_mut(index).unwrap();
        object.scale(settings.scale.0, settings.scale.1, settings.scale.2);
        object.position(
            settings.position.0,
            settings.position.1,
            settings.position.2,
        );
        //object.update(&mut self.renderer)?;

        Ok(object)
    }

    /// Returns mutable object
    pub fn get_object(&mut self, index: usize) -> Option<&mut Object> {
        self.objects.get_mut(index)
    }
}
impl Object {
    /// Scales an object. e.g. 2.0 doubles the size and 0.5 halves
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        /*for i in self.vertices.iter_mut() {
            i.position[0] *= x;
            i.position[1] *= y;
            i.position[2] *= z;
        }*/

        self.size.0 *= x;
        self.size.1 *= y;
        self.size.2 *= z;

        let transformation_matrix = self.transformation_matrix;
        let result = nalgebra_glm::scale(&transformation_matrix, &nalgebra_glm::vec3(x, y, z));
        self.transformation_matrix = result;
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
            let a = normalize(width, window_size.width);
            let b = normalize(self.size.0, window_size.width);
            if a != 0f32 && b != 0f32 {
                a / b
            } else {
                b
            }
        } else {
            0.0
        };
        let difference_in_height = if self.size.1 != 0.0 && height != 0.0 {
            let a = normalize(height, window_size.height);
            let b = normalize(self.size.1, window_size.height);
            if a != 0f32 && b != 0f32 {
                a / b
            } else {
                b
            }
        } else {
            0.0
        };
        let difference_in_depth = if self.size.2 != 0.0 && depth != 0.0 {
            let a = normalize(depth, window_size.width);
            let b = normalize(self.size.2, window_size.width);
            if a != 0f32 && b != 0f32 {
                a / b
            } else {
                b
            }
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
        let axis = match axis {
            RotateAxis::Z => nalgebra_glm::vec3(0.0, 0.0, 1.0),
            RotateAxis::X => nalgebra_glm::vec3(0.0, 1.0, 0.0),
            RotateAxis::Y => nalgebra_glm::vec3(1.0, 0.0, 0.0),
        };
        rotation_matrix = nalgebra_glm::rotate(&rotation_matrix, angle, &axis);
        self.transformation_matrix = rotation_matrix;

        self.changed = true;
    }

    /// Moves the object by the amount you specify in the axis you specify
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let mut position_matrix = self.transformation_matrix;
        position_matrix = nalgebra_glm::translate(&position_matrix, &nalgebra_glm::vec3(x, y, z));
        self.transformation_matrix = position_matrix;

        self.changed = true;
    }

    /// Sets the position of the object in 3D space relative to the window
    pub fn position(&mut self, x: f32, y: f32, z: f32) {
        /*
        let difference = ((self.position.0 - x).powf(2.0)
            + (self.position.1 - y).powf(2.0)
            + (self.position.2 - z).powf(2.0))
        .sqrt();
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
        };*/

        self.translate(
            self.position.0 - x,
            self.position.1 - y,
            self.position.2 - z,
        );

        self.position.0 = x;
        self.position.1 = y;
        self.position.2 = z;
    }

    /// Changes the color of the object. If textures exist, the color of textures will change
    pub fn set_color(&mut self, red: f32, green: f32, blue: f32, alpha: f32) -> anyhow::Result<()> {
        self.color = Array4 {
            data: [red, green, blue, alpha],
        };
        self.changed = true;

        Ok(())
    }

    /// Changes the main color of the object. If textures exist, the color of textures will change
    pub fn set_main_color(
        &mut self,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) -> anyhow::Result<()> {
        self.main_color = Array4 {
            data: [red, green, blue, alpha],
        };
        self.changed = true;

        Ok(())
    }

    /// Replaces the object's texture with provided one
    pub fn set_texture(&mut self, texture: Textures) -> anyhow::Result<()> {
        self.pipeline.texture = texture;
        self.changed = true;

        Ok(())
    }

    /// Update and apply changes done to an object
    pub fn update(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        self.update_vertex_buffer(renderer)?;
        self.update_uniform_buffer(renderer)?;
        self.update_shader(renderer)?;
        self.changed = false;
        Ok(())
    }

    pub(crate) fn update_vertex_buffer(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let updated_buffer =
            renderer.build_vertex_buffer(self.vertices.clone(), self.indices.clone())?;
        self.pipeline.vertex_buffer = updated_buffer;

        Ok(())
    }

    pub(crate) fn update_shader(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let updated_shader = renderer.build_shader(
            self.name.unwrap_or("Object"),
            self.shader_builder.build_shader(),
            Some(&self.uniform_layout),
            self.shader_settings,
        )?;
        self.pipeline.shader = updated_shader;

        Ok(())
    }

    pub(crate) fn update_uniform_buffer(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        self.uniform_buffers[0] = UniformBuffer::Matrix(
            "Transformation Matrix",
            uniform_type::Matrix::from_im(self.transformation_matrix),
        );
        self.uniform_buffers[1] = UniformBuffer::Array4("Color", self.color);
        let updated_buffer = renderer
            .build_uniform_buffer(self.uniform_buffers.clone())?
            .0;

        self.pipeline.uniform = Some(updated_buffer);

        Ok(())
    }
}

pub struct ShaderBuilder {
    pub blocks: String,
    pub input_and_output: String,
    pub texture_data: String,
    pub vertex_stage: String,
    pub fragment_stage: String,
}

impl ShaderBuilder {
    pub fn new(camera_effect: bool) -> Self {
        Self {
            blocks: format!(
                // step 1 define blocks
                "\n{}\n{}\n{}",
                r#"
struct TransformationUniforms {
    transform_matrix: mat4x4<f32>,
};
@group(2) @binding(0)
var<uniform> transform_uniform: TransformationUniforms;"#,
                r#"
struct FragmentUniforms {
    color: vec4<f32>,
};
@group(2) @binding(1)
var<uniform> fragment_uniforms: FragmentUniforms;"#,
                if camera_effect {
                    r#"
struct CameraUniforms {
    camera_matrix: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera_uniform: CameraUniforms;"#
                } else {
                    ""
                }
            ),
            input_and_output: format!(
                // step 2 define input and output for vertex
                "\n{}",
                r#"struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texture_coordinates: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
};"#
            ),
            texture_data: format!(
                // step 3 define texture data
                "\n{}",
                r#"@group(0) @binding(0)
var texture_diffuse: texture_2d<f32>;

@group(0) @binding(1)
var sampler_diffuse: sampler;"#
            ),
            vertex_stage: format!(
                // step 4 vertex stage according to data before
                "\n// ===== VERTEX STAGE ===== //\n{}\n{}\n{}",
                r#"@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.texture_coordinates = input.texture_coordinates;"#,
                if camera_effect {
                    "out.position = camera_uniform.camera_matrix * (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0));"
                } else {
                    "out.position = transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0);"
                },
                r#"return out;
}"#
            ),
            fragment_stage: format!(
                // step 5 fragment stage
                "\n// ===== Fragment STAGE ===== //\n{}",
                r#"@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates) * fragment_uniforms.color;
}"#
            ),
        }
    }

    pub(crate) fn build_shader(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.blocks,
            self.input_and_output,
            self.texture_data,
            self.vertex_stage,
            self.fragment_stage
        )
    }
}

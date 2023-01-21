/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use crate::header::{
    pixel_to_cartesian, uniform_type, Object, ObjectSettings, Pipeline, Renderer, RotateAxis,
    TextureData, Textures, Vertex,
};
use crate::uniform_type::{Array4, Matrix};
use crate::utils::default_resources::{DEFAULT_MATRIX_4, DEFAULT_SHADER, DEFAULT_TEXTURE};
use crate::{ObjectStorage, StringBuffer};

impl Renderer {
    pub fn build_object(
        &mut self,
        name: impl StringBuffer,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
        settings: ObjectSettings,
    ) -> anyhow::Result<Object> {
        let vertex_buffer = self.build_vertex_buffer(&verticies, &indicies)?;

        let uniform = self.build_uniform_buffer(&vec![
            self.build_uniform_buffer_part("Transformation Matrix", DEFAULT_MATRIX_4),
            self.build_uniform_buffer_part(
                "Color",
                crate::uniform_type::Array4 {
                    data: crate::utils::default_resources::DEFAULT_COLOR,
                },
            ),
        ])?;

        let shader = self.build_shader(
            name.as_str(),
            DEFAULT_SHADER.to_string(),
            Some(&uniform.1),
            settings.shader_settings,
        )?;

        let texture = self.build_texture(
            "Default Texture",
            TextureData::Bytes(DEFAULT_TEXTURE.to_vec()),
            crate::header::TextureMode::Clamp,
            //crate::header::TextureFormat::PNG
        )?;

        Ok(Object {
            name: name.as_string(),
            vertices: verticies,
            indices: indicies,
            pipeline: Pipeline {
                vertex_buffer,
                shader: shader,
                texture: texture,
                uniform: Some(uniform.0),
            },
            uniform_layout: uniform.1,
            size: (100f32, 100f32, 100f32),
            scale: (1f32, 1f32, 1f32),
            position: (0f32, 0f32, 0f32),
            rotation: (0f32, 0f32, 0f32),
            changed: false,
            position_matrix: DEFAULT_MATRIX_4.to_im(),
            scale_matrix: DEFAULT_MATRIX_4.to_im(),
            rotation_matrix: DEFAULT_MATRIX_4.to_im(),
            inverse_transformation_matrix: Matrix::from_im(nalgebra_glm::transpose(
                &nalgebra_glm::inverse(&DEFAULT_MATRIX_4.to_im()),
            )),
            uniform_color: crate::uniform_type::Array4 {
                data: crate::utils::default_resources::DEFAULT_COLOR,
            },
            color: crate::uniform_type::Array4 {
                data: crate::utils::default_resources::DEFAULT_COLOR,
            },
            shader_builder: ShaderBuilder::new(settings.camera_effect),
            shader_settings: settings.shader_settings,
            camera_effect: settings.camera_effect,
            uniform_buffers: vec![
                self.build_uniform_buffer_part("Transformation Matrix", DEFAULT_MATRIX_4),
                self.build_uniform_buffer_part(
                    "Color",
                    crate::uniform_type::Array4 {
                        data: crate::utils::default_resources::DEFAULT_COLOR,
                    },
                ),
            ],
        })
    }
}

impl ObjectStorage {
    /// Creates a new object
    pub fn new_object(
        &mut self,
        name: impl StringBuffer,
        verticies: Vec<Vertex>,
        indicies: Vec<u16>,
        settings: ObjectSettings,
        renderer: &mut Renderer,
    ) -> anyhow::Result<()> {
        self.add_object(
            name.clone(),
            renderer.build_object(name.clone(), verticies, indicies, settings)?,
        )?;

        /*self.update_object(name, |object| {
            object.scale(1f32, 1f32, 1f32);
            object.position(
                0f32, 0f32, 0f32
            );
        }); */

        Ok(())
    }

    pub fn add_object(&mut self, key: impl StringBuffer, object: Object) -> anyhow::Result<()> {
        self.insert(key.as_string(), object);

        Ok(())
    }

    /// Allows for safe update of objects
    pub fn update_object<T: Fn(&mut Object)>(&mut self, key: impl StringBuffer, callback: T) {
        let object = self.get_mut(&key.as_string());
        if object.is_some() {
            callback(object.unwrap());
        }
    }
}

impl Object {
    /// Scales an object. e.g. 2.0 doubles the size and 0.5 halves
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.size.0 *= x;
        self.size.1 *= y;
        self.size.2 *= z;

        let transformation_matrix = self.scale_matrix;
        let result = nalgebra_glm::scale(&transformation_matrix, &nalgebra_glm::vec3(x, y, z));
        self.scale_matrix = result;
        self.inverse_matrices();

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
            let a = pixel_to_cartesian(width, window_size.width);
            let b = pixel_to_cartesian(self.size.0, window_size.width);
            if a != 0f32 && b != 0f32 {
                a / b
            } else {
                b
            }
        } else {
            0.0
        };

        let difference_in_height = if self.size.1 != 0.0 && height != 0.0 {
            let a = pixel_to_cartesian(height, window_size.height);
            let b = pixel_to_cartesian(self.size.1, window_size.height);
            if a != 0f32 && b != 0f32 {
                a / b
            } else {
                b
            }
        } else {
            0.0
        };
        let difference_in_depth = if self.size.2 != 0.0 && depth != 0.0 {
            let a = pixel_to_cartesian(depth, window_size.width);
            let b = pixel_to_cartesian(self.size.2, window_size.width);
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
        // The reason for using different transformation matrix is because
        // of alteration of translation that happens due to rotation. The
        // solution suggested by https://github.com/tksuoran fixed this through
        // separating the matrices and multiplying them back at the end.
        let mut rotation_matrix = self.rotation_matrix;
        let axis = match axis {
            RotateAxis::X => {
                self.rotation.0 += angle;
                nalgebra_glm::Vec3::x_axis()
            }
            RotateAxis::Y => {
                self.rotation.1 += angle;
                nalgebra_glm::Vec3::y_axis()
            }
            RotateAxis::Z => {
                self.rotation.2 += angle;
                nalgebra_glm::Vec3::z_axis()
            }
        };

        rotation_matrix = nalgebra_glm::rotate(&rotation_matrix, angle.to_radians(), &axis);
        self.rotation_matrix = rotation_matrix;
        self.inverse_matrices();

        self.changed = true;
    }

    /// Moves the object by the amount you specify in the axis you specify
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position.0 -= x;
        self.position.1 -= y;
        self.position.2 -= z;

        let mut position_matrix = self.position_matrix;
        position_matrix = nalgebra_glm::translate(&position_matrix, &nalgebra_glm::vec3(x, y, z));
        self.position_matrix = position_matrix;

        self.inverse_matrices();
        self.changed = true;
    }

    /// Sets the position of the object in 3D space relative to the window
    pub fn position(&mut self, x: f32, y: f32, z: f32) {
        self.translate(
            (self.position.0 - x) * -1f32,
            (self.position.1 - y) * -1f32,
            (self.position.2 - z) * -1f32,
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

    /// Changes the main color of the object hat is sent to GPU. If textures exist, the color of textures will change
    pub fn set_uniform_color(
        &mut self,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) -> anyhow::Result<()> {
        self.uniform_color = Array4 {
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

    /// This will flag object as changed and altered, leading to rebuilding parts, or entirety on next frame.
    /// Best used if you directly altered fields of the object. The functions normally flag the object as
    /// changed on every call anyways. But this function is to manually flag it yourself.
    pub fn flag_as_changed(&mut self) {
        self.changed = true;
    }

    /// same as flag_as_changed, but inverse
    pub fn flag_as_unchanged(&mut self) {
        self.changed = false;
    }

    pub fn inverse_matrices(&mut self) {
        self.inverse_transformation_matrix =
            Matrix::from_im(nalgebra_glm::transpose(&nalgebra_glm::inverse(
                &(self.position_matrix * self.rotation_matrix * self.scale_matrix),
            )));
    }

    /// Update and apply changes done to an object
    pub fn update(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        self.update_vertex_buffer(renderer)?;
        self.update_uniform_buffer(renderer)?;
        self.update_shader(renderer)?;
        self.changed = false;
        Ok(())
    }

    pub fn update_vertex_buffer(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let updated_buffer = renderer.build_vertex_buffer(&self.vertices, &self.indices)?;
        self.pipeline.vertex_buffer = updated_buffer;

        Ok(())
    }

    pub fn update_shader(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        let updated_shader = renderer.build_shader(
            self.name.as_str(),
            self.shader_builder.shader.clone(),
            Some(&self.uniform_layout),
            self.shader_settings,
        )?;
        self.pipeline.shader = updated_shader;

        Ok(())
    }

    pub fn update_uniform_buffer(&mut self, renderer: &mut Renderer) -> anyhow::Result<()> {
        self.uniform_buffers[0] = renderer.build_uniform_buffer_part(
            "Transformation Matrix",
            uniform_type::Matrix::from_im(
                self.position_matrix * self.rotation_matrix * self.scale_matrix,
            ),
        );
        self.uniform_buffers[1] = renderer.build_uniform_buffer_part("Color", self.uniform_color);

        let updated_buffer = renderer.build_uniform_buffer(&self.uniform_buffers)?;

        self.pipeline.uniform = Some(updated_buffer.0);
        self.uniform_layout = updated_buffer.1;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ShaderBuilder {
    pub shader: String,
}

impl ShaderBuilder {
    pub fn new(camera_effect: bool) -> Self {
        Self {
            shader: format!(
                "{}{}{}",
                format!(
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
                format!(
                    // step 2 define input and output for vertex
                    "\n{}",
                    r#"struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texture_coordinates: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
};

@group(0) @binding(0)
var texture_diffuse: texture_2d<f32>;

@group(0) @binding(1)
var sampler_diffuse: sampler;"#
                ),
                format!(
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
}

// ===== Fragment STAGE ===== //
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates) * fragment_uniforms.color;
}"#
                )
            ),
        }
    }
}

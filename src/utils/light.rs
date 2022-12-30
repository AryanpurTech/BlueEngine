use crate::{ObjectStorage};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct LightUniforms {
    light_color: crate::uniform_type::Array4,     // 4 units
    light_position: crate::uniform_type::Array3,  // 3 units
    ambient_strength: f32,                        // 1 unit
    camera_position: crate::uniform_type::Array3, // 3 units
    specular_strength: f32,                       // 1 unit
    inverse_model: crate::uniform_type::Matrix,   // 4x4 units
}

impl crate::LightManager {
    pub fn new() -> Self {
        Self {
            ambient_color: crate::uniform_type::Array4 {
                data: [1f32, 1f32, 1f32, 1f32], //0.051f32, 0.533f32, 0.898f32
            },
            ambient_strength: 0f32,
            affected_objects: Vec::new(),
            light_objects: std::collections::BTreeMap::new(),
        }
    }

    pub fn update(
        &mut self,
        objects: &mut ObjectStorage,
        renderer: &mut crate::Renderer,
        camera: &crate::Camera,
    ) -> anyhow::Result<()> {
        let light_keys: Vec<String> = self.light_objects.keys().map(|x| x.clone()).collect();

        for i in objects.iter_mut() {
            let i = i.1;
            if light_keys.contains(&i.name) {
                self.light_objects.insert(
                    i.name.clone(),
                    ([i.position.0, i.position.1, i.position.2], i.color),
                );
            } else {
                let result = i.color * self.ambient_color;
                i.set_uniform_color(
                    result.data[0],
                    result.data[1],
                    result.data[2],
                    result.data[3],
                )?;

                let pos = *self.light_objects.get(&light_keys[0]).unwrap();
                let light_uniform_buffer = renderer.build_uniform_buffer_part(
                    "light_uniform_buffer",
                    LightUniforms {
                        light_color: pos.1,
                        light_position: crate::uniform_type::Array3 {
                            data: [pos.0[0] * -1f32, pos.0[1] * -1f32, pos.0[2] * -1f32],
                        },
                        ambient_strength: self.ambient_strength,
                        inverse_model: i.inverse_transformation_matrix,
                        camera_position: crate::uniform_type::Array3 {
                            data: camera.position.data.0[0],
                        },
                        specular_strength: 0.8,
                    },
                );
                if i.uniform_buffers.len() == 2 {
                    i.uniform_buffers.push(light_uniform_buffer);
                } else {
                    i.uniform_buffers[2] = light_uniform_buffer;
                }

                i.update_uniform_buffer(renderer)?;

                if !self.affected_objects.contains(&i.name) {
                    i.shader_builder.blocks = format!(
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
var<uniform> fragment_uniforms: FragmentUniforms;

struct LightUniforms {
    light_color: vec4<f32>,
    light_position: vec3<f32>,
    ambient_strength: f32,
    camera_position: vec3<f32>,
    specular_strength: f32,
    inverse_model: mat4x4<f32>,
};
@group(2) @binding(2)
var<uniform> light_uniform_buffer: LightUniforms;"#,
                        if i.camera_effect {
                            r#"
struct CameraUniforms {
    camera_matrix: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera_uniform: CameraUniforms;"#
                        } else {
                            ""
                        }
                    );
                    i.shader_builder.input_and_output = format!(
                        "\n{}",
                        /* wgsl */
                        r#"struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texture_coordinates: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) fragment_position: vec3<f32>,
    @location(3) ambient_intensity: f32,
};"#
                    );
                    i.shader_builder.vertex_stage = format!(
                        "\n// ===== VERTEX STAGE ===== //\n{}\n{}\n{}",
                        r#"@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.texture_coordinates = input.texture_coordinates;
    out.normal = (light_uniform_buffer.inverse_model * vec4<f32>(input.normal, 0.0)).xyz;
    out.fragment_position = (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0)).xyz;
    out.ambient_intensity = light_uniform_buffer.ambient_strength;"#,
                        if i.camera_effect {
                            "out.position = camera_uniform.camera_matrix * (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0));"
                        } else {
                            "out.position = transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0);"
                        },
                        r#"return out;
}"#
                    );

                    i.shader_builder.fragment_stage = format!(
                        // step 5 fragment stage
                        "\n// ===== Fragment STAGE ===== //\n{}",
                        r#"@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // ambient
    var ambient: vec4<f32> = input.ambient_intensity * light_uniform_buffer.light_color;

    // diffuse
    var norm: vec3<f32> = normalize(input.normal);
    var light_dir: vec3<f32> = normalize(light_uniform_buffer.light_position - input.fragment_position);
    var diff: f32 = max(dot(norm, light_dir), 0.0);
    var diffuse = diff * light_uniform_buffer.light_color;

    // specular
    var view_dir: vec3<f32> = normalize(light_uniform_buffer.camera_position - input.fragment_position);
    var reflect_dir: vec3<f32> = reflect(-light_dir, norm);
    var spec: f32 = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    var specular = light_uniform_buffer.specular_strength * spec * light_uniform_buffer.light_color;

    var result = (ambient + diffuse + specular) * fragment_uniforms.color;

    return textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates) * result;
}"#
                    );
                    i.pipeline.shader = renderer.build_shader(
                        i.name.as_str(),
                        i.shader_builder.build_shader(),
                        Some(&i.uniform_layout),
                        i.shader_settings,
                    )?;
                    self.affected_objects.push(i.name.clone());
                }
            }
        }

        Ok(())
    }

    pub fn set_object_as_light(&mut self, object: String) {
        self.light_objects.insert(
            object,
            (
                [0f32, 0f32, 0f32],
                crate::uniform_type::Array4 {
                    data: [0f32, 0f32, 0f32, 0f32],
                },
            ),
        );
    }
}

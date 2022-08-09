impl crate::LightManager {
    pub fn new() -> Self {
        Self {
            ambient_color: crate::uniform_type::Array4 {
                data: [1f32, 1f32, 1f32, 1f32],
            },
            affected_objects: Vec::new(),
            light_objects: std::collections::BTreeMap::new(),
        }
    }

    pub fn update(
        &mut self,
        objects: &mut Vec<crate::Object>,
        renderer: &mut crate::Renderer,
    ) -> anyhow::Result<()> {
        let light_keys: Vec<usize> = self.light_objects.keys().map(|x| *x).collect();

        for i in objects {
            if light_keys.contains(&&i.object_index) {
                self.light_objects
                    .insert(i.object_index, [i.position.0, i.position.1, i.position.2]);
            } else {
                let result = i.main_color * self.ambient_color;
                i.set_color(
                    result.data[0],
                    result.data[1],
                    result.data[2],
                    result.data[3],
                )?;
                let pos = *self.light_objects.get(&light_keys[0]).unwrap();
                let light_pos = crate::UniformBuffer::Array4(
                    "light_pos",
                    crate::uniform_type::Array4 {
                        data: [
                            pos[0] * -1f32,
                            pos[1] * -1f32,
                            pos[2] * -1f32, // For diffuse light mistaking it
                            1f32,
                        ],
                    },
                );

                if i.uniform_buffers.len() == 2 {
                    i.uniform_buffers.push(light_pos);
                } else {
                    i.uniform_buffers[2] = light_pos;
                }

                let new_uniform_buffers = renderer
                    .build_uniform_buffer(i.uniform_buffers.clone())
                    .unwrap();
                i.pipeline.uniform = Some(new_uniform_buffers.0);
                i.uniform_layout = new_uniform_buffers.1;

                if !self.affected_objects.contains(&i.object_index) {
                    i.shader_builder.input_and_output = format!(
                        "\n{}",
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
};"#
                    );
                    i.shader_builder.vertex_stage = format!(
                        "\n// ===== VERTEX STAGE ===== //\n{}\n{}\n{}",
                        r#"@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.texture_coordinates = input.texture_coordinates;
    out.normal = input.normal;
    out.fragment_position = (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0)).xyz;"#,
                        if i.camera_effect {
                            "out.position = camera_uniform.camera_matrix * (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0));"
                        } else {
                            "out.position = transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0);"
                        },
                        r#"return out;
}"#
                    );
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

struct LightPosition {
    light_pos: vec4<f32>,
};
@group(2) @binding(2)
var<uniform> light_position: LightPosition;"#,
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
                    i.shader_builder.fragment_stage = format!(
                        // step 5 fragment stage
                        "\n// ===== Fragment STAGE ===== //\n{}",
                        r#"@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    var norm: vec3<f32> = normalize(input.normal);
    var light_dir: vec3<f32> = normalize(light_position.light_pos.xyz - input.fragment_position);
    var diff: f32 = max(dot(norm, light_dir), 0.0);
    var light_color: vec4<f32>;
    light_color.x = 0.9;
    light_color.y = 0.9;
    light_color.z = 1.0;
    light_color.w = 1.0;
    var diffuse = (diff + 1.0) * light_color;

    // textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates) *
    return textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates) * fragment_uniforms.color * diffuse;
}"#
                    );
                    i.pipeline.shader = renderer.build_shader(
                        i.name.unwrap_or("Object"),
                        i.shader_builder.build_shader(),
                        Some(&i.uniform_layout),
                        i.shader_settings,
                    )?;
                    self.affected_objects.push(i.object_index);
                }
            }
        }

        Ok(())
    }

    pub fn set_object_as_light(&mut self, object: usize) {
        self.light_objects.insert(object, [0f32, 0f32, 0f32]);
    }
}

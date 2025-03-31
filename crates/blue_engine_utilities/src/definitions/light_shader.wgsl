struct TransformationUniforms {
    transform_matrix: mat4x4<f32>,
};
@group(2) @binding(0)
var<uniform> transform_uniform: TransformationUniforms;

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
var<uniform> light_uniform_buffer: LightUniforms;

// if camera is affecting, then the uniform for camera will be added
//@CAMERASTRUCT

struct VertexInput {
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
};

@group(0) @binding(0)
var texture_diffuse: texture_2d<f32>;

@group(0) @binding(1)
var sampler_diffuse: sampler;

// ===== VERTEX STAGE ===== //
@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.texture_coordinates = input.texture_coordinates;
    out.normal = (light_uniform_buffer.inverse_model * vec4<f32>(input.normal, 0.0)).xyz;
    out.fragment_position = (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0)).xyz;
    out.ambient_intensity = light_uniform_buffer.ambient_strength;
    
    // this will be used to replace the out for camera effect or not
    //@CAMERAOUT

    return out;
}

// ===== Fragment STAGE ===== //
@fragment
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
}
// blocks

//@CAMERA_STRUCT

struct TransformationUniforms {
    transform_matrix: mat4x4<f32>,
};
@group(2) @binding(0)
var<uniform> transform_uniform: TransformationUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) texture_coordinates: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_coordinates: vec2<f32>,
};

struct InstanceInput {
    @location(3) model_matrix_0: vec4<f32>,
    @location(4) model_matrix_1: vec4<f32>,
    @location(5) model_matrix_2: vec4<f32>,
    @location(6) model_matrix_3: vec4<f32>,
};

// Vertex Stage
@vertex
fn vs_main(input: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.texture_coordinates = input.texture_coordinates;
    //@CAMERA_VERTEX
    return out;
}

// Fragment Stage
struct FragmentUniforms {
    color: vec4<f32>,
};
@group(2) @binding(1)
var<uniform> fragment_uniforms: FragmentUniforms;

@group(0) @binding(0)
var texture_diffuse: texture_2d<f32>;

@group(0) @binding(1)
var sampler_diffuse: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    if fragment_uniforms.color.w == 0.0 {
        return textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates);
    } else {
        return textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates) * fragment_uniforms.color;
    }
}
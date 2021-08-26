// Vertex Stage

[[block]]
struct VertexUniforms {
    camera_matrix: mat4x4<f32>;
};
[[group(1), binding(0)]]
var<uniform> vertex_uniforms: VertexUniforms;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] texture_coordinates: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] texture_coordinates: vec2<f32>;
};

[[stage(vertex)]]
fn main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vertex_uniforms.camera_matrix * vec4<f32>(input.position, 1.0);
    out.texture_coordinates = input.texture_coordinates;
    return out;
}

// Fragment Stage

[[block]]
struct FragmentUniforms {
    color: vec4<f32>;
};
[[group(1), binding(1)]]
var<uniform> fragment_uniforms: FragmentUniforms;

[[group(0), binding(0)]]
var texture_diffuse: texture_2d<f32>;

[[group(0), binding(1)]]
var sampler_diffuse: sampler;

[[stage(fragment)]]
fn main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates) * fragment_uniforms.color;
}
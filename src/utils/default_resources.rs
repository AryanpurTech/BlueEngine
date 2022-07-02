/*
 * Blue Engine by Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

pub const DEFAULT_SHADER: &str = r#"
// Vertex Stage


struct CameraUniforms {
    camera_matrix: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera_uniform: CameraUniforms;


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

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = camera_uniform.camera_matrix * (transform_uniform.transform_matrix * vec4<f32>(input.position, 1.0));
    out.texture_coordinates = input.texture_coordinates;
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
    return textureSample(texture_diffuse, sampler_diffuse, input.texture_coordinates) * fragment_uniforms.color;
}
"#;

pub const DEFAULT_TEXTURE: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0,
    0, 0, 31, 21, 196, 137, 0, 0, 1, 130, 105, 67, 67, 80, 73, 67, 67, 32, 112, 114, 111, 102, 105,
    108, 101, 0, 0, 40, 207, 149, 145, 75, 40, 68, 81, 28, 198, 127, 238, 16, 121, 46, 76, 145,
    164, 187, 192, 138, 18, 146, 165, 134, 72, 81, 26, 163, 188, 22, 238, 189, 99, 134, 154, 123,
    77, 247, 142, 108, 44, 149, 237, 148, 133, 199, 198, 107, 97, 99, 205, 214, 194, 86, 41, 229,
    81, 178, 179, 179, 34, 54, 210, 245, 63, 119, 212, 76, 106, 148, 83, 167, 243, 235, 59, 231,
    251, 58, 231, 59, 160, 29, 164, 44, 219, 43, 237, 2, 219, 201, 184, 209, 145, 136, 62, 61, 51,
    171, 151, 63, 163, 209, 64, 21, 45, 212, 26, 150, 151, 30, 159, 28, 142, 81, 116, 124, 220, 82,
    162, 214, 155, 78, 149, 197, 255, 70, 77, 124, 209, 179, 160, 68, 23, 30, 176, 210, 110, 70,
    120, 65, 184, 111, 45, 147, 86, 188, 35, 28, 182, 150, 140, 184, 240, 169, 112, 135, 43, 23,
    20, 190, 87, 186, 153, 227, 23, 197, 201, 128, 53, 149, 25, 118, 99, 209, 65, 225, 176, 176,
    158, 44, 96, 179, 128, 173, 37, 215, 22, 238, 21, 110, 141, 219, 142, 228, 107, 211, 57, 142,
    43, 94, 87, 108, 167, 86, 173, 159, 123, 170, 23, 86, 47, 58, 83, 147, 74, 151, 217, 204, 8,
    163, 140, 51, 129, 142, 201, 42, 203, 164, 200, 208, 41, 171, 35, 138, 71, 84, 246, 35, 69,
    252, 77, 129, 127, 66, 92, 166, 184, 150, 177, 196, 49, 196, 10, 54, 70, 224, 71, 253, 193,
    239, 110, 189, 68, 79, 119, 46, 169, 58, 2, 101, 79, 190, 255, 214, 6, 229, 91, 240, 149, 245,
    253, 207, 67, 223, 255, 58, 130, 208, 35, 92, 56, 121, 255, 202, 1, 244, 191, 139, 158, 205,
    107, 173, 251, 80, 183, 1, 103, 151, 121, 205, 220, 134, 243, 77, 104, 124, 72, 27, 174, 17,
    72, 33, 153, 90, 34, 1, 175, 39, 242, 77, 51, 80, 127, 13, 149, 115, 185, 222, 126, 246, 57,
    190, 131, 152, 116, 53, 118, 5, 187, 123, 208, 158, 148, 236, 249, 34, 239, 174, 40, 236, 237,
    207, 51, 65, 127, 68, 190, 1, 84, 201, 114, 155, 16, 187, 186, 109, 0, 0, 0, 9, 112, 72, 89,
    115, 0, 0, 46, 34, 0, 0, 46, 34, 1, 170, 226, 221, 146, 0, 0, 0, 13, 73, 68, 65, 84, 24, 87,
    99, 248, 255, 255, 255, 127, 0, 9, 251, 3, 253, 5, 67, 69, 202, 0, 0, 0, 0, 73, 69, 78, 68,
    174, 66, 96, 130,
];

pub const DEFAULT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub const DEFAULT_MATRIX_4: crate::header::uniform_type::Matrix =
    crate::header::uniform_type::Matrix {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

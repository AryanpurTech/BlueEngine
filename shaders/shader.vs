// vertex shader
#version 450

layout(location=0) in vec3 in_position;
layout(location=1) in vec3 in_color;

layout(location=0) out vec3 v_color;

void main () {
    v_color = in_color;
    gl_Position = vec4(in_position, 1.0);
}
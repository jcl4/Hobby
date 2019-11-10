#version 450

layout(location = 0) in vec4 position;
layout(location = 1) in vec4 color;

layout(location = 0) out vec4 frag_color;

// layout(set = 0, binding = 0) uniform locals {
//     mat4 transform;
// };

void main() {
    gl_Position  = position;
    // gl_Position  = transform * position;
    frag_color = color;
}
#version 450

// NOTE: names must match the `Vertex` struct in Rust
layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

layout(location = 0) out vec4 fragColor;

layout(set = 0, binding = 0) uniform Transform {
    mat4 model;
} transform;

void main() {
    gl_Position = transform.model * vec4(position, 1.0);
    fragColor = color;
}

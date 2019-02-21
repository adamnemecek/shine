#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 pos;
layout(location = 1) in vec4 color;

layout(set = 0, binding = 0) uniform Args {
    mat4 proj;
    mat4 view;
};

layout(location = 0) out vec4 frag_color;

void main() {
    frag_color = color;
    gl_Position = proj * view * vec4(pos, 1.0);
}
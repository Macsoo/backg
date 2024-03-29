#version 460

layout (location = 0) in vec3 vPos;
layout (location = 1) in vec3 nPos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 normal;

void main() {
    gl_Position = projection * view * model * vec4(vPos, 1.0);
    normal = nPos;
}
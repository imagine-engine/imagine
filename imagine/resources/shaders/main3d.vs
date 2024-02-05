#version 450

layout(location=0) in vec3 position;
layout(location=1) in vec3 normal;
layout(location=2) in vec2 uv;

layout(set=1, binding=0) uniform Camera {
    mat4 view;
    mat4 projection;
};

layout(set=2, binding=0) uniform Model {
    mat4 transform;
    mat4 normal_matrix;
};

layout(location=0) out vec2 out_uv;
layout(location=1) out vec3 view_position;
// layout(location=2) out vec3 world_normal;

void main() {
    vec4 model_view = view * transform * vec4(position, 1.0);

    out_uv = uv;
    view_position = model_view.xyz;
    gl_Position = projection * model_view;
    // world_normal = normalize(normal_matrix * normal);

    // gl_Position = view * projection * transform * vec4(position, 1.0);
}
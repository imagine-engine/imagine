#version 450

layout(location=0) in vec2 uv;
// layout(location=1) in vec3 view_normal;
layout(location=1) in vec3 position;

layout(location=0) out vec4 f_color;

// layout(set=0, binding=0) uniform float opacity;

layout(set=0, binding=0) uniform texture2D t_normal;
layout(set=0, binding=1) uniform sampler s_normal;

layout(set=0, binding=2) uniform texture2D t_diffuse;
layout(set=0, binding=3) uniform sampler s_diffuse;

layout(set=0, binding=4) uniform texture2D t_specular;
layout(set=0, binding=5) uniform sampler s_specular;

// struct Light {
//     mat4 transform;
//     mat4 normal_matrix;
// };

// layout(set=1, binding=0) readonly buffer _Lights {
//     Light lights[];
// };

void main() {
    // vec3 normal_color = texture(sampler2D(t_normal, s_normal), uv).xyz;
    // vec4 diffuse_color = texture(sampler2D(t_diffuse, s_diffuse), uv);
    // vec3 specular_color = texture(sampler2D(t_specular, s_specular), uv).rgb;

    f_color = vec4(0.5, 0, 0.5, 1.0);
    // f_color = diffuse_color;
}
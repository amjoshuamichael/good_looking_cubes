layout(push_constant) uniform PushConstants {
    vec4 camera_pos;
    vec4 camera_dir;

    uint world[512];
} pc;

layout(location = 0) in vec4 vertex_color;

layout(location = 0) out vec4 fragment_color;

layout(set = 0, binding = 0, rgba8_snorm) uniform readonly image3D world_texture;
layout(set = 0, binding = 1) uniform sampler world_sampler;
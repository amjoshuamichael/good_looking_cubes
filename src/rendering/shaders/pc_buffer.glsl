layout(push_constant) uniform PushConstants {
    vec4 camera_pos;
    vec4 camera_dir;

    uint world[size * size * size];
} pc;

layout(location = 0) in vec4 vertex_color;
layout(location = 0) out vec4 fragment_color;

layout(set = 0, binding = 0, std140) uniform WorldBuffer {
    int data[size * size * size];
} world;


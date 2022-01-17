layout(push_constant) uniform PushConstants {
    vec4 camera_pos;
    vec4 camera_dir;
} pc;

layout(location = 0) in vec4 vertex_color;
layout(location = 0) out vec4 fragment_color;

layout(std430, set = 0, binding = 0) buffer WorldBuffer {
    int data[VOXEL_COUNT];
} world;


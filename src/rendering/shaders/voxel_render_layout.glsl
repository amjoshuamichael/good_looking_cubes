layout(location = 0) in vec4 vertex_color;
layout(location = 0) out vec4 fragment_color;

layout(set = 0, binding = 0, rgba8_snorm) uniform writeonly image2D depth_image;
layout(set = 0, binding = 1) uniform sampler depth_sampler;

layout(std430, set = 1, binding = 0) buffer WorldBuffer {
    int data[VOXEL_COUNT];
} world;

const float total_samples = 100;
const float highlight_samples = 75;
const float highlight_sensitivity = 0.99;
const uint bounce_dist = 20;

const uint AIR = 0;
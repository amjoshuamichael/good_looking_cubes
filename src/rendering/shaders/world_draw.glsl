layout(set = 0, binding = 0, rgba8_snorm) uniform coherent image3D world_texture;
layout(set = 0, binding = 1) uniform sampler world_sampler;

void main() {
    float test_r_color = 0.9;

    imageStore(world_texture, ivec3(0), vec4(1.0, 1.0, 1.0, 1.0));

    if (imageLoad(world_texture, ivec3(0)).r == 0.0) {
        for (int x = 0; x < 8; x++) {
            for (int y = 0; y < 8; y++) {
                for (int z = 0; z < 8; z++) {
                    imageStore(world_texture, ivec3(x, y, z), vec4(test_r_color, 1.0, float(z) / 8.0, 1.0));
                }
            }
        }
    }

    return;
}
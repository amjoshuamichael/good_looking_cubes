vec4 color_from(uint voxel) {
    return pc.palette[voxel >> 24];
}

float emission_from(uint voxel) {
    uint bits = bitfieldExtract(voxel, 20, 4);
    return float(bits) / 4.0;
}

float metallic_from(uint voxel) {
    uint bits = bitfieldExtract(voxel, 18, 2);
    return float(bits) / 4.0;
}

bool is_air(uint voxel) {
    return bitfieldExtract(voxel, 0, 1) == 0;
}
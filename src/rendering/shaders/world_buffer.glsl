layout(std430, set = 0, binding = 0) buffer WorldBuffer {
    int data[VOXEL_COUNT];
    int filled_chunks[CHUNK_COUNT];
} world;
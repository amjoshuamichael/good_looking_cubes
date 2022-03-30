#version 450
#pragma optionNV (unroll all)

const uint CHUNKS_X = 16;
const uint CHUNKS_Y = 16;
const uint CHUNKS_Z = 16;
const uint CHUNK_COUNT = 16 * 16 * 16;
const uint CHUNK_SIZE = 16;
const uint CHUNK_VOL = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
const uint WORLD_SIZE_X = CHUNKS_X * CHUNK_SIZE;
const uint WORLD_SIZE_Y = CHUNKS_Y * CHUNK_SIZE;
const uint WORLD_SIZE_Z = CHUNKS_Z * CHUNK_SIZE;
const uint VOXEL_COUNT = CHUNK_VOL * CHUNKS_X * CHUNKS_Y * CHUNKS_Z;


uint unit_at(vec3 _pos) {
    if (_pos.x < 0.0 || _pos.x > WORLD_SIZE_X - 1
     || _pos.y < 0.0 || _pos.y > WORLD_SIZE_Y - 1
     || _pos.z < 0.0 || _pos.z > WORLD_SIZE_Z - 1) {
        return 0;
    }

    uvec3 pos = uvec3(_pos);
    uvec3 chunkPos = pos / CHUNK_SIZE;
    uvec3 posInChunk = pos % CHUNK_SIZE;

    uint index = uint(
        (chunkPos.x + chunkPos.y * CHUNKS_X + chunkPos.z * CHUNKS_X * CHUNKS_Y) * CHUNK_VOL +
        posInChunk.x + posInChunk.y * CHUNK_SIZE + posInChunk.z * CHUNK_SIZE * CHUNK_SIZE
    );
    return uint(world.data[index]);
}


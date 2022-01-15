uint unit_at(vec3 pos) {
    if (pos.x < 0.0 || pos.x > size || pos.y < 0.0 || pos.y > size) {
        return 0;
    }

    uint index = uint(pos.z * size * size + pos.y * size + pos.x);
    return uint(world.data[index]);
}


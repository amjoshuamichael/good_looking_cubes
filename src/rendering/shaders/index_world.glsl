uint unit_at(vec3 pos) {
//    if (pos.x < 0.0 || pos.x > size - 1.0 || pos.y < 0.0 || pos.y > size - 1.0 || pos.z < 0.0 || pos.z > size - 1.0) {
//        return 0;
//    }

    uint index = uint(pos.z * size * size + pos.y * size + pos.x);
    return uint(pc.world[index]);
}


uint unit_at(vec3 pos) {
    if (pos.x < 0.0 || pos.x > 7.9 || pos.y < 0.0 || pos.y > 7.9 || pos.z < 0.0 || pos.z > 7.9) {
        return 0;
    }

    return pc.world[uint(pos.z * 64.0 + pos.y * 8.0 + pos.x)];
}


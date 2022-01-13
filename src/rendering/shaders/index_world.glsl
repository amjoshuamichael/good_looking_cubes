uint unit_at(vec3 pos) {
//    return pc.world[uint(pos.z * 64.0 + pos.y * 8.0 + pos.x)];

    if (imageLoad(world_texture, ivec3(pos)).r == 0.0) {
            return 0;
    } else {
            return 8388736;
    }
}


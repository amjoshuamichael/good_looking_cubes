vec3 intersection_with_plane_at_x(vec3 ro, vec3 rd, float x) {
    float adj = ((x - ro.x) / rd.x);
    return vec3 (x, ro.yz + rd.yz * adj);
}

vec3 intersection_with_plane_at_y(vec3 ro, vec3 rd, float y) {
    float adj = (y - ro.y) / rd.y;
    return vec3 (ro.x + rd.x * adj, y, ro.z + rd.z * adj);
}

vec3 intersection_with_plane_at_z(vec3 ro, vec3 rd, float z) {
    float adj = (z - ro.z) / rd.z;
    return vec3 (ro.xy + rd.xy * adj, z);
}


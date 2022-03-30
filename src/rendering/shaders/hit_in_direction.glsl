layout(location = 0) in vec4 vertex_color;
layout(location = 0) out vec4 fragment_color;

struct hit {
    vec3 pos;
    vec3 normal;
    uint dist; // from origin of ray

    uint unit_code;
};

hit voxel_hit_in_direction(vec3 ro, vec3 rd, uint dist, uint mask_color) {
    vec3 check_point = floor(ro);

    float xy = rd.x / rd.y;
    float yz = rd.y / rd.z;
    float zx = rd.z / rd.x;
    float xz = rd.x / rd.z;
    float yx = rd.y / rd.x;
    float zy = rd.z / rd.y;

    vec3 ray_unit_step_size = vec3(
    sqrt(1 + zx * zx + yx * yx),
    sqrt(1 + xy * xy + zy * zy),
    sqrt(1 + xz * xz + yz * yz)
    );
    vec3 step = sign(rd);
    vec3 ray_length = (step * (check_point - ro) + (step / 2 + 0.5)) * ray_unit_step_size;

    vec3 comp;
    uint unit_at_check_point;
    for (int i = 0; i < dist; i++) {
        comp = vec3(bvec3(
        ray_length.x < ray_length.y && ray_length.x <= ray_length.z,
        ray_length.y < ray_length.z && ray_length.y <= ray_length.x,
        ray_length.z < ray_length.x && ray_length.z <= ray_length.y
        ));

        check_point += comp * step;

        unit_at_check_point = voxel_unit_at(check_point);
        if(unit_at_check_point != mask_color) {
            return hit(ro + rd * size_of_min_dimension(ray_length), - comp * step, i, unit_at_check_point);
        }

        ray_length += comp * ray_unit_step_size;
    };

    return hit(vec3(0.0), vec3(0.0), 0, 0);
}

vec3 filled_chunk_in_direction(vec3 ro, vec3 rd, uint dist) {
    vec3 check_point = floor(ro);

    if (is_chunk_filled_at(check_point)) {
        return ro;
    }

    float xy = rd.x / rd.y;
    float yz = rd.y / rd.z;
    float zx = rd.z / rd.x;
    float xz = rd.x / rd.z;
    float yx = rd.y / rd.x;
    float zy = rd.z / rd.y;

    vec3 ray_unit_step_size = vec3(
        sqrt(1 + zx * zx + yx * yx),
        sqrt(1 + xy * xy + zy * zy),
        sqrt(1 + xz * xz + yz * yz)
    );
    vec3 step = sign(rd);
    vec3 ray_length = (step * (check_point - ro) + (step / 2 + 0.5)) * ray_unit_step_size;

    vec3 comp;
    uint unit_at_check_point;

    for (int i = 0; i < dist; i++) {
        comp = vec3(bvec3(
            ray_length.x < ray_length.y && ray_length.x <= ray_length.z,
            ray_length.y < ray_length.z && ray_length.y <= ray_length.x,
            ray_length.z < ray_length.x && ray_length.z <= ray_length.y
        ));

        check_point += comp * step;

        if(is_chunk_filled_at(check_point)) {
            return ro;
        }

        ray_length += comp * ray_unit_step_size;
    };

    return ro;
}

hit hit_in_direction(vec3 ro, vec3 rd, uint dist, uint mask_color) {
    vec3 chunk_starting_point = filled_chunk_in_direction(ro / 16, rd, dist) * 16;

    fragment_color += length(ro - chunk_starting_point) / 3.0;

    return voxel_hit_in_direction(ro, rd, dist, mask_color);
}


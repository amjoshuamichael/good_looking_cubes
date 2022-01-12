struct hit {
    vec3 pos;
    vec3 normal;

    uint unit_code;
};

vec4 color_from(uint color) {
    uint r = color >> 24;
    uint g = color <<  8 >> 24;
    uint b = color << 16 >> 24;
    uint a = color << 24 >> 24;

    return vec4(float(r), float(g), float(b), float(a));
}

hit hit_in_direction(vec3 ro, vec3 rd) {
    vec3 check_point = floor(ro);

    vec3 ray_unit_step_size = vec3(
        sqrt(1 + (rd.z / rd.x) * (rd.z / rd.x) + (rd.y / rd.x) * (rd.y / rd.x)),
        sqrt(1 + (rd.x / rd.y) * (rd.x / rd.y) + (rd.z / rd.y) * (rd.z / rd.y)),
        sqrt(1 + (rd.x / rd.z) * (rd.x / rd.z) + (rd.y / rd.z) * (rd.y / rd.z))
    );
    vec3 step = vec3(sign(rd));
    vec3 ray_length;

    if (rd.x < 0) {
        ray_length.x = (ro.x - check_point.x) * ray_unit_step_size.x;
    } else {
        ray_length.x = (check_point.x + 1 - ro.x) * ray_unit_step_size.x;
    }

    if (rd.y < 0) {
        ray_length.y = (ro.y - check_point.y) * ray_unit_step_size.y;
    } else {
        ray_length.y = (check_point.y + 1 - ro.y) * ray_unit_step_size.y;
    }

    if (rd.z < 0) {
        ray_length.z = (ro.z - check_point.z) * ray_unit_step_size.z;
    } else {
        ray_length.z = (check_point.z + 1 - ro.z) * ray_unit_step_size.z;
    }

    float max_dist = 100.0f;
    float current_dist = 0.0f;

    vec3 comp;

    uint unit_at_check_point;
    do {
        comp = vec3(bvec3(
            ray_length.x < ray_length.y && ray_length.x <= ray_length.z,
            ray_length.y < ray_length.z && ray_length.y <= ray_length.x,
            ray_length.z < ray_length.x && ray_length.z <= ray_length.y
        ));

        if (comp == vec3(0.0)) {
            comp.z = 1.0;
        }

        check_point += comp * step;
        vec3 ray_length_min_dimension = comp * ray_length;
        const vec3 AllOnes = vec3(1.0);
        // dot(ray_length_min_dimension, AllOnes) is mathematically equivalent to
        // ray_length_min_dimension.x + ray_length_min_dimension.y + ray_length_min_dimension.z,
        // but the former runs faster.
        current_dist = dot(ray_length_min_dimension, AllOnes);
        ray_length += comp * ray_unit_step_size;

        unit_at_check_point = unit_at(check_point);
    } while (current_dist < max_dist && unit_at_check_point == 0);

    if (unit_at_check_point == 0) {
        return hit(vec3(0.0), vec3(0.0), 0);
    } else {
        return hit(ro + rd * current_dist, - comp * step, unit_at_check_point);
    }
}

void main() {
    uint num_samples = 30;
    int rand_seed = setup_rand(vertex_color.xy);

    float fov = 1.0;

    vec3 rd =
    normalize(
        vec3(
            cos(pc.camera_dir.x) * vertex_color.x - sin(pc.camera_dir.x) * fov,
             - vertex_color.y,
            sin(pc.camera_dir.x) * vertex_color.x + cos(pc.camera_dir.x) * fov
        )
    );
    vec3 ro = vec3(pc.camera_pos.x, pc.camera_pos.y, pc.camera_pos.z);

    hit initial_hit = hit_in_direction(ro, rd);
    vec4 albedo_color = flatten_color(color_from(initial_hit.unit_code));
    if (albedo_color == vec4(0.0)) {
        fragment_color = vec4(1.0);
        return;
    }

    vec4 shadow = vec4(0.0);

    vec3 ldir = vec3(1.0);

    int rand_x = rand(rand_seed);
    int rand_y = rand(rand_seed);
    int rand_z = rand(rand_seed);

    for (uint i = 0; i < num_samples; i = i + 1) {
        vec3 to_light = normalize( initial_hit.normal + normalize(vec3(rand_x, rand_y, rand_z)) * 0.5);
        hit to_light_hit = hit_in_direction(initial_hit.pos, to_light);

        if (to_light_hit.unit_code == 0) {
            shadow += albedo_color;
        }

        rand_x = rand(rand_z);
        rand_y = rand(rand_y);
        rand_z = rand(rand_x);
    }

    shadow = shadow / float(num_samples);

    fragment_color = shadow;
}


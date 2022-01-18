struct hit {
    vec3 pos;
    vec3 normal;
    uint dist; // from origin of ray

    uint unit_code;
};

vec4 color_from(uint color) {
    return pc.palette[color >> 24];
}

hit hit_in_direction(vec3 ro, vec3 rd, uint dist) {
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

    vec3 comp;

    uint unit_at_check_point;

    for (int i = 0; i < dist; i++){
        comp = vec3(bvec3(
            ray_length.x < ray_length.y && ray_length.x <= ray_length.z,
            ray_length.y < ray_length.z && ray_length.y <= ray_length.x,
            ray_length.z < ray_length.x && ray_length.z <= ray_length.y
        ));

        check_point += comp * step;

        unit_at_check_point = unit_at(check_point);
        if(unit_at_check_point != 0) {
            return hit(ro + rd * size_of_min_dimension(ray_length), - comp * step, i, unit_at_check_point);
        }

        ray_length += comp * ray_unit_step_size;
    };

    return hit(vec3(0.0), vec3(0.0), 0, 0);
}

void main() {
    float fov = 1.0;

    vec3 rd = normalize(
        vec3(
            cos(pc.camera_dir.x) * vertex_color.x - sin(pc.camera_dir.x) * fov,
            - vertex_color.y,
            sin(pc.camera_dir.x) * vertex_color.x + cos(pc.camera_dir.x) * fov
        )
    );
    vec3 ro = vec3(pc.camera_pos.x, pc.camera_pos.y, pc.camera_pos.z);

    int rand_seed = setup_rand(rd.xz + rd.yy + ro.xz + ro.yy);

    hit initial_hit = hit_in_direction(ro, rd, 400);
    vec4 albedo_color = color_from(initial_hit.unit_code);
    if (albedo_color.xyz == vec3(0.0, 0.0, 0.0)) {
        fragment_color = mix(vec4(1.0), vec4(vertex_color.xy, 1.0, 1.0), 0.5);
        return;
    }

    initial_hit.pos += initial_hit.normal * 0.001;

    vec3 hit_normal_mask = vec3(equal(initial_hit.normal, vec3(0.0))) * sum_of_dimensions(initial_hit.normal);
    vec4 shadow = vec4(0.0);

    int rand_x = rand(rand_seed);
    int rand_y = rand(rand_seed);
    int rand_z = rand(rand_seed);

    // randomize the initial vector a little more
    for (float n = 0.0; n < 4.0; n++) {
        rand_x = rand(rand_z);
        rand_y = rand(rand_y);
        rand_z = rand(rand_x);
    }

    const float num_samples = 100;
    const float highlight_samples = 50;
    const float highlight_sensitivity = 0.99;
    const uint bounce_dist = 20;
    float light_amount = 1.0;

    for (float i = 0; i < num_samples; i++) {
        vec3 to_light = normalize(initial_hit.normal + normalize(vec3(rand_x, rand_y, rand_z)) * hit_normal_mask * 4.0);
        hit to_light_hit = hit_in_direction(initial_hit.pos, to_light, bounce_dist);

        if (to_light_hit.unit_code == 0) {
            shadow += albedo_color;
            light_amount++;
        } else {
            shadow += albedo_color * (float(to_light_hit.dist) / float(bounce_dist));
        }

        if (light_amount / i > highlight_sensitivity && i > highlight_samples) {
            fragment_color = albedo_color;
            return;
        }

        rand_x = rand(rand_z);
        rand_y = rand(rand_y);
        rand_z = rand(rand_x);
    }

    fragment_color = shadow / num_samples;
//
//    uint index =
//    uint(
//        floor((vertex_color.y + 1) / 2 * 16) * 16 +
//        floor((vertex_color.x + 1) / 2 * 16)
//    );
//    fragment_color = pc.palette[index];
}


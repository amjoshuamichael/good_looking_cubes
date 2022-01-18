const float total_samples = 100;
const float highlight_samples = 75;
const float highlight_sensitivity = 0.99;
const uint bounce_dist = 20;

struct hit {
    vec3 pos;
    vec3 normal;
    uint dist; // from origin of ray

    uint unit_code;
};

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

vec4 sample_at_hit(hit the_hit, float num_samples) {
    if (is_air(the_hit.unit_code)) {
        return mix(vec4(1.0), vec4(vertex_color.xy, 1.0, 1.0), 0.5);
    }

    // 977 is a number that seemed to look good after i tried a bunch of prime numbers
    int rand_x = rand(int(the_hit.pos.x * the_hit.pos.y * the_hit.pos.z * 977) % 256 + 1);
    int rand_y = rand(rand_x);
    int rand_z = rand(rand_y);

    vec4 albedo_color = color_from(the_hit.unit_code);
    float emission_amount = 1 + emission_from(the_hit.unit_code) * 0.5;
    vec3 hit_normal_mask = vec3(equal(the_hit.normal, vec3(0.0))) * sum_of_dimensions(the_hit.normal);
    vec3 start_pos = the_hit.pos + the_hit.normal * 0.001;
    float light_amount = 0.0;

    vec4 color_out = vec4(0.0);

    for (float i = 0; i < num_samples; i++) {
        vec3 to_light = normalize(the_hit.normal + normalize(vec3(rand_x, rand_y, rand_z)) * hit_normal_mask * 4.0);
        hit to_light_hit = hit_in_direction(the_hit.pos, to_light, bounce_dist);
        float hit_dist = length(the_hit.pos - to_light_hit.pos) / float(bounce_dist);

        if (to_light_hit.unit_code == 0) {
            color_out += albedo_color * emission_amount;
            light_amount++;
        } else if (emission_from(to_light_hit.unit_code) > 0) {
            color_out += emission_from(to_light_hit.unit_code) * color_from(to_light_hit.unit_code) * 1.5 * (1 - hit_dist);
        } else {
            color_out += albedo_color * hit_dist;
        }

        if (light_amount / i > highlight_sensitivity && i > highlight_samples) {
            return albedo_color * emission_amount;
        }

        rand_x = rand(rand_z);
        rand_y = rand(rand_x);
        rand_z = rand(rand_y);
    }

    return color_out / num_samples;
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

    hit initial_hit = hit_in_direction(ro, rd, 400);

    float gloss = metallic_from(initial_hit.unit_code);

    if (gloss > 0) {
        vec4 base_color = sample_at_hit(initial_hit, 100 * (1 - gloss));

        vec3 reflected_dir = reflect(rd, initial_hit.normal);
        hit reflected_hit = hit_in_direction(initial_hit.pos, reflected_dir, 400);
        vec4 reflected_color = sample_at_hit(reflected_hit, 100 * gloss);

        fragment_color = mix(base_color, reflected_color, gloss);
    } else {
        fragment_color = sample_at_hit(initial_hit, 100 * (1 - gloss));
    }
}
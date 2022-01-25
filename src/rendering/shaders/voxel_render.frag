layout(location = 0) in vec4 vertex_color;
layout(location = 0) out vec4 fragment_color;

const float total_samples = 100;
const float highlight_samples = 75;
const float highlight_sensitivity = 0.99;
const uint bounce_dist = 20;

const uint AIR = 0;

struct hit {
    vec3 pos;
    vec3 normal;
    uint dist; // from origin of ray

    uint unit_code;
};

hit hit_in_direction(vec3 ro, vec3 rd, uint dist, uint mask_color) {
    vec3 check_point = floor(ro);

    float xy = rd.x / rd.y;
    float yz = rd.y / rd.z;
    float zx = rd.z / rd.x;
    float xz = rd.x / rd.z;
    float yx = rd.y / rd.x;
    float zy = rd.z / rd.y;

    vec3 ray_unit_step_size = vec3(
        sqrt(1 + zx * zx + yx * yx),
        sqrt(1 + (rd.x / rd.y) * (rd.x / rd.y) + (rd.z / rd.y) * (rd.z / rd.y)),
        sqrt(1 + (rd.x / rd.z) * (rd.x / rd.z) + (rd.y / rd.z) * (rd.y / rd.z))
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

        unit_at_check_point = unit_at(check_point);
        if(unit_at_check_point != mask_color) {
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
        hit to_light_hit = hit_in_direction(the_hit.pos, to_light, bounce_dist, AIR);
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

    hit init_hit = hit_in_direction(ro, rd, 400, AIR);

    float gloss = metallic_from(init_hit.unit_code);
    float translucent = translucent_from(init_hit.unit_code);
    vec4 output_color;

    if (gloss > 0) {
        vec4 base_color = sample_at_hit(init_hit, total_samples * (1 - gloss));

        vec3 reflected_dir = reflect(rd, init_hit.normal);
        hit reflected_hit = hit_in_direction(init_hit.pos, reflected_dir, 400, AIR);
        vec4 reflected_color = sample_at_hit(reflected_hit, total_samples * gloss);

        output_color = mix(base_color, reflected_color, gloss);
    } else if (translucent > 0) {
        vec4 base_color = sample_at_hit(init_hit, total_samples * (1 - translucent));

        hit ray_solid_through = hit_in_direction(init_hit.pos - init_hit.normal * 0.001, rd, 400, init_hit.unit_code);
        hit ray_pass_through = hit_in_direction(ray_solid_through.pos + init_hit.normal * 0.001, rd, 400, 0);
        vec4 color_through_translucense = sample_at_hit(ray_pass_through, total_samples * translucent);

        output_color = mix(base_color, color_through_translucense, translucent);
    } else {
        output_color = sample_at_hit(init_hit, total_samples);
    }

    fragment_color = output_color * pc.exposure;
}

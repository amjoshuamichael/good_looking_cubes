const float total_samples = 200;
const float highlight_samples = 75;
const float highlight_sensitivity = 0.99;
const uint bounce_dist = 20;

const uint AIR = 0;

vec4 sample_at_hit(hit the_hit, float num_samples) {
    if (is_air(the_hit.unit_code)) {
        return mix(vec4(1.0), vec4(vertex_color.xy, 1.0, 1.0), 0.5);
    }

    // 977 is a number that seemed to look good after i tried a bunch of prime numbers
    vec3 rand_vec = hash33(vertex_color.xyz + the_hit.pos);

    vec4 albedo_color = color_from(the_hit.unit_code);
    float emission_amount = 1 + emission_from(the_hit.unit_code) * 0.5;
    vec3 hit_normal_mask = vec3(equal(the_hit.normal, vec3(0.0))) * sum_of_dimensions(the_hit.normal);
    vec3 start_pos = the_hit.pos + the_hit.normal * 0.001;
    float light_amount = 0.0;

    vec4 color_out = vec4(0.0);

    for (float i = 0; i < num_samples; i++) {
        rand_vec -= vec3(0.5);
        vec3 to_light = normalize(the_hit.normal + normalize(rand_vec) * hit_normal_mask * 4.0);
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

        rand_vec = hash33(rand_vec);
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

    fragment_color += output_color * pc.exposure;
}

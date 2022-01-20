const uint FONT_IMAGE_WIDTH = 280;
const uint FONT_IMAGE_HEIGHT = 7;
const uint FONT_IMAGE_AREA = FONT_IMAGE_WIDTH * FONT_IMAGE_HEIGHT;
const uint CHAR_AMOUNT = 32;
const uint CHAR_WIDTH = 7;

layout(location = 0) out vec4 fragment_color;
layout(location = 0) in vec4 vertex_color;

layout(set = 0, binding = 0) uniform texture2D temp_texture;
layout(set = 0, binding = 1) uniform sampler temp_sampler;

layout(std430, set = 1, binding = 0) buffer FontBuffer {
    int data[FONT_IMAGE_AREA];
} font;

layout(set = 2, binding = 0, rgba8_snorm) uniform readonly image2D depth_image;
layout(set = 2, binding = 1) uniform sampler depth_sampler;

bool is_letter_here() {
    uint pos_index = uint(
        floor((vertex_color.y / 2 + 0.5) * CHAR_AMOUNT) * CHAR_AMOUNT +
        floor((vertex_color.x / 2 + 0.5) * CHAR_AMOUNT)
    );

    uint letter_index = pc.text_to_show[pos_index];

    uint char_x = uint(letter_index * CHAR_WIDTH + fract(vertex_color.x / 2 * float(CHAR_AMOUNT)) * 7);
    uint char_y = uint(fract(vertex_color.y / 2 * float(CHAR_AMOUNT)) * 7);

    return font.data[char_y * FONT_IMAGE_WIDTH + char_x] != 0;
}

vec4 sample_temp_at(vec2 offset) {
    return texture(sampler2D(temp_texture, temp_sampler), vertex_color.xy / 2.0 + vec2(0.5) + offset / 400);
}

vec4 sample_depth_at(ivec2 offset) {
    vec2 vertex_norm = vertex_color.xy / 2 + 0.5;
    return imageLoad(depth_image, ivec2(vertex_norm * 400) + offset);
}

void main() {
    vec4 current_depth_color = sample_depth_at(ivec2(0));
    vec4 current_main_color = sample_temp_at(vec2(0.0));

    int blur_amount = int(pc.camera_dir.z);

    for (int x = -blur_amount; x <= blur_amount; x++) {
        for (int y = -blur_amount; y <= blur_amount; y++) {
            float dist_from_main_point = blur_amount / ( x * y );

            vec4 xy_depth_color = sample_depth_at(ivec2(x, y));

            if (length(xy_depth_color - current_depth_color) > 0) {
                fragment_color += current_main_color;
                break;
            } else {
                fragment_color += mix(current_main_color, sample_temp_at(vec2(x, y)), clamp(dist_from_main_point, 0, 1));
            }
        }
    }

    fragment_color /= (blur_amount * 2 + 1) * (blur_amount * 2 + 1);

    // brightness
    fragment_color.rgb = fragment_color.rgb + vec3(pc.brightness);

    // contrast
    float factor = (259 * (pc.contrast + 255)) / (255 * (259 - pc.contrast));
    fragment_color.rgb = flatten_rgb(factor * (fragment_color.rgb - 128) + 128);

    // hue & saturation
    vec3 hsl = rgb_to_hsl(fragment_color.rgb);
    hsl.x = fract(hsl.x + pc.hue);
    hsl.y = clamp(hsl.y * pc.saturation, 0, 1);

    fragment_color.rgb = hsl_to_rgb(hsl);

    if (is_letter_here()) {
        fragment_color = vec4(vec3(1.0) - fragment_color.xyz, 1.0);
    }
}
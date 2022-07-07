layout(location = 0) out vec4 fragment_color;
layout(location = 0) in vec4 vertex_color;

layout(set = 0, binding = 0) uniform texture2D u_texture;
layout(set = 0, binding = 1) uniform sampler u_sampler;

const uint FONT_IMAGE_WIDTH = 280;
const uint FONT_IMAGE_HEIGHT = 7;
const uint FONT_IMAGE_AREA = FONT_IMAGE_WIDTH * FONT_IMAGE_HEIGHT;
const uint CHAR_AMOUNT = 32;
const uint CHAR_WIDTH = 7;

layout(std430, set = 1, binding = 0) buffer FontBuffer {
    int data[FONT_IMAGE_AREA];
} font;

bool is_letter_here() {
    uint pos_index = uint(
        floor((vertex_color.y / 2 + 0.5) * CHAR_AMOUNT) * CHAR_AMOUNT +
        floor((vertex_color.x / 2 + 0.5) * CHAR_AMOUNT)
    );

    if (pos_index > 128) {
        return false;
    }

    uint letter_index = pc.text_to_show[pos_index];

    uint char_x = uint(letter_index * CHAR_WIDTH + fract(vertex_color.x / 2 * float(CHAR_AMOUNT)) * 7);
    uint char_y = uint(fract(vertex_color.y / 2 * float(CHAR_AMOUNT)) * 7);

    return font.data[char_y * FONT_IMAGE_WIDTH + char_x] != 0;
}

void main() {
    fragment_color = texture(sampler2D(u_texture, u_sampler), vertex_color.xy / 2.0 + vec2(0.5));

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

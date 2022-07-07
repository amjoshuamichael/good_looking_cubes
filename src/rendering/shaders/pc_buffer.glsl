layout(push_constant) uniform PushConstants {
    vec4 camera_pos;
    vec4 camera_dir;

    vec4 palette[256];

    uint text_to_show[256];

    int time;

    float contrast;
    float brightness;
    float exposure;
    float hue;
    float saturation;
} pc;

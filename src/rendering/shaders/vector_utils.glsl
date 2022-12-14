vec4 flatten_color(vec4 color) {
    return clamp(color, vec4(0.0), vec4(1.0));
}

vec3 flatten_rgb(vec3 color) {
    return clamp(color, vec3(0.0), vec3(1.0));
}

float size_of_min_dimension(vec3 vector) {
    return min(vector.x, min(vector.y, vector.z));
}

float size_of_max_dimension(vec3 vector) {
    return max(vector.x, max(vector.y, vector.z));
}

float sum_of_dimensions(vec3 vector) {
    return vector.x + vector.y + vector.z;
}

//https://stackoverflow.com/questions/15095909/from-rgb-to-hsv-in-opengl-glsl
vec3 rgb_to_hsl(vec3 c) {
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

vec3 hsl_to_rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}
vec4 flatten_color(vec4 color) {
    return clamp(color, vec4(0.0), vec4(1.0));
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
vec4 flatten_color(vec4 color) {
    return clamp(color, vec4(0.0), vec4(1.0));
}
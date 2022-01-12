vec2 positions[6] = vec2[](
    vec2(-1.0, 1.0),
    vec2(1.0, 1.0),
    vec2(-1.0, -1.0),
    vec2(1.0, 1.0),
    vec2(1.0, -1.0),
    vec2(-1.0, -1.0)
);

layout(location = 0) out vec4 vertex_color;

void main() {
    vec2 pos = positions[gl_VertexIndex];
    gl_Position = vec4(pos, 0.0, 1.0);
    vertex_color = gl_Position;
}
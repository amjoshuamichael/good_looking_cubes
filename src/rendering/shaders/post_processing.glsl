layout(location = 0) out vec4 fragment_color;

layout(location = 0) in vec4 vertex_color;

layout(set = 0, binding = 0) uniform texture2D u_texture;
layout(set = 0, binding = 1) uniform sampler u_sampler;


void main() {
    vec4 color = texture(sampler2D(u_texture, u_sampler), vertex_color.xy / 2.0 + vec2(0.5));

    fragment_color = color;
}
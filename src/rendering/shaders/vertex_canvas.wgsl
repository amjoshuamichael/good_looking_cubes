// Because the ray tracing system doesn't require any work with vertices, the vertices just need to be arranged in a
// shape around the viewport, on which the actual graphics can be rendered.

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    switch(in_vertex_index) {
        default: { out.clip_position = vec4<f32>(-1.0, -1.0, 1.0, 1.0); }
        case 1: { out.clip_position = vec4<f32>(1.0, -1.0, 1.0, 1.0); }
        case 2: { out.clip_position = vec4<f32>(-1.0, 1.0, 1.0, 1.0); }
        case 3: { out.clip_position = vec4<f32>(1.0, -1.0, 1.0, 1.0); }
        case 4: { out.clip_position = vec4<f32>(1.0, 1.0, 1.0, 1.0); }
        case 5: { out.clip_position = vec4<f32>(-1.0, 1.0, 1.0, 1.0); }
    }

    out.pos = out.clip_position.xy;

    return out;
}
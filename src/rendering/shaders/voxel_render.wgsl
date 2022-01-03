// The same struct is in vertex_canvas.wgsl
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] pos: vec2<f32>;
};

struct Camera {
    pos: vec3<f32>;
    dir: vec3<f32>;
};
[[group(0), binding(0)]]
var<uniform> camera: Camera;

[[group(0), binding(0)]]
var<private> world: array<bool, 2>;

fn intersection_with_plane_at_z(ro: vec3<f32>, rd: vec3<f32>, z: f32) -> vec3<f32> {
    return vec3<f32> (ro.xy + rd.xy * (z - ro.z), z);
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let fov: f32 = 1.0;
    let rd: vec3<f32> = vec3<f32>(in.pos.x + camera.pos.x, in.pos.y + camera.pos.y, fov + camera.pos.z);
    let ro: vec3<f32> = vec3<f32>(camera.pos);

    let point = intersection_with_plane_at_z(ro, rd, 1.0);

    return vec4<f32>(point, 0.0);
}


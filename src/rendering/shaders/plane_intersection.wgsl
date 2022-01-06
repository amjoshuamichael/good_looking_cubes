fn intersection_with_plane_at_x(ro: vec3<f32>, rd: vec3<f32>, x: f32) -> vec3<f32> {
    let adj = ((x - ro.x) / rd.x);
    return vec3<f32> (x, ro.yz + rd.yz * adj);
}

fn intersection_with_plane_at_y(ro: vec3<f32>, rd: vec3<f32>, y: f32) -> vec3<f32> {
    let adj = ((y - ro.y) / rd.y);
    return vec3<f32> (ro.x + rd.x * adj, y, ro.z + rd.z * adj);
}

fn intersection_with_plane_at_z(ro: vec3<f32>, rd: vec3<f32>, z: f32) -> vec3<f32> {
    let adj = (z - ro.z) / rd.z;
    return vec3<f32> (ro.xy + rd.xy * adj, z);
}
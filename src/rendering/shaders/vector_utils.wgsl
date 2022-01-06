fn flatten_color(in: vec4<f32>) -> vec4<f32> {
    return clamp(in, vec4<f32>(0.0), vec4<f32>(1.0));
}

fn equal2(lhs: vec2<f32>, rhs: vec2<f32>) -> bool {
    let comp = lhs == rhs;
    return comp.x && comp.y;
}

fn equal3(lhs: vec3<f32>, rhs: vec3<f32>) -> bool {
    let comp = lhs == rhs;
    return comp.x && comp.y && comp.z;
}

fn equal4(lhs: vec4<f32>, rhs: vec4<f32>) -> bool {
    let comp = lhs == rhs;
    return comp.w && comp.x && comp.y && comp.z;
}


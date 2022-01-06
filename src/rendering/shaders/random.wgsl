var<private> rand_index: i32 = 0;
fn setup_rand(pos: vec2<f32>) {
    rand_index = i32(pos.x * 5000.0) * 1973 + i32(pos.y * 5000.0) * 9277 | 1;
}

fn rand() -> i32 {
    rand_index = rand_index ^ (rand_index << u32(13));
    rand_index = rand_index ^ (rand_index >> u32(17));
    rand_index = rand_index ^ (rand_index << u32(5));
    return rand_index;
}

fn rand_vector() -> vec3<f32> {
    return normalize(vec3<f32>(vec3<i32>(rand(), rand(), rand())));
}

//var<private> seed: u32 = 0;
//fn setup_rand(pos: vec2<f32>) {
//    seed = u32(pos.x * 5000.0) * u32(1973) + u32(pos.y * 5000.0) * u32(9277) | u32(1);
//}

//fn rand_u32() -> u32 {
//    seed = u32(seed ^ u32(61)) ^ u32(seed >> u32(16));
//    seed = seed * u32(9);
//    seed = seed ^ (seed >> u32(4));
//    seed = seed * u32(0x27d4eb2d);
//    seed = seed ^ (seed >> u32(15));
//    return seed;
//}

//fn rand_float() -> f32 {
//    return f32(rand_u32()) / 4294967296.0;
//}

//fn rand_vector() -> vec3<f32> {
//    return normalize(vec3<f32>(rand_float(), rand_float(), rand_float()));
//}


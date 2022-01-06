struct WorldData {
    data: array<u32, 512>;
};
[[group(1), binding(0)]]
var<uniform> world: WorldData;

fn unit_at(_pos: vec3<f32>) -> u32 {
    if (_pos.x < 0.0 || _pos.x > 7.9 || _pos.y < 0.0 || _pos.y > 7.9 || _pos.z < 0.0 || _pos.z > 7.9) {
        return u32(0);
    }

    let pos = vec3<i32>(_pos);

    return world.data[pos.z * 8 * 8 + pos.y * 8 + pos.x];
}
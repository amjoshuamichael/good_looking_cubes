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

struct WorldData {
    data: array<u32, 512>;
};
[[group(1), binding(0)]]
var<uniform> world: WorldData;

struct hit {
    pos: vec3<f32>;
    normal: vec3<f32>;    
};
var<private> hit: hit;

fn intersection_with_plane_at_z(ro: vec3<f32>, rd: vec3<f32>, z: f32) -> vec3<f32> {
    return vec3<f32> (ro.xy + rd.xy * (z - ro.z), z);
}

fn intersection_with_plane_at_x(ro: vec3<f32>, rd: vec3<f32>, x: f32) -> vec3<f32> {
    return vec3<f32> (
        x,
        ro.y + rd.y * ((x - ro.x) / rd.x),
        ro.z + rd.z * ((x - ro.x) / rd.x)
    );
}

fn intersection_with_plane_at_y(ro: vec3<f32>, rd: vec3<f32>, y: f32) -> vec3<f32> {
    return vec3<f32> (
        ro.x + rd.x * ((y - ro.y) / rd.y),
        y,
        ro.z + rd.z * ((y - ro.y) / rd.y),
    );
}

fn sqr_magnitude(in: vec3<f32>) -> f32 {
    return in.x * in.x + in.y * in.y + in.z * in.z;
}

fn unit_at(_pos: vec3<f32>) -> u32 {
    if (_pos.x < 0.0 || _pos.x > 8.0 || _pos.y < 0.0 || _pos.y > 8.0 || _pos.z < 0.0 || _pos.z > 8.0) {
        return u32(0);
    }

    let pos = vec3<i32>(_pos);

    return world.data[pos.z * 8 * 8 + pos.y * 8 + pos.x];
}

fn sign(in: f32) -> f32 {
    if (in > 0.0) {
        return 1.0;
    } else {
        return -1.0;
    }
}

fn color_in_direction(ro: vec3<f32>, rd: vec3<f32>) -> u32 {
    let ro_floored: vec3<f32> = vec3<f32>(vec3<i32>(ro));

    var x_collision = ro;
    var y_collision = ro;
    var z_collision = ro;
    var x_point = ro;
    var y_point = ro;
    var z_point = ro;

    let u0 = u32(0);

    let x_check_direction = sign(rd.x);
    for (var i: f32 = (x_check_direction / 2.0 + 0.5); i < 10.0; i = i + 1.0) {
        let point = intersection_with_plane_at_x(ro, rd, ro_floored.x + x_check_direction * i);

        if (unit_at(point) != u0) {
            x_collision = point;
            x_point = point;
            break;
        }

        if (unit_at(point - vec3<f32>(1.0, 0.0, 0.0)) != u0) {
            x_collision = point - vec3<f32>(1.0, 0.0, 0.0);
            x_point = point;
            break;
        }
    }

    let y_check_direction = sign(rd.y);
    for (var i: f32 = (y_check_direction / 2.0 + 0.5); i < 10.0; i = i + 1.0) {
        let point = intersection_with_plane_at_y(ro, rd, ro_floored.y + y_check_direction * i);

        if (unit_at(point) > u0) {
            y_collision = point;
            y_point = point;
            break;
        }

        if (unit_at(point - vec3<f32>(0.0, 1.0, 0.0)) != u0) {
            y_collision = point - vec3<f32>(0.0, 1.0, 0.0);
            y_point = point;
            break;
        }
    }

    let z_check_direction = sign(rd.z);
    for (var i: f32 = (z_check_direction / 2.0 + 0.5); i < 20.0; i = i + 1.0) {
        let point = intersection_with_plane_at_z(ro, rd, ro_floored.z + z_check_direction * i);

        if (unit_at(point) > u0) {
            z_collision = point;
            z_point = point;
            break;
        }

        if (unit_at(point - vec3<f32>(0.0, 0.0, 1.0)) != u0) {
            z_collision = point - vec3<f32>(0.0, 0.0, 1.0);
            z_point = point;
            break;
        }
    }

    var x_collision_magnitude = sqr_magnitude(ro - x_point);
    var y_collision_magnitude = sqr_magnitude(ro - y_point);
    var z_collision_magnitude = sqr_magnitude(ro - z_point);

    if (x_collision.x == ro.x) { x_collision_magnitude = 1000.0; }
    if (y_collision.y == ro.y) { y_collision_magnitude = 1000.0; }
    if (z_collision.z == ro.z) { z_collision_magnitude = 1000.0; }

    if (x_collision_magnitude < y_collision_magnitude &&
        x_collision_magnitude < z_collision_magnitude) {
        hit.pos = x_point;
        hit.normal = vec3<f32>( - x_check_direction, 0.0, 0.0);
        return unit_at(x_collision);
    }

    if (y_collision_magnitude < z_collision_magnitude) {
        hit.pos = y_point;
        hit.normal = vec3<f32>(0.0, - y_check_direction, 0.0);
        return unit_at(y_collision);
    }

    if (z_collision.z != ro.z) {
        hit.pos = z_point;
        hit.normal = vec3<f32>(0.0, 0.0, - z_check_direction);
        return unit_at(z_collision);
    }

    //hit.normal = vec3<f32>(0.0, 0.0, 0.0);

    return u32(2147483647);
}

fn color_from(in: u32) -> vec4<f32> {
    let r = in >> u32(24);
    let g = in << u32( 8) >> u32(24);
    let b = in << u32(16) >> u32(24);
    let a = in << u32(24) >> u32(24);

    return vec4<f32>(f32(r), f32(g), f32(b), f32(a));
}

fn flatten(in: vec4<f32>) -> vec4<f32> {
    return clamp(in, vec4<f32>(0.0), vec4<f32>(1.0));
}

fn equal(lhs: vec4<f32>, rhs: vec4<f32>) -> bool {
    let comp = lhs == rhs;
    return comp.w && comp.x && comp.y && comp.z;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let fov: f32 = 1.0;
    let rd_unrotated: vec3<f32> = vec3<f32>(in.pos.x, in.pos.y, fov);
    let rd: vec3<f32> = vec3<f32>(in.pos.x * cos(camera.dir.x), in.pos.y, fov * sin(camera.dir.x));
    let ro: vec3<f32> = vec3<f32>(camera.pos);

    var working_color = color_from(color_in_direction(ro, rd));

    if (equal(working_color, vec4<f32>(1.0))) {
        return mix(working_color, vec4<f32>(1.0), 0.8);
    }

    for (var i: i32 = 0; i < 5; i = i + 1) {
        let reflection = reflect(rd, hit.normal);
        let reflected_color = color_from(color_in_direction(hit.pos, reflection));
    
        working_color = mix(flatten(reflected_color), flatten(working_color), 0.95);
    
        if (equal(reflected_color, vec4<f32>(1.0))) {
            return mix(working_color, vec4<f32>(1.0), 0.95);
        }
    }

    return working_color;
}


struct Camera {
    pos: vec3<f32>;
    dir: vec3<f32>;
    lpos: vec3<f32>;
    ldir: vec3<f32>;
};
[[group(0), binding(0)]]
var<uniform> camera: Camera;

struct hit {
    pos: vec3<f32>;
    normal: vec3<f32>;    
};
var<private> hit: hit;

fn color_from(in: u32) -> vec4<f32> {
    let r = in >> u32(24);
    let g = in << u32( 8) >> u32(24);
    let b = in << u32(16) >> u32(24);
    let a = in << u32(24) >> u32(24);

    return vec4<f32>(f32(r), f32(g), f32(b), f32(a));
}

var<private> z5: f32 = 0.5;

fn color_in_direction(ro: vec3<f32>, rd: vec3<f32>, check_away_from_center: f32) -> u32 {
    let ro_floored = floor(ro);
    let check_dir = sign(rd);
    let check_init = check_dir * 0.5 + 0.5;
    let check_migrate = check_dir * 0.5 - 0.5;

    var x_collision = ro;
    var y_collision = ro;
    var z_collision = ro;
    var x_point = ro;
    var y_point = ro;
    var z_point = ro;

    let u0 = u32(0);

    for (var i: f32 = check_init.x; i < 10.0; i = i + 1.0) {
        let point = intersection_with_plane_at_x(ro, rd, ro_floored.x + check_dir.x * i);
        var check_point = point;
        check_point.x = check_point.x + check_migrate.x;

        if (unit_at(check_point) != u0) {
            x_collision = check_point;
            x_point = point;
            break;
        }
    }

    for (var i: f32 = check_init.y; i < 10.0; i = i + 1.0) {
        let point = intersection_with_plane_at_y(ro, rd, ro_floored.y + check_dir.y * i);
        var check_point = point;
        check_point.y = check_point.y + check_migrate.y;

        if (unit_at(check_point) != u0) {
            y_collision = check_point;
            y_point = point;
            break;
        }
    }

    for (var i: f32 = check_init.z; i < 20.0; i = i + 1.0) {
        let point = intersection_with_plane_at_z(ro, rd, ro_floored.z + check_dir.z * i);
        var check_point = point;
        check_point.z = check_point.z + check_migrate.z;

        if (unit_at(check_point) != u0) {
            z_collision = check_point;
            z_point = point;
            break;
        }
    }

    var x_collision_magnitude = length(ro - x_point);
    var y_collision_magnitude = length(ro - y_point);
    var z_collision_magnitude = length(ro - z_point);

    if (x_collision.x == ro.x) { x_collision_magnitude = 1000.0; }
    if (y_collision.y == ro.y) { y_collision_magnitude = 1000.0; }
    if (z_collision.z == ro.z) { z_collision_magnitude = 1000.0; }

    if (x_collision_magnitude < y_collision_magnitude &&
        x_collision_magnitude < z_collision_magnitude) {
        hit.pos = x_point;
        hit.normal = vec3<f32>( - check_dir.x, 0.0, 0.0);
        return unit_at(x_collision);
    }

    if (y_collision_magnitude < z_collision_magnitude) {
        hit.pos = y_point;
        hit.normal = vec3<f32>(0.0, - check_dir.y, 0.0);
        return unit_at(y_collision);
    }

    if (z_collision.z != ro.z) {
        hit.pos = z_point;
        hit.normal = vec3<f32>(0.0, 0.0, - check_dir.z);
        return unit_at(z_collision);
    }

    return u32(0);
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let num_samples: i32 = 5;
    setup_rand(in.pos.xy);

    let fov: f32 = 1.0;

    let rd_unrotated: vec3<f32> = vec3<f32>(in.pos.x, in.pos.y, fov);
    let rd: vec3<f32> = vec3<f32>(
        cos(camera.dir.x) * in.pos.x - sin(camera.dir.x) * fov, 
        in.pos.y, 
        sin(camera.dir.x) * in.pos.x + cos(camera.dir.x) * fov, 
    );
    let ro: vec3<f32> = vec3<f32>(camera.pos);

    let albedo_color = flatten_color(color_from(color_in_direction(ro, rd, 0.5)));
    let initial_hit_pos = hit.pos;
    if (equal4(albedo_color, vec4<f32>(0.0))) {
        return vec4<f32>(1.0);
    }

    var shadow = vec4<f32>(0.0);

    for (var i = 0; i < num_samples; i = i + 1) {
        let to_light = normalize( - camera.ldir + rand_vector() * camera.dir.z);
        let to_light_unit = color_in_direction(initial_hit_pos, to_light, 1.0);

        if (to_light_unit == u32(0)) {
            shadow = shadow + albedo_color;
        }
    }

    shadow = shadow / f32(num_samples);

    return shadow;


    //return vec4<f32>(to_light, 0.0);
}


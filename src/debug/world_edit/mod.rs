use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use crate::{Background, GPUData, ModelHolder};
use crate::input::MousePos;
use crate::world::{CHUNK_SIZE, CHUNK_COUNT, chunk_position_to_index, ClearWorld};

const EDIT_RAYCAST_DIST: u32 = 100;

#[derive(PartialEq)]
enum WorldChange {
    Delete,
    Create,
}

pub fn edit_world(
    cursor_pos: Res<MousePos>,
    mouse_input: Res<Input<KeyCode>>,
    gpu_data: Res<GPUData>,
    mut clear_world_events: EventWriter<ClearWorld>,
    mut background_models: Query<(&mut ModelHolder, &mut Background)>,
) {
    let world_change = if mouse_input.just_pressed(KeyCode::R) {
        WorldChange::Delete
    } else if mouse_input.just_pressed(KeyCode::F) {
        WorldChange::Create
    } else {
        return;
    };

    let camera_data = Vec3::from_slice(&gpu_data.pos);
    let camera_dir = Vec3::from_slice(&gpu_data.dir);

    for (mut model, mut background) in background_models.iter_mut() {
        if let ModelHolder::Tiled { ref mut filled_spots, .. } = *model {
            let possible_hit = get_pointed_to_tile(&cursor_pos.0, &camera_data, &camera_dir, &filled_spots);

            if let Some(hit) = possible_hit {
                match world_change {
                    WorldChange::Delete => {
                        filled_spots[hit.index] = false;
                        clear_world_events.send(ClearWorld);
                    }
                    WorldChange::Create => {
                        let create_pos = hit.pos + hit.normal;
                        let create_pos_uvec = UVec3::new(
                            create_pos.x as u32,
                            create_pos.y as u32,
                            create_pos.z as u32
                        );
                        filled_spots[chunk_position_to_index(create_pos_uvec)] = true;
                    }
                }

                background.has_been_drawn = false;
            }
        }
    }
}

pub struct Hit {
    pub index: usize,
    pub pos: IVec3,
    pub normal: IVec3,
}

const FOV: f32 = 1.0;

fn get_pointed_to_tile(
    cursor_pos: &Vec2,
    camera_pos: &Vec3,
    camera_dir: &Vec3,
    tile_map: &[bool; CHUNK_COUNT],
) -> Option<Hit> {
    let rd = Vec3::new(
                camera_dir.x.cos() * cursor_pos.x - camera_dir.x.sin() * FOV,
                - cursor_pos.y,
                camera_dir.x.sin() * cursor_pos.x + camera_dir.x.cos() * FOV
            ).normalize();
    let ro = *camera_pos * (1.0 / CHUNK_SIZE as f32);

    // 3D raycasting DDA algorithm

    let mut check_point = ro.floor();

    let xy = rd.x / rd.y;
    let yz = rd.y / rd.z;
    let zx = rd.z / rd.x;
    let xz = rd.x / rd.z;
    let yx = rd.y / rd.x;
    let zy = rd.z / rd.y;

    let ray_unit_step_size = Vec3::new(
        (1.0 + zx * zx + yx * yx).sqrt(),
        (1.0 + xy * xy + zy * zy).sqrt(),
        (1.0 + xz * xz + yz * yz).sqrt()
    );

    let step = rd.signum();
    let mut ray_length = (step * (check_point - ro) + (step / 2.0 + 0.5)) * ray_unit_step_size;

    let mut comp = Vec3::default();
    for _ in 0..EDIT_RAYCAST_DIST {
        comp.x = bool_to_f32(ray_length.x < ray_length.y && ray_length.x <= ray_length.z);
        comp.y = bool_to_f32(ray_length.y < ray_length.z && ray_length.y <= ray_length.x);
        comp.z = bool_to_f32(ray_length.z < ray_length.x && ray_length.z <= ray_length.y);

        check_point += comp * step;

        let check_point_floored = UVec3::new(check_point.x as u32, check_point.y as u32, check_point.z as u32);
        let check_point_index = chunk_position_to_index(check_point_floored);

        if check_point_index > tile_map.len() {
            return None;
        }

        let unit_at_check_point = tile_map[check_point_index];

        if unit_at_check_point == true {
            return Some(Hit {
                index: check_point_index,
                pos: IVec3::new(
                    check_point_floored.x as i32,
                    check_point_floored.y as i32,
                    check_point_floored.z as i32,
                ),
                normal: IVec3::new(
                    ( - comp.x * step.x) as i32,
                    ( - comp.y * step.y) as i32,
                    ( - comp.z * step.z) as i32
                ),
            });
        }

        ray_length += comp * ray_unit_step_size;
    };

    return None;
}

fn bool_to_f32(input: bool) -> f32 {
    if input {
        1.0
    } else {
        0.0
    }
}
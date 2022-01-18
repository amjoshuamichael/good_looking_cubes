use bevy::prelude::*;
use rand::prelude::*;
use crate::{CameraData, DataBuffer};

use crate::world::{CHUNK_SIZE, CHUNK_VOL, CHUNKS_X, CHUNKS_Y, CHUNKS_Z, ChunkUpdate, parse_pec};

fn as_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((a as u32) << 24) +
    ((b as u32) << 16) +
    ((c as u32) <<  8) +
    (d as u32)
}


pub fn load_vox(
    mut world_changes: ResMut<Vec<ChunkUpdate>>,
    mut camera_data_buffer: ResMut<DataBuffer<CameraData>>,
) {
    let mut chunks_to_load: Vec<UVec3> = Vec::new();

    let vox_data = vox_format::from_file("assets/models/monu16.vox").unwrap();
    let vox_pec = parse_pec::parse_pec(include_str!("../../assets/models/monu16.pec"));

    for v in &vox_data.models[0].voxels {
        let chunk_pos = UVec3::new(v.point.x as u32 / 16, v.point.z as u32 / 16,  v.point.y as u32 / 16);
        if !chunks_to_load.contains(&chunk_pos) {
            chunks_to_load.push(chunk_pos);
        }
    }

    for c in &chunks_to_load {
        let mut new_data = [0; CHUNK_VOL];

        for v in &vox_data.models[0].voxels {
            let chunk_pos = UVec3::new(v.point.x as u32 / 16, v.point.z as u32 / 16,  v.point.y as u32 / 16);
            if c.x != chunk_pos.x || c.y != chunk_pos.y || c.z != chunk_pos.z { continue; }

            let color_info = *vox_pec.get(&v.color_index.0).unwrap_or(&0);

            let color = (v.color_index.0 as u32) << 24;

            new_data[
                v.point.x as usize % CHUNK_SIZE +
                v.point.z as usize % CHUNK_SIZE * CHUNK_SIZE +
                v.point.y as usize % CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE
                ] = color + color_info + 1;
        }

        world_changes.push(
            ChunkUpdate {
                pos: *c,
                data: new_data,
            }
        );
    }
}

pub fn change_world(
    mut world_changes: ResMut<Vec<ChunkUpdate>>,
) {
    for _ in 0..4 {
        place_random_sphere(&mut world_changes);
    }
}

fn place_random_sphere(world_changes: &mut ResMut<Vec<ChunkUpdate>>) {
    let mut rng = thread_rng();
    let mut new_data = [0; CHUNK_VOL];

    let color: u32 = rng.gen();
    let size = rng.gen_range(0..64);

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let o_x = x as i32 - CHUNK_SIZE as i32 / 2;
                let o_y = y as i32 - CHUNK_SIZE as i32 / 2;
                let o_z = z as i32 - CHUNK_SIZE as i32 / 2;

                if o_x * o_x + o_y * o_y + o_z * o_z < size {
                    new_data[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE] = color;
                }
            }
        }
    }

    world_changes.push(
        ChunkUpdate {
            pos: UVec3::new(
                rng.gen_range(1..CHUNKS_X) as u32,
                rng.gen_range(1..CHUNKS_Y) as u32,
                rng.gen_range(1..CHUNKS_Z) as u32
            ),
            data: new_data,
        }
    )
}
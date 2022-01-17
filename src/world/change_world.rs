use bevy::prelude::*;
use rand::prelude::*;
use crate::{CameraData, DataBuffer};

use crate::world::{CHUNK_SIZE, CHUNK_VOL, CHUNKS_X, CHUNKS_Y, CHUNKS_Z, ChunkUpdate};

pub fn change_world(
    mut world_changes: ResMut<Vec<ChunkUpdate>>,
    camera_data_buffer: Res<DataBuffer<CameraData>>,
) {
    if camera_data_buffer.data.dir[2] < 1.2 {
        println!("changing world...");
        for _ in 0..4 {
            place_random_sphere(&mut world_changes);
        }
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
                rng.gen_range(0..CHUNKS_X) as u32,
                rng.gen_range(0..CHUNKS_Y) as u32,
                rng.gen_range(0..CHUNKS_Z) as u32
            ),
            // pos: UVec3::ZERO,
            data: new_data,
        }
    )
}
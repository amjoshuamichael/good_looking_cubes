use std::collections::HashMap;
use std::fs::read_to_string;

use bevy::prelude::*;
use gfx_hal::buffer::SubRange;
use gfx_hal::command::{CommandBuffer, CommandBufferFlags};
use gfx_hal::prelude::CommandQueue;

use crate::debug::Command;
use crate::rendering::resources::RenderInfo;
use crate::world::{CHUNK_SIZE, CHUNK_VOL, ChunkUpdate};
use crate::world::parse_pec::parse_pec;

pub fn load_vox_command<B: gfx_hal::Backend>(
    world_changes: EventWriter<ChunkUpdate>,
    mut commands: EventReader<Command>,
    mut command_buffer: ResMut<B::CommandBuffer>,
    mut res: ResMut<RenderInfo<B>>,
) {
    for cmd in commands.iter() {
        if cmd.is("load-vox") {
            let world_buffer = &res.buffers[0];

            unsafe {
                command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);
                command_buffer.fill_buffer(world_buffer, SubRange::WHOLE, 0);
                command_buffer.finish();
                res.queue_group.queues[0].submit_without_semaphores(vec![&*command_buffer], None);
            }

            load_vox(world_changes, cmd.get_arg(0));
            return;
        }
    }
}

fn load_vox(
    mut world_changes: EventWriter<ChunkUpdate>,
    name: &String
) {
    let mut chunks_to_load: Vec<UVec3> = Vec::new();

    let vox_data = vox_format::from_file(format!("assets/models/{}.vox", name)).unwrap();

    let mut vox_pec = HashMap::new();
    if let Ok(file) = read_to_string(format!("assets/models/{}.pec", name)) {
        vox_pec = parse_pec(&file);
    }

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

        world_changes.send(
            ChunkUpdate {
                pos: *c,
                data: new_data,
            }
        );
    }
}
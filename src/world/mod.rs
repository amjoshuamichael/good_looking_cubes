use bevy::app::Plugin;
use bevy::prelude::*;
use gfx_hal::command::{CommandBuffer, CommandBufferFlags};
use gfx_hal::prelude::CommandQueue;

use crate::{App, FixedTimestep, PHYSICS_TIME_STEP};
use crate::rendering::resources::RenderInfo;

pub mod change_world;
mod parse_pec;

pub const CHUNKS_X: usize = 16;
pub const CHUNKS_Y: usize = 16;
pub const CHUNKS_Z: usize = 16;
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOL: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
pub const VOXEL_COUNT: usize = CHUNK_VOL * CHUNKS_X * CHUNKS_Y * CHUNKS_Z;

/// A 16 x 16 x 16 array of data that will be loaded at a specific position in the world data buffer
/// on the gpu before the next frame is rendered.
pub struct ChunkUpdate {
    pos: UVec3,
    data: [u32; CHUNK_VOL],
}

#[derive(Default)]
pub struct CtklrWorldPlugin;

impl Plugin for CtklrWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Vec<ChunkUpdate>>()
            .add_system_set_to_stage(CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(PHYSICS_TIME_STEP))
                    .with_system(update_world::<backend::Backend>)
            );
    }
}

/// Pushes all of the changes from the Vec<ChunkUpdate> in bevy resources into the GPU buffer, and
/// then clears the vector.
fn update_world<B: gfx_hal::Backend>(
    mut command_buffer: ResMut<B::CommandBuffer>,
    mut res: ResMut<RenderInfo<B>>,
    mut chunk_updates: ResMut<Vec<ChunkUpdate>>,
) {
    let world_buffer = &res.buffers[0];

    unsafe {
        command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

        for update in chunk_updates.iter() {
            let offset = (update.pos.x +
                update.pos.y * CHUNKS_X as u32 +
                update.pos.z * CHUNKS_X as u32 * CHUNKS_Y as u32)
                * CHUNK_VOL as u32 * 4;
            command_buffer.update_buffer(world_buffer, offset.into(), bytemuck::cast_slice(&update.data));
        }

        command_buffer.finish();

        res.queue_group.queues[0].submit_without_semaphores(vec![&*command_buffer], None);
    }

    chunk_updates.clear();
}
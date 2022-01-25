use bevy::app::Plugin;
use bevy::prelude::*;
use gfx_hal::command::{CommandBuffer, CommandBufferFlags};
use gfx_hal::prelude::CommandQueue;

use model_loader::ModelAssetPlugin;

use crate::rendering::resources::RenderInfo;
use crate::{App, FixedTimestep, PHYSICS_TIME_STEP};

pub mod change_world;
mod draw;
pub mod load_elements;
pub mod model_holder;
pub mod model_loader;
mod parse_pec;
pub mod tform;
pub mod draw_type;
pub mod model_type;

pub const CHUNKS_X: usize = 16;
pub const CHUNKS_Y: usize = 16;
pub const CHUNKS_Z: usize = 16;
pub const CHUNK_COUNT: usize = CHUNKS_X * CHUNKS_Y * CHUNKS_Z;
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOL: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
pub const VOXEL_COUNT: usize = CHUNK_VOL * CHUNK_COUNT;

/// A 16 x 16 x 16 array
#[derive(Copy, Clone)]
pub struct ChunkData {
    pos: UVec3,
    data: [u32; CHUNK_VOL],
}

pub fn chunk_index_to_position(index: usize) -> UVec3 {
    UVec3::new(
        (index % CHUNKS_X) as u32,
        ((index / CHUNKS_X) % CHUNKS_Y) as u32,
        (index / CHUNKS_X / CHUNKS_Y) as u32,
    )
}

#[derive(Default)]
pub struct CtklrWorldPlugin;

impl Plugin for CtklrWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChunkData>()
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(PHYSICS_TIME_STEP))
                    .with_system(update_world::<backend::Backend>),
            )
            .add_system(change_world::load_vox_command::<backend::Backend>)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::steps_per_second(60.0))
                    .with_system(draw::draw::<backend::Backend>),
            )
            .add_plugin(ModelAssetPlugin)
            .add_startup_system(load_elements::load_elements);
    }
}

/// Pushes all of the changes from the Vec<ChunkUpdate> in bevy resources into the GPU buffer, and
/// then clears the vector.
fn update_world<B: gfx_hal::Backend>(
    mut command_buffer: ResMut<B::CommandBuffer>,
    mut res: ResMut<RenderInfo<B>>,
    mut chunk_updates: EventReader<ChunkData>,
) {
    let world_buffer = &res.buffers[0];

    unsafe {
        command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

        for update in chunk_updates.iter() {
            let offset = (update.pos.x
                + update.pos.y * CHUNKS_X as u32
                + update.pos.z * CHUNKS_X as u32 * CHUNKS_Y as u32)
                * CHUNK_VOL as u32
                * 4;

            command_buffer.update_buffer(
                world_buffer,
                offset.into(),
                bytemuck::cast_slice(&update.data),
            );
        }

        command_buffer.finish();
        res.queue_group.queues[0].submit_without_semaphores(vec![&*command_buffer], None);
    }
}

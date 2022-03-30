use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::utils::HashMap;
use gfx_hal::command::{CommandBuffer, CommandBufferFlags};
use gfx_hal::prelude::CommandQueue;
use gfx_hal::Backend;
use gfx_hal::buffer::SubRange;

use model_loader::ModelAssetPlugin;

use crate::rendering::resources::RenderInfo;
use crate::{App, FixedTimestep, PHYSICS_TIME_STEP};
use crate::world::draw_type::ModelType;

pub mod draw;
pub mod draw_type;
pub mod load_elements;
pub mod model_loader;
pub mod model_type;
mod parse_pec;

pub const CHUNKS_X: usize = 16;
pub const CHUNKS_Y: usize = 16;
pub const CHUNKS_Z: usize = 16;
pub const CHUNK_COUNT: usize = CHUNKS_X * CHUNKS_Y * CHUNKS_Z;
pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_VOL: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;
pub const VOXEL_COUNT: usize = CHUNK_VOL * CHUNK_COUNT;

/// The position of the filled_chunks value in the world buffer.
pub const FILLED_CHUNKS_MEM_OFFSET: usize = VOXEL_COUNT * std::mem::size_of::<u32>();

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

pub fn chunk_position_to_index(pos: UVec3) -> usize {
    pos.x as usize + pos.y as usize * CHUNKS_X + pos.z as usize * CHUNKS_X * CHUNKS_Y
}

pub type WorldUpdates = HashMap<UVec3, Vec<(ModelType, [u32; CHUNK_VOL])>>;

#[derive(Default)]
pub struct CtklrWorldPlugin;

/// used as a bevy event. when sent, the world is cleared.
pub struct ClearWorld;

impl Plugin for CtklrWorldPlugin {
    fn build(&self, app: &mut App) {
        static RENDER: &str = "render";
        static CHANGE_WORLD: &str = "change_world";

        app.add_plugin(ModelAssetPlugin)
            .add_stage_after(CoreStage::Update, RENDER, SystemStage::parallel())
            .add_system(draw::draw_background::draw.label(CHANGE_WORLD))
            .add_system(update_world::<backend::Backend>.after(CHANGE_WORLD))
            .add_event::<ClearWorld>()
            .insert_resource(WorldUpdates::default());
    }
}

pub fn update_world<B: gfx_hal::Backend>(
    mut world_updates: ResMut<WorldUpdates>,
    mut command_buffer: ResMut<B::CommandBuffer>,
    mut render_info: ResMut<RenderInfo<B>>,
    mut should_clear: EventReader<ClearWorld>,
) {
    let world_buffer = &render_info.buffers[0];

    unsafe {
        command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

        if should_clear.iter().count() != 0 {
            clear_world::<B>(world_buffer, &mut command_buffer);
        }

        for (pos, update) in world_updates.iter() {
            let chunk_pos_index =
                pos.x + pos.y * CHUNKS_X as u32 + pos.z * CHUNKS_X as u32 * CHUNKS_Y as u32;

            let voxel_data_offset = chunk_pos_index * (CHUNK_VOL * std::mem::size_of::<u32>()) as u32;

            command_buffer.update_buffer(
                world_buffer,
                voxel_data_offset.into(),
                bytemuck::cast_slice(&update[0].1)
            );

            let chunk_data_offset =
                FILLED_CHUNKS_MEM_OFFSET as u32 + chunk_pos_index * std::mem::size_of::<u32>() as u32;

            command_buffer.update_buffer(
                world_buffer,
                chunk_data_offset.into(),
                &[1],
            )
        }

        command_buffer.finish();
        render_info.queue_group.queues[0].submit_without_semaphores(vec![&*command_buffer], None);
    }

    world_updates.clear();
}

unsafe fn clear_world<B: gfx_hal::Backend>(
    world_buffer: &B::Buffer,
    mut command_buffer: &mut ResMut<B::CommandBuffer>,
) {
    command_buffer.fill_buffer(
        world_buffer,
        SubRange::WHOLE,
        0,
    );
}

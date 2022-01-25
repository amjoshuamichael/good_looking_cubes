use bevy::prelude::*;
use gfx_hal::buffer::SubRange;
use gfx_hal::command::{CommandBuffer, CommandBufferFlags};
use gfx_hal::prelude::CommandQueue;

use crate::debug::Command;
use crate::rendering::resources::RenderInfo;
use crate::world::{ChunkData, draw};
use crate::world::model_loader::Model;

pub fn load_vox_command<B: gfx_hal::Backend>(
    mut world_changes: EventWriter<ChunkData>,
    mut commands: EventReader<Command>,
    mut command_buffer: ResMut<B::CommandBuffer>,
    mut res: ResMut<RenderInfo<B>>,
    asset_server: Res<AssetServer>,
    models: ResMut<Assets<Model>>,
) {
    for cmd in commands.iter() {
        if cmd.is("load-vox") {
            draw::clear_world::<B>(&mut command_buffer, &mut res);

            let model_handle: Handle<Model> =
                asset_server.get_handle(format!("models/{}.vox", cmd.get_arg(0)));
            let model = models.get(model_handle).expect("failed to get model");

            for change in model.voxels.iter() {
                world_changes.send(change.clone())
            }

            return;
        }
    }
}
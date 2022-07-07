use crate::debug::Command;
use crate::input::MousePos;
use crate::world::model_loader::*;
use crate::world::{chunk_position_to_index, ClearWorld, CHUNK_COUNT, CHUNK_SIZE};
use crate::{Background, GPUData, ModelHolder};
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

pub fn load_vox(
    mut debug_commands: EventReader<Command>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut clear_world_events: EventWriter<ClearWorld>,
) {
    for cmd in debug_commands.iter().filter(|cmd| cmd.is("load")) {
        let model_name = cmd.get_arg(0);
        let model: Handle<Model> = asset_server.load(&*format!("models/{model_name}.vox"));
        let model = crate::world::model_type::ModelHolder::new_static(model);

        commands
            .spawn()
            .insert(crate::world::draw_type::Background::default())
            .insert(model);

        clear_world_events.send(ClearWorld::default());
    }
}

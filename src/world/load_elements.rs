use bevy::prelude::*;
use crate::world::model_loader::Model;

pub fn load_elements(
    asset_server: Res<AssetServer>,
    _models: ResMut<Assets<Model>>,
) {
    let _model_handle: Handle<Model> = asset_server.load("models/monu16.vox");
}
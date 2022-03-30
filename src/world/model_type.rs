use super::model_loader::Model;
use crate::world::ChunkData;
use bevy::prelude::*;
use crate::CHUNK_COUNT;

#[derive(Component)]
pub enum ModelHolder {
    Static { model: Handle<Model> },
    Tiled { map: Handle<Model>, filled_spots: [bool; CHUNK_COUNT] },
}

impl ModelHolder {
    pub fn new_static(model: Handle<Model>) -> Self {
        ModelHolder::Static { model }
    }

    pub fn new_tiled(map: Handle<Model>) -> Self {
        ModelHolder::Tiled { map, filled_spots: [false; CHUNK_COUNT] }
    }

    pub fn handle(&self) -> &Handle<Model> {
        match &self {
            ModelHolder::Static {model, ..} => model,
            ModelHolder::Tiled {map, ..} => map,
        }
    }
}
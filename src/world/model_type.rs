use bevy::prelude::*;
use super::model_loader::Model;

#[derive(Component)]
struct StaticModel {
    model: Handle<Model>,
}

impl StaticModel {
    pub fn new(model: Handle<Model>) -> Self {
        StaticModel {
            model,
        }
    }
}
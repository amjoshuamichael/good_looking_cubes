use bevy::prelude::*;

use crate::world::model_loader::Model;

#[derive(Component)]
pub struct ModelHolder( pub Handle<Model> );
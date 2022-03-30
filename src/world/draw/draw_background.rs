use bevy::prelude::*;

use crate::world::draw::draw_model::draw_model;
use crate::world::draw_type::{Background, ModelType};
use crate::world::model_loader::Model;
use crate::world::model_type::ModelHolder;
use crate::world::WorldUpdates;

pub fn draw(
    mut objects_to_draw: Query<(&ModelHolder, &mut Background)>,
    mut world_updates: ResMut<WorldUpdates>,
    models: ResMut<Assets<Model>>,
) {
    for (model_holder, mut background) in objects_to_draw.iter_mut() {
        if background.has_been_drawn { continue }

        println!("drawing!!");

        let model = match models.get(model_holder.handle()) {
            Some(val) => Some(val),
            None => { continue }
        };

        background.has_been_drawn = true;

        let drawn = draw_model(model, model_holder);

        for update in drawn.iter() {
            match world_updates.get_mut(&update.pos) {
                Some(mut updates_at_pos) => {
                    updates_at_pos.push((ModelType::Background, update.data));
                },
                None => {
                    world_updates.insert(update.pos, vec![(ModelType::Background, update.data)]);
                },
            }
        }
    }
}
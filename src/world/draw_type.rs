use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Background {
    pub has_been_drawn: bool,
}

#[derive(Component)]
pub struct Element;

#[derive(Component)]
pub struct Dynamic;

/// Not used as a component, instead used to transmit data about a given model.
pub enum ModelType {
    Background,
    Element,
    Dynamic,
}
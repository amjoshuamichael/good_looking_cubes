use crate::world::CHUNK_COUNT;
use bevy::prelude::*;

/// The way an object is transformed to be placed in the world. Can be as simple as the Basic TForm,
/// which places an object in a specific chunk, or as complex as the Tiled TForm, which describes
/// repeating a model in a specific shape throughout the world, like a TileMap. Named TForm instead
/// of transform to avoid conflicting with the bevy_transform object, which is not used in
/// Caterkiller.
#[derive(Component)]
pub struct TForm {
    pub kind: TFormKind,
    pub draw_properties: DrawProperties,
}

pub enum TFormKind {
    Basic {
        chunk: UVec3,
    },
    Tiled {
        chunk: UVec3,
        filled_spots: [bool; CHUNK_COUNT],
    },
}

pub enum DrawProperties {
    AlwaysRedraw,
    DrawOnChange { has_been_drawn: bool },
}

impl TForm {
    fn voxel_pos(&self) -> UVec3 {
        match self.kind {
            TFormKind::Basic { chunk } => chunk * 16,
            TFormKind::Tiled { chunk, .. } => chunk * 16,
        }
    }
}

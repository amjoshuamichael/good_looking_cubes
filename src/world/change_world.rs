use bevy::prelude::*;
use crate::rendering::data_buffer::*;
use super::world_data::WorldData;
use rand::prelude::*;
use rand::seq::SliceRandom;
use super::size;

pub fn change_world(
    mut world_data_buffer: ResMut<DataBuffer<WorldData>>
) {
    return;

    let mut rng = thread_rng();

    let color: u32 = rng.gen();
    let create_place = rng.gen_range(0..(size * size * size)) as usize;
    world_data_buffer.data.data[create_place] = color;

    let destroy_place = rng.gen_range(0..(size * size * size)) as usize;
    world_data_buffer.data.data[destroy_place] = 0;
}
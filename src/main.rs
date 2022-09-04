use bevy::{core::FixedTimestep, prelude::*};
use rand::{thread_rng, Rng};

use debug::CtklrDebugPlugin;
use input::CtklrInputPlugin;
use rendering::CtklrRenderPlugin;
use world::CtklrWorldPlugin;

use crate::rendering::gpu_data::GPUData;
use crate::world::draw_type::Background;
use crate::world::model_type::ModelHolder;
use crate::world::CHUNK_COUNT;

mod debug;
mod input;
mod rendering;
mod world;

const PHYSICS_TIME_STEP: f64 = 1.0 / 60.0;

fn main() {
    println!("hello!");

    env_logger::init();

    App::new()
        .add_event::<rendering::render::RenderEvent>()
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::transform::TransformPlugin::default())
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy::input::InputPlugin::default())
        .add_plugin(bevy::asset::AssetPlugin::default())
        .add_plugin(bevy::scene::ScenePlugin::default())
        .add_plugin(CtklrRenderPlugin::default())
        .add_plugin(CtklrDebugPlugin::default())
        .add_plugin(CtklrWorldPlugin::default())
        .add_plugin(CtklrInputPlugin::default())
        .add_startup_system(spawn_tilemap.before("spawn"))
        .add_startup_system(change_tilemap.label("change"))
        .run();
}

fn change_tilemap(mut model_holders: Query<(&mut Background, &mut ModelHolder)>) {
    for (mut background, mut model_holder) in model_holders.iter_mut() {
        if background.has_been_drawn == true {
            return;
        }

        if let ModelHolder::Tiled {
            ref mut filled_spots,
            ..
        } = *model_holder
        {
            for spot in filled_spots.iter_mut() {
                *spot = false;
            }

            let mut rng = thread_rng();

            for _ in 1..100 {
                let pos: usize = rng.gen_range(0..CHUNK_COUNT);

                filled_spots[pos] = true;
            }
        }
    }
}

fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut tiled =
        crate::world::model_type::ModelHolder::new_tiled(asset_server.load("models/block.vox"));

    if let ModelHolder::Tiled {
        ref mut filled_spots,
        ..
    } = tiled
    {
        let mut rng = thread_rng();

        for _ in 1..100 {
            let pos: usize = rng.gen_range(0..CHUNK_COUNT);

            filled_spots[pos] = true;
        }
    }

    commands
        .spawn()
        .insert(crate::world::draw_type::Background::default())
        .insert(tiled);
}

use bevy::{
    core::FixedTimestep,
    prelude::*
};

use debug::CtklrDebugPlugin;
use rendering::CtklrRenderPlugin;
use world::CtklrWorldPlugin;
use input::CtklrInputPlugin;

use crate::rendering::gpu_data::GPUData;

mod rendering;
mod world;
mod debug;
mod input;

const PHYSICS_TIME_STEP: f64 = 1.0 / 60.0;

fn main() {
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
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(PHYSICS_TIME_STEP))
        //         .with_system(world::change_world::change_world)
        // )
        .run();
}


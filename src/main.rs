use bevy::{
    core::FixedTimestep,
    prelude::*
};

use debug::CtklrDebugPlugin;
use rendering::CtklrRenderPlugin;
use world::CtklrWorldPlugin;

use crate::rendering::camera_data_buffer::CameraData;
use crate::rendering::data_buffer::DataBuffer;

mod rendering;
mod world;
mod debug;

const PHYSICS_TIME_STEP: f64 = 1.0 / 60.0;

fn main() {
    env_logger::init();

    let vox_data = vox_format::from_file("assets/models/chr_sword.vox").unwrap();
    println!("{:#?}", vox_data);

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
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(PHYSICS_TIME_STEP))
                .with_system(input_test)
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(PHYSICS_TIME_STEP))
                .with_system(world::change_world::change_world)
        )
        .run();
}

fn input_test(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_data_buffer: ResMut<DataBuffer<CameraData>>
) {
    let mut move_speed = 0.02;

    if keyboard_input.pressed(KeyCode::Return) {
        move_speed = 0.1;
    }

    if keyboard_input.pressed(KeyCode::A) {
        camera_data_buffer.data.pos[0] -= move_speed;
    } else if keyboard_input.pressed(KeyCode::D) {
        camera_data_buffer.data.pos[0] += move_speed;
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        camera_data_buffer.data.pos[1] -= move_speed;
    } else if keyboard_input.pressed(KeyCode::Space) {
        camera_data_buffer.data.pos[1] += move_speed;
    }

    if keyboard_input.pressed(KeyCode::S) {
        camera_data_buffer.data.pos[2] -= move_speed;
    } else if keyboard_input.pressed(KeyCode::W) {
        camera_data_buffer.data.pos[2] += move_speed;
    }

    if keyboard_input.pressed(KeyCode::T) {
        camera_data_buffer.data.dir[2] -= move_speed;
    } else if keyboard_input.pressed(KeyCode::Y) {
        camera_data_buffer.data.dir[2] += move_speed;
    }

    if keyboard_input.pressed(KeyCode::Q) {
        camera_data_buffer.data.dir[0] -= move_speed;
    } else if keyboard_input.pressed(KeyCode::E) {
        camera_data_buffer.data.dir[0] += move_speed;
    }
}


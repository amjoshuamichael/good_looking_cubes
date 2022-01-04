use bevy::{
    prelude::*,
    core::FixedTimestep
};
use winit::event::DeviceEvent::Key;

mod rendering;
mod world;
use rendering::create_window::CtklrWindowPlugin;
use rendering::render_system::DataBuffer;
use rendering::camera_data_buffer::CameraData;

const TIME_STEP: f64 = 1.0 / 60.0;

fn main() {
    env_logger::init();

    App::new()
        .add_plugin(bevy::core::CorePlugin::default())
        .add_plugin(bevy::transform::TransformPlugin::default())
        .add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
        .add_plugin(bevy::input::InputPlugin::default())
        .add_plugin(bevy::asset::AssetPlugin::default())
        .add_plugin(bevy::scene::ScenePlugin::default())
        .add_plugin(bevy::gilrs::GilrsPlugin::default())
        .add_plugin(bevy::gltf::GltfPlugin::default())
        .add_plugin(CtklrWindowPlugin::default())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(input_test)
        )
        .run();
}

fn input_test(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_data_buffer: ResMut<DataBuffer<CameraData>>
) {
    if keyboard_input.pressed(KeyCode::A) {
        camera_data_buffer.data.pos[0] -= 0.02;
    } else if keyboard_input.pressed(KeyCode::D) {
        camera_data_buffer.data.pos[0] += 0.02;
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        camera_data_buffer.data.pos[1] -= 0.02;
    } else if keyboard_input.pressed(KeyCode::Space) {
        camera_data_buffer.data.pos[1] += 0.02;
    }

    if keyboard_input.pressed(KeyCode::S) {
        camera_data_buffer.data.pos[2] -= 0.02;
    } else if keyboard_input.pressed(KeyCode::W) {
        camera_data_buffer.data.pos[2] += 0.02;
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        camera_data_buffer.data.pos[1] -= 0.02;
    } else if keyboard_input.pressed(KeyCode::Space) {
        camera_data_buffer.data.pos[1] += 0.02;
    }

    if keyboard_input.pressed(KeyCode::Q) {
        camera_data_buffer.data.dir[0] -= 0.02;
    } else if keyboard_input.pressed(KeyCode::E) {
        camera_data_buffer.data.dir[0] += 0.02;
    }
}
use bevy::prelude::*;
use bevy::prelude::KeyCode::*;
use std::collections::HashMap;
use crate::GPUData;

#[derive(Default)]
pub struct CtklrInputPlugin;

impl Plugin for CtklrInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_movement)
            .insert_resource(InputState::FreeCam);
    }
}

#[derive(PartialEq)]
pub enum InputState {
    Commands,
    FreeCam,
}

impl Default for InputState {
    fn default() -> Self { InputState::FreeCam }
}

fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    input_state: Res<InputState>,
    mut gpu_data: ResMut<GPUData>,
) {
    if *input_state != InputState::FreeCam { return; }

    let mut move_speed = 0.02;

    if keyboard_input.pressed(Return) {
        move_speed = 0.1;
    }

    if keyboard_input.pressed(A) {
        gpu_data.pos[0] -= move_speed;
    } else if keyboard_input.pressed(D) {
        gpu_data.pos[0] += move_speed;
    }

    if keyboard_input.pressed(LShift) {
        gpu_data.pos[1] -= move_speed;
    } else if keyboard_input.pressed(Space) {
        gpu_data.pos[1] += move_speed;
    }

    if keyboard_input.pressed(S) {
        gpu_data.pos[2] -= move_speed;
    } else if keyboard_input.pressed(W) {
        gpu_data.pos[2] += move_speed;
    }

    if keyboard_input.pressed(T) {
        gpu_data.dir[2] -= move_speed;
    } else if keyboard_input.pressed(Y) {
        gpu_data.dir[2] += move_speed;
        println!("{}", gpu_data.dir[2]);
    }

    if keyboard_input.pressed(Q) {
        gpu_data.dir[0] -= move_speed;
    } else if keyboard_input.pressed(E) {
        gpu_data.dir[0] += move_speed;
    }
}
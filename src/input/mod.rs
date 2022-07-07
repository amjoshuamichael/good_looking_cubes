use crate::GPUData;
use bevy::prelude::KeyCode::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct CtklrInputPlugin;

impl Plugin for CtklrInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_movement)
            .insert_resource(KeyboardInputState::default())
            .insert_resource(CursorInputState::TilemapEdit)
            .insert_resource(GamepadInputState::Play)
            .insert_resource(MousePos(Vec2::new(0.0, 0.0)));
    }
}

#[derive(PartialEq)]
pub enum KeyboardInputState {
    Commands,
    FreeCam,
}

impl Default for KeyboardInputState {
    fn default() -> Self {
        KeyboardInputState::FreeCam
    }
}

#[derive(PartialEq)]
pub enum CursorInputState {
    TilemapEdit,
    Play,
}

#[derive(PartialEq)]
pub enum GamepadInputState {
    Play,
}

pub struct MousePos(pub Vec2);

fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    input_state: Res<KeyboardInputState>,
    mut gpu_data: ResMut<GPUData>,
) {
    if *input_state != KeyboardInputState::FreeCam {
        return;
    }

    let mut move_speed = 0.02;

    if keyboard_input.pressed(Return) {
        move_speed = 1.0;
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

    if keyboard_input.just_pressed(P) {
        gpu_data.time += 1;
        println!("{}", gpu_data.time);
    }

    //gpu_data.time += 1;
}

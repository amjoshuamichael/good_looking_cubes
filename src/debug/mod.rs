#![allow(dead_code)]

use crate::input::KeyboardInputState;
use crate::GPUData;
use bevy::core::FixedTimestep;
use bevy::prelude::KeyCode::*;
use bevy::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

mod load_vox;
mod log_framerate;
mod world_edit;

const DEBUG_TIME_STEP: f64 = 1.0 / 5.0;

#[derive(Default)]
pub struct CtklrDebugPlugin;

impl Plugin for CtklrDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(DEBUG_TIME_STEP))
                .with_system(log_framerate::log_framerate),
        )
        .add_system(command_input)
        .add_system(world_edit::edit_world)
        .add_system(load_vox::load_vox)
        .add_event::<Command>();
    }
}

lazy_static! {
    static ref KEYBOARD_MAP: HashMap<KeyCode, u32> = HashMap::from([
        (A, 1),
        (B, 2),
        (C, 3),
        (D, 4),
        (E, 5),
        (F, 6),
        (G, 7),
        (H, 8),
        (I, 9),
        (J, 10),
        (K, 11),
        (L, 12),
        (M, 13),
        (N, 14),
        (O, 15),
        (P, 16),
        (Q, 17),
        (R, 18),
        (S, 19),
        (T, 20),
        (U, 21),
        (V, 22),
        (W, 23),
        (X, 24),
        (Y, 25),
        (Z, 26),
        (Key1, 27),
        (Key2, 28),
        (Key3, 29),
        (Key4, 30),
        (Key5, 31),
        (Key6, 32),
        (Key7, 33),
        (Key8, 34),
        (Key9, 35),
        (Key0, 36),
        (Period, 37),
        (Minus, 38),
        (Space, 39),
    ]);
    static ref NUM_TO_CHAR: HashMap<u32, char> = HashMap::from([
        (1, 'a'),
        (2, 'b'),
        (3, 'c'),
        (4, 'd'),
        (5, 'e'),
        (6, 'f'),
        (7, 'g'),
        (8, 'h'),
        (9, 'i'),
        (10, 'j'),
        (11, 'k'),
        (12, 'l'),
        (13, 'm'),
        (14, 'n'),
        (15, 'o'),
        (16, 'p'),
        (17, 'q'),
        (18, 'r'),
        (19, 's'),
        (20, 't'),
        (21, 'u'),
        (22, 'v'),
        (23, 'w'),
        (24, 'x'),
        (25, 'y'),
        (26, 'z'),
        (27, '1'),
        (28, '2'),
        (29, '3'),
        (30, '4'),
        (31, '5'),
        (32, '6'),
        (33, '7'),
        (34, '8'),
        (35, '9'),
        (36, '0'),
        (37, '.'),
        (38, '-'),
        (39, ' '),
    ]);
}

pub fn command_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut input_state: ResMut<KeyboardInputState>,
    mut gpu_data: ResMut<GPUData>,
    mut command_events: EventWriter<Command>,
) {
    if keyboard_input.pressed(LShift) && keyboard_input.just_pressed(C) {
        *input_state = KeyboardInputState::Commands;
        return;
    }

    if *input_state != KeyboardInputState::Commands {
        return;
    }

    if keyboard_input.pressed(Escape) {
        *input_state = KeyboardInputState::default();
        clear_text(&mut gpu_data.text_to_show);
        return;
    } else if keyboard_input.pressed(Return) {
        *input_state = KeyboardInputState::default();
        command_events.send(parse_command(&gpu_data.text_to_show));
        clear_text(&mut gpu_data.text_to_show);
        return;
    }

    for input in keyboard_input.get_just_pressed() {
        let new_char_pos = gpu_data
            .text_to_show
            .iter()
            .position(|&c| c == 0)
            .expect("no more room in command!");

        if input == &Back {
            gpu_data.text_to_show[new_char_pos - 1] = 0;
        } else {
            gpu_data.text_to_show[new_char_pos] = *KEYBOARD_MAP.get(input).unwrap_or(&0);
        }
    }
}

fn clear_text(text: &mut [u32; 256]) {
    for c in text.iter_mut() {
        *c = 0
    }
}

fn parse_command(text: &[u32; 256]) -> Command {
    let mut cmd_string = String::new();

    for c in text {
        if let Some(char_to_push) = NUM_TO_CHAR.get(c) {
            cmd_string.push(*char_to_push);
        }
    }

    let mut cmd_split = cmd_string.split(" ");
    let function = cmd_split.next().expect("empty command").to_string();
    let arguments = cmd_split.map(|str| str.to_string()).collect();

    Command {
        function,
        arguments,
    }
}

#[derive(Debug)]
pub struct Command {
    pub function: String,
    pub arguments: Vec<String>,
}

impl Command {
    pub fn is(&self, check: &str) -> bool {
        check.to_string() == self.function
    }

    //TODO: use generics here

    pub fn parse(&self) -> u32 {
        self.parse_arg_at(0)
    }

    pub fn parse_arg_at(&self, index: usize) -> u32 {
        self.arguments[index]
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("cmd err: expected a(n) u32 for arg {}.", index))
    }

    pub fn parse_f32(&self) -> f32 {
        self.parse_arg_at_f32(0)
    }

    pub fn parse_arg_at_f32(&self, index: usize) -> f32 {
        self.arguments[index]
            .parse::<f32>()
            .unwrap_or_else(|_| panic!("cmd err: expected a(n) u32 for arg {}.", index))
    }

    pub fn get_arg(&self, index: usize) -> &String {
        &self.arguments[index]
    }
}

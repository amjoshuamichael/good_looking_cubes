use bevy::prelude::*;
use crate::debug::Command;
use crate::GPUData;

pub fn setup_picture_data(
    mut commands: EventReader<Command>,
    mut gpu_data_buffer: ResMut<GPUData>,
) {
    for cmd in commands.iter() {
        if cmd.is("contrast") { gpu_data_buffer.contrast = cmd.parse_f32(); }
        if cmd.is("brightness") { gpu_data_buffer.brightness = cmd.parse_f32(); }
        if cmd.is("exposure") { gpu_data_buffer.exposure = cmd.parse_f32(); }
        if cmd.is("saturation") { gpu_data_buffer.saturation = cmd.parse_f32(); }
        if cmd.is("hue") { gpu_data_buffer.hue = cmd.parse_f32(); }
    }
}
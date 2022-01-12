use bevy::prelude::*;
use bevy::app::Events;
use crate::rendering::render::RenderEvent;

pub fn log_framerate(
    mut render_event_reader: EventReader<RenderEvent>,
) {
    for event in render_event_reader.iter() {
        println!("current path-trace rate: {}fps",
                 (1.0 / event.time.elapsed().as_seconds_f64()) as u32
        );
    }
}

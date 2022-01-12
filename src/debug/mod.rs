use bevy::core::FixedTimestep;
use bevy::prelude::*;

mod log_framerate;

const DEBUG_TIME_STEP: f64 = 1.0 / 5.0;

#[derive(Default)]
pub struct CtklrDebugPlugin;

impl Plugin for CtklrDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::new()
            .with_run_criteria(FixedTimestep::step(DEBUG_TIME_STEP))
            .with_system(log_framerate::log_framerate)
        );
    }
}
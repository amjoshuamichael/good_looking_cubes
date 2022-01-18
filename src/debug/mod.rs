use bevy::core::FixedTimestep;
use bevy::prelude::*;
use std::any::type_name;
use std::str::FromStr;

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

pub struct Command {
    function: String,
    arguments: Vec<String>,
}

impl Command {
    pub fn is(&self, check: &str) -> bool {
        check.to_string() == self.function
    }

    pub fn parse_as<T: FromStr>(&self) -> T {
        self.parse_arg_at_as::<T>(0)
    }

    pub fn parse_two_as<T: FromStr, S: FromStr>(&self) -> (T, S) {
        (self.parse_arg_at_as::<T>(0), self.parse_arg_at_as::<S>(1))
    }

    pub fn parse_arg_at_as<T: FromStr>(&self, index: usize) -> T {
        self.arguments[index].parse::<T>()
            .unwrap_or(panic!("cmd err: expected a(n) {} for arg {}.", type_name::<T>(), index))
    }
}
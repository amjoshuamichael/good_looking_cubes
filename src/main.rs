use bevy::{core::FixedTimestep, prelude::*};
use rand::{Rng, thread_rng};

use debug::CtklrDebugPlugin;
use input::CtklrInputPlugin;
use rendering::CtklrRenderPlugin;
use world::CtklrWorldPlugin;

use crate::rendering::gpu_data::GPUData;
use crate::world::CHUNK_COUNT;
use crate::world::model_holder::ModelHolder;
use crate::world::tform::{DrawProperties, TFormKind};

mod debug;
mod input;
mod rendering;
mod world;

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
        .add_startup_system(spawn_tilemap)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(change_tilemap)
        )
        .run();
}

fn change_tilemap(
    mut objects_to_draw: Query<&mut crate::world::tform::TForm, With<ModelHolder>>,
) {
    for mut t in objects_to_draw.iter_mut() {
        if let TFormKind::Tiled{ ref mut chunk, ref mut filled_spots } = t.kind {
            // for spot in filled_spots.iter_mut() { *spot = false; }

            let mut rng = thread_rng();

            for _ in 1..100 {
                let pos: usize = rng.gen_range(0..CHUNK_COUNT);

                filled_spots[pos] = true;
            }
        }
    }
}

fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(crate::world::tform::TForm {
            kind: TFormKind::Tiled {
                chunk: UVec3::new(0, 0, 0),
                filled_spots: [false; CHUNK_COUNT],
            },
            draw_properties: DrawProperties::DrawOnChange { has_been_drawn: false },
        })
        .insert(crate::world::model_holder::ModelHolder(
            asset_server.load("models/block.vox"),
        ));
}

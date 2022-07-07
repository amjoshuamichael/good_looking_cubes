use std::fs::read_to_string;

use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

use crate::world::parse_pec::parse_pec;
use crate::world::{ChunkData, CHUNK_SIZE, CHUNK_VOL};

#[derive(Default)]
pub struct ModelAssetPlugin;

impl Plugin for ModelAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Model>().init_asset_loader::<ModelLoader>();
    }
}

/// A set of voxels that can be duplicated and placed throughout the level.
#[derive(TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct Model {
    // A set of 16 x 16 sets of voxel data paired with their positions relative to each other.
    pub voxels: Vec<ChunkData>,
}

impl Model {
    pub fn new() -> Self {
        Model { voxels: Vec::new() }
    }
}

#[derive(Default)]
pub struct ModelLoader;

impl AssetLoader for ModelLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            //TODO: optimize
            let pec = {
                let model_path = load_context
                    .path()
                    .file_name()
                    .expect("could not get file name for model")
                    .to_str()
                    .expect("could not convert file name to string");

                // remove .vox file extension by cutting out the last 4 characters

                let model_name = &model_path[0..model_path.len() - 4];

                let pec_file_data = read_to_string(format!("assets/models/{}.pec", model_name))
                    .unwrap_or_else(|_| "".into());

                parse_pec(pec_file_data)
            };

            let vox_data =
                vox_format::from_slice(bytes).expect("could not parse vox data from file");

            let mut chunks_to_load: Vec<UVec3> = Vec::new();

            for v in &vox_data.models[0].voxels {
                let chunk_pos = UVec3::new(
                    v.point.x as u32 / 16,
                    v.point.z as u32 / 16,
                    v.point.y as u32 / 16,
                );
                if !chunks_to_load.contains(&chunk_pos) {
                    chunks_to_load.push(chunk_pos);
                }
            }

            let mut output_model = Model::new();

            for c in &chunks_to_load {
                let mut new_data = [0; CHUNK_VOL];

                for v in &vox_data.models[0].voxels {
                    let chunk_pos = UVec3::new(
                        v.point.x as u32 / 16,
                        v.point.z as u32 / 16,
                        v.point.y as u32 / 16,
                    );

                    if c.x != chunk_pos.x || c.y != chunk_pos.y || c.z != chunk_pos.z {
                        continue;
                    }

                    let color_info = pec.get(&v.color_index.0).unwrap_or(&0);

                    let color = (v.color_index.0 as u32) << 24;

                    new_data[v.point.x as usize % CHUNK_SIZE
                        + v.point.z as usize % CHUNK_SIZE * CHUNK_SIZE
                        + v.point.y as usize % CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE] =
                        color + color_info + 1;
                }

                output_model.voxels.push(ChunkData {
                    pos: *c,
                    data: new_data,
                });
            }

            load_context.set_default_asset(LoadedAsset::new(output_model));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vox"]
    }
}

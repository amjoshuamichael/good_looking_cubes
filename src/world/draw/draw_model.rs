use crate::CHUNK_COUNT;
use crate::world::{chunk_index_to_position, ChunkData};
use crate::world::model_loader::Model;
use crate::world::model_type::ModelHolder;

pub fn draw_model(possible_model: Option<&Model>, model_holder: &ModelHolder) -> Vec<ChunkData> {
    let model = match possible_model {
        Some(loaded_model) => loaded_model,
        None => return Vec::new(),
    };

    match model_holder {
        ModelHolder::Static {..} => draw_static(model),
        ModelHolder::Tiled {filled_spots, ..} => draw_tiled(model, &filled_spots),
    }
}

fn draw_static(model: &Model) -> Vec<ChunkData> {
    let mut output = Vec::new();

    for chunk_data in &model.voxels {
        let translated_pos = chunk_data.pos;

        output.push(ChunkData {
            pos: translated_pos,
            data: chunk_data.data.clone(),
        });
    }

    output
}

fn draw_tiled(model: &Model, filled_spots: &[bool; CHUNK_COUNT]) -> Vec<ChunkData> {
    let mut output = Vec::new();

    for (index, spot_is_filled) in filled_spots.iter().enumerate() {
        if !spot_is_filled { continue }

        let tiled_pos = chunk_index_to_position(index);

        output.push(ChunkData {
            pos: tiled_pos,
            data: model.voxels[0].data.clone(),
        });
    }

    output
}
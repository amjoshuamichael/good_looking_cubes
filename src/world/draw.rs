use bevy::prelude::*;
use gfx_hal::command::{CommandBuffer, CommandBufferFlags};
use gfx_hal::buffer::SubRange;
use gfx_hal::prelude::CommandQueue;
use crate::rendering::resources::RenderInfo;

use crate::world::model_holder::ModelHolder;
use crate::world::model_loader::Model;
use crate::world::tform::TForm;
use crate::world::{chunk_index_to_position, ChunkData};
use crate::{DrawProperties, TFormKind};

pub fn draw<B: gfx_hal::Backend>(
    objects_to_draw: Query<(&TForm, &ModelHolder)>,
    models: ResMut<Assets<Model>>,
    mut world_changes: EventWriter<ChunkData>,
    mut command_buffer: ResMut<B::CommandBuffer>,
    mut res: ResMut<RenderInfo<B>>,
) {
    // start by clearing the canvas
    clear_world::<B>(&mut command_buffer, &mut res);

    for (t, m) in objects_to_draw.iter() {
        let model = match models.get(&m.0) {
            Some(val) => val,
            None => {
                continue;
            }
        };

        match t.draw_properties {
            DrawProperties::DrawOnChange { mut has_been_drawn } => {
                if has_been_drawn {
                    continue;
                } else {
                    has_been_drawn = false;
                }
            }
        }

        match t.kind {
            TFormKind::Basic { chunk } => {
                for chunk_data in &model.voxels {
                    let translated_pos = chunk_data.pos + chunk;

                    world_changes.send(ChunkData {
                        pos: translated_pos,
                        data: chunk_data.data.clone(),
                    });
                }
            }
            TFormKind::Tiled { chunk, filled_spots } => {
                for (index, spot_is_filled) in filled_spots.iter().enumerate() {
                    if *spot_is_filled {
                        let translated_pos = chunk_index_to_position(index) + chunk;

                        world_changes.send(ChunkData {
                            pos: translated_pos,
                            data: model.voxels[0].data.clone(),
                        });
                    }
                }
            }
        }
    }
}

// TODO: make this its own bevy system
pub fn clear_world<B: gfx_hal::Backend>(
    command_buffer: &mut B::CommandBuffer,
    res: &mut RenderInfo<B>,
) {
    let world_buffer = &res.buffers[0];

    unsafe {
        command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);
        command_buffer.fill_buffer(world_buffer, SubRange::WHOLE, 0);
        command_buffer.finish();
        res.queue_group.queues[0].submit_without_semaphores(vec![&*command_buffer], None);
    }
}

use gfx_hal::prelude::*;
use std::fs::File;
use gfx_hal::buffer::SubRange;
use gfx_hal::command::CommandBufferFlags;
use gfx_hal::format::Format;
use gfx_hal::image::Kind;
use gfx_hal::queue::QueueGroup;
use crate::rendering::constructs::create_buffer_bindings::create_buffer_bindings;
use crate::rendering::constructs::memory::{create_buffer, create_image};

/// Converts an image from a file into a buffer (not an image, a buffer containing image data)
/// that can be read from the gpu.
pub unsafe fn create_image_buffer_from_file<B: gfx_hal::Backend>(
    device: &B::Device,
    mut command_buffer: &mut B::CommandBuffer,
    mut queue_group: &mut QueueGroup<B>,
    file: &str,
) -> B::Buffer {
    let decoder = png::Decoder::new(File::open(file).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];

    // load the bytes into a temporary buffer and then move those bytes into an image.
    let buffer = create_buffer::<B>(&device, bytes.len() as u64);
    command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);
    command_buffer.update_buffer(&buffer, 0, bytes);
    command_buffer.finish();
    queue_group.queues[0].submit_without_semaphores(vec![&*command_buffer], None);

    buffer
}
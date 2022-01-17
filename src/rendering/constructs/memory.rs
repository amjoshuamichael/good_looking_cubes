use gfx_hal::MemoryTypeId;
use gfx_hal::device::Device;
use gfx_hal::format::Format;
use gfx_hal::image::{Kind, Tiling, ViewCapabilities};

pub unsafe fn create_buffer<B: gfx_hal::Backend>(
    device: &B::Device,
    size: u64,
) -> B::Buffer {
    let mut buffer = device
        .create_buffer(
            size,
            gfx_hal::buffer::Usage::all(),
        )
        .expect("failed to create world data buffer");

    let memory = device
        .allocate_memory(
            MemoryTypeId(0),
            device.get_buffer_requirements(&buffer).size,
        )
        .expect("failed to allocate memory for world data buffer");

    device
        .bind_buffer_memory(
            &memory,
            0,
            &mut buffer,
        )
        .expect("failed to bind memory");

    buffer
}

pub unsafe fn create_image<B: gfx_hal::Backend> (
    device: &B::Device,
    kind: Kind,
    format: Format,
) -> B::Image {
    let mut image = device
        .create_image(
            kind,
            1,
            format,
            Tiling::Linear,
            gfx_hal::image::Usage::all(),
            ViewCapabilities::empty(),
        )
        .unwrap();

    let memory = device
        .allocate_memory(
            MemoryTypeId(0),
            device.get_image_requirements(&image).size,
        )
        .expect("failed to allocate memory for world data buffer");

    device
        .bind_image_memory(
            &memory,
            0,
            &mut image,
        )
        .expect("failed to bind memory");

    image
}
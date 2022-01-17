use gfx_hal::adapter::Adapter;
use gfx_hal::prelude::*;
use gfx_hal::queue::QueueGroup;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event_loop::EventLoop;

pub fn get_sizes(
    event_loop: &EventLoop<()>,
    sizes: [u32; 2],
) -> (LogicalSize<u32>, PhysicalSize<u32>) {
    let dpi = event_loop.primary_monitor().unwrap().scale_factor();
    let logical: LogicalSize<u32> = sizes.into();
    let physical: PhysicalSize<u32> = logical.to_physical(dpi);

    (logical, physical)
}

pub unsafe fn device_info<B: gfx_hal::Backend>(
    surface: &B::Surface,
    adapter: &Adapter<B>,
) -> (B::Device, QueueGroup<B>) {
    let queue_family = adapter
        .queue_families
        .iter()
        .find(|family| {
            surface.supports_queue_family(family) && family.queue_type().supports_graphics()
        })
        .expect("No compatible queue family found");

    let mut gpu = adapter
        .physical_device
        .open(&[(queue_family, &[1.0])], gfx_hal::Features::empty())
        .expect("Failed to open device");

    (gpu.device, gpu.queue_groups.pop().unwrap())
}
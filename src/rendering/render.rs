use std::fmt::Debug;
use std::io::Read;
use bevy::prelude::*;
use bevy::app::Events;
use bevy::render::render_resource::Buffer;
use gfx_hal::buffer::SubRange;
use gfx_hal::device::BindError;
use time::Instant;
use gfx_hal::prelude::*;
use crate::CameraData;
use crate::rendering::data_buffer::DataBuffer;
use crate::world::world_data::WorldData;
use super::create_window::{RenderInfo, ResourceHolder, Resources};

pub struct RenderEvent {
    pub time: Instant,
}

const SCALE_FACTOR: u32 = 2;

pub fn render_draw<B: gfx_hal::Backend>(
    mut render_info: &mut RenderInfo<B>,
    mut resource_holder: &mut ResourceHolder<B>,
    mut command_buffer: &mut B::CommandBuffer,
    camera_data_buffer: &DataBuffer<CameraData>,
    world_data_buffer: &DataBuffer<WorldData>,
    mut should_configure_swapchain: bool,
) {
    let res: &mut Resources<_> = &mut resource_holder.0;
    let render_pass = &res.render_passes[0];
    let temp_pipeline_layout = &res.pipeline_layouts[0];
    let temp_pipeline = &res.render_pipelines[0];
    let temp_image_view = &res.image_views[0];
    let surface_pipeline_layout = &res.pipeline_layouts[1];
    let surface_pipeline = &res.render_pipelines[1];
    let world_buffer = &res.buffers[0];

    unsafe {
        // We refuse to wait more than a second, to avoid hanging.
        let render_timeout_ns = 1_000_000_000;

        res.device
            .wait_for_fence(&res.submission_complete_fence, render_timeout_ns)
            .expect("Out of memory or device lost");

        res.device
            .reset_fence(&res.submission_complete_fence)
            .expect("Out of memory");

        res.command_pool.reset(false);
    }

    if should_configure_swapchain {
        use gfx_hal::window::SwapchainConfig;

        let caps = res.surface.capabilities(&render_info.adapter.physical_device);

        let mut swapchain_config =
            SwapchainConfig::from_caps(&caps, render_info.surface_color_format, render_info.surface_extent);

        // This seems to fix some fullscreen slowdown on macOS.
        if caps.image_count.contains(&3) {
            swapchain_config.image_count = 3;
        }

        render_info.surface_extent = swapchain_config.extent;
        render_info.surface_extent = gfx_hal::window::Extent2D {
            width: render_info.surface_extent.width,
            height: render_info.surface_extent.height,
        };

        unsafe {
            res.surface
                .configure_swapchain(&res.device, swapchain_config)
                .expect("Failed to configure swapchain");
        };

        should_configure_swapchain = false;
    }

    let surface_image = unsafe {
        // We refuse to wait more than a second, to avoid hanging.
        let acquire_timeout_ns = 1_000_000_000;

        match res.surface.acquire_image(acquire_timeout_ns) {
            Ok((image, _)) => image,
            Err(_) => {
                should_configure_swapchain = true;
                return;
            }
        }
    };

    let temp_framebuffer = unsafe {
        use std::borrow::Borrow;

        use gfx_hal::image::Extent;

        res.device
            .create_framebuffer(
                render_pass,
                vec![temp_image_view],
                Extent {
                    width: render_info.render_resolution.0,
                    height: render_info.render_resolution.1,
                    depth: 1
                },
            )
            .unwrap()
    };

    let surface_framebuffer = unsafe {
        use std::borrow::Borrow;

        use gfx_hal::image::Extent;

        res.device
            .create_framebuffer(
                render_pass,
                vec![surface_image.borrow()],
                Extent {
                    width: render_info.surface_extent.width,
                    height: render_info.surface_extent.height,
                    depth: 1
                },
            )
            .unwrap()
    };

    let (low_res_viewport, full_viewport) = {
        use gfx_hal::pso::{Rect, Viewport};

        let low_res = Viewport {
            rect: Rect {
                x: 0,
                y: 0,
                w: render_info.render_resolution.0 as i16,
                h: render_info.render_resolution.1 as i16,
            },
            depth: 0.0..1.0,
        };

        let full = Viewport {
            rect: Rect {
                x: 0,
                y: 0,
                w: render_info.surface_extent.width as i16,
                h: render_info.surface_extent.height as i16,
            },
            depth: 0.0..1.0,
        };

        (low_res, full)
    };
    unsafe {
        use gfx_hal::pso::ShaderStageFlags;
        use super::camera_data_buffer::*;
        use gfx_hal::command::{
            ClearColor, ClearValue, CommandBuffer, CommandBufferFlags, SubpassContents,
        };
        use gfx_hal::image::{Layout, Filter};

        command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

        command_buffer.set_viewports(0, &[low_res_viewport.clone()]);
        command_buffer.set_scissors(0, &[low_res_viewport.rect]);

        command_buffer.bind_graphics_descriptor_sets(&temp_pipeline_layout, 0, Some(&res.description_sets[1]), &[]);


        command_buffer.update_buffer(
            world_buffer,
            0,
            world_data_buffer.bytes_8(),
        );

        command_buffer.begin_render_pass(
            render_pass,
            &temp_framebuffer,
            low_res_viewport.rect,
            &[ClearValue {
                color: ClearColor {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }],
            SubpassContents::Inline,
        );
        command_buffer.bind_graphics_pipeline(temp_pipeline);

        command_buffer.push_graphics_constants(
            temp_pipeline_layout,
            camera_data_buffer.shader_stage,
            0,
            camera_data_buffer.bytes(),
        );

        command_buffer.push_graphics_constants(
            temp_pipeline_layout,
            world_data_buffer.shader_stage,
            32,
            world_data_buffer.bytes(),
        );



        command_buffer.draw(0..6, 0..1);
        command_buffer.end_render_pass();

        command_buffer.bind_graphics_descriptor_sets(&surface_pipeline_layout, 0, Some(&res.description_sets[0]), &[]);

        command_buffer.set_viewports(0, &[full_viewport.clone()]);
        command_buffer.set_scissors(0, &[full_viewport.rect]);

        command_buffer.begin_render_pass(
            render_pass,
            &surface_framebuffer,
            full_viewport.rect,
            &[ClearValue {
                color: ClearColor {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }],
            SubpassContents::Inline,
        );
        command_buffer.bind_graphics_pipeline(surface_pipeline);

        command_buffer.draw(0..6, 0..1);
        command_buffer.end_render_pass();

        command_buffer.finish();
    }

    unsafe {
        use gfx_hal::queue::{CommandQueue, Submission};

        let submission = Submission {
            command_buffers: vec![&command_buffer],
            wait_semaphores: None,
            signal_semaphores: vec![&res.rendering_complete_semaphore],
        };

        render_info.queue_group.queues[0].submit(submission, Some(&res.submission_complete_fence));

        let result = render_info.queue_group.queues[0].present(
            &mut res.surface,
            surface_image,
            Some(&res.rendering_complete_semaphore),
        );

        should_configure_swapchain |= result.is_err();

        res.device.destroy_framebuffer(temp_framebuffer);
        res.device.destroy_framebuffer(surface_framebuffer);
    }
}
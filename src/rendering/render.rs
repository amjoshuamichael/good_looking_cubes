use std::borrow::Borrow;
use gfx_hal::command::{ClearColor, ClearValue, CommandBufferFlags, SubpassContents};
use gfx_hal::image::Extent;
use gfx_hal::prelude::*;
use gfx_hal::pso::{Rect, ShaderStageFlags, Viewport};
use gfx_hal::queue::Submission;
use time::Instant;

use crate::GPUData;
use crate::rendering::resources::RenderInfo;

pub struct RenderEvent {
    pub time: Instant,
}

pub fn render_draw<B: gfx_hal::Backend>(
    res: &mut RenderInfo<B>,
    command_buffer: &mut B::CommandBuffer,
    camera_data_buffer: &GPUData,
) -> Result<(), ()> {
    let render_pass = &res.render_passes[0];
    let temp_pipeline_layout = &res.pipeline_layouts[0];
    let temp_pipeline = &res.render_pipelines[0];
    let temp_image_view = &res.image_views[0];
    let surface_pipeline_layout = &res.pipeline_layouts[1];
    let surface_pipeline = &res.render_pipelines[1];

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

    let surface_image = unsafe {
        // We refuse to wait more than a second, to avoid hanging.
        let acquire_timeout_ns = 1_000_000_000;

        match res.surface.acquire_image(acquire_timeout_ns) {
            Ok((image, _)) => image,
            Err(_) => return Err(()),
        }
    };

    let (temp_framebuffer, surface_framebuffer) = unsafe {
        let temp = res.device
            .create_framebuffer(
                render_pass,
                vec![temp_image_view],
                Extent {
                    width: res.render_resolution.0,
                    height: res.render_resolution.1,
                    depth: 1
                },
            )
            .unwrap();

        let surface = res.device
            .create_framebuffer(
                render_pass,
                vec![surface_image.borrow()],
                Extent {
                    width: res.surface_extent.width,
                    height: res.surface_extent.height,
                    depth: 1
                },
            )
            .unwrap();

        (temp, surface)
    };

    let low_res_viewport = Viewport {
        rect: Rect {
            x: 0,
            y: 0,
            w: res.render_resolution.0 as i16,
            h: res.render_resolution.1 as i16,
        },
        depth: 0.0..1.0,
    };

    let full_viewport = Viewport {
        rect: Rect {
            x: 0,
            y: 0,
            w: res.surface_extent.width as i16,
            h: res.surface_extent.height as i16,
        },
        depth: 0.0..1.0,
    };

    unsafe {
        command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

        command_buffer.set_viewports(0, &[low_res_viewport.clone()]);
        command_buffer.set_scissors(0, &[low_res_viewport.rect]);

        command_buffer.bind_graphics_descriptor_sets(temp_pipeline_layout, 0, Some(&res.description_sets[1]), &[]);

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
            ShaderStageFlags::ALL,
            0,
            camera_data_buffer.bytes(),
        );

        command_buffer.draw(0..6, 0..1);
        command_buffer.end_render_pass();

        command_buffer.bind_graphics_descriptor_sets(surface_pipeline_layout, 0, Some(&res.description_sets[0]), &[]);
        command_buffer.bind_graphics_descriptor_sets(surface_pipeline_layout, 1, Some(&res.description_sets[2]), &[]);

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

    let submission = Submission {
        command_buffers: vec![&command_buffer],
        wait_semaphores: None,
        signal_semaphores: vec![&res.rendering_complete_semaphore],
    };

    unsafe {
        res.queue_group.queues[0].submit(submission, Some(&res.submission_complete_fence));

        let result = res.queue_group.queues[0].present(
            &mut res.surface,
            surface_image,
            Some(&res.rendering_complete_semaphore),
        );

        res.device.destroy_framebuffer(temp_framebuffer);
        res.device.destroy_framebuffer(surface_framebuffer);

        if result.is_err() { return Err(()) }
    }

    Ok(())
}
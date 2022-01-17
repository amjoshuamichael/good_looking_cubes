use bevy::app::Events;
use bevy::prelude::*;
use gfx_hal::command::Level;
use gfx_hal::device::Device;
use gfx_hal::format::ChannelType;
use gfx_hal::image::Kind;
use gfx_hal::pool::CommandPoolCreateFlags;
use gfx_hal::prelude::*;
use gfx_hal::pso::ShaderStageFlags;
use gfx_hal::window::Extent2D;
use gfx_hal::window::SwapchainConfig;
use time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

use camera_data_buffer::CameraData;
use constructs::color_format::*;
use constructs::create_buffer_bindings::*;
use constructs::create_image_bindings::*;
use constructs::image_view::*;
use constructs::memory::*;
use constructs::pipeline::*;
use constructs::render_pass::*;
use constructs::screen::*;
use data_buffer::DataBuffer;
use render::render_draw;
use render::RenderEvent;
use resources::RenderInfo;
use crate::world::VOXEL_COUNT;

pub mod bevy_to_winit;
pub mod camera_data_buffer;
pub mod render;
pub mod data_buffer;

pub mod shaders;
pub mod constructs;
pub mod resources;

#[derive(Default)]
pub struct CtklrRenderPlugin;

impl Plugin for CtklrRenderPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(|app| unsafe { create_window(app) });
    }
}

unsafe fn create_window(
    mut app: App,
) {
    const APP_NAME: &'static str = "gfx test";

    let camera_data_buffer = DataBuffer::<CameraData>::new(ShaderStageFlags::FRAGMENT);

    let event_loop = winit::event_loop::EventLoop::new();

    let (logical_window_size, physical_window_size) = get_sizes(&event_loop, [512, 512]);
    let surface_extent = Extent2D {
        width: physical_window_size.width,
        height: physical_window_size.height,
    };
    let window = winit::window::WindowBuilder::new()
        .with_title(APP_NAME)
        .with_inner_size(logical_window_size)
        .with_decorations(false)
        .build(&event_loop)
        .expect("Failed to create window");

    let (width, height) = (400, 400);

    let instance = backend::Instance::create("ctklr", 1).expect("Backend not supported");
    let surface = instance.create_surface(&window).expect("Failed to create window surface");
    let adapter = instance.enumerate_adapters().remove(0);
    let (device, queue_group) = device_info(&surface, &adapter);

    let surface_color_format =
        find_color_format(&surface, &adapter, |format| format.base_format().1 == ChannelType::Srgb);

    // The word "temp" is used to described the texture that the shaders render on to. This texture
    // is then upscaled and rendered to the main screen through the surface pipeline.
    let temp_image = create_image::<backend::Backend>(&device, Kind::D2(width, height, 1, 1), surface_color_format);
    let temp_image_view = create_image_view::<backend::Backend>(&device, &temp_image, surface_color_format);
    let (temp_set_layout, temp_description_set, temp_sampler) =
        create_image_bindings::<backend::Backend>(&device, &temp_image_view);

    let world_buffer = create_buffer::<backend::Backend>(&device, (VOXEL_COUNT * 4) as u64);
    let (world_set_layout, world_description_set) = create_buffer_bindings::<backend::Backend>(&device, &world_buffer);

    let mut command_pool = device
        .create_command_pool(queue_group.family, CommandPoolCreateFlags::empty())
        .expect("Out of memory");
    let mut command_buffer = command_pool.allocate_one(Level::Primary);

    let render_pass = create_render_pass::<backend::Backend>(&device, surface_color_format);

    let vertex_shader = shaders::VERTEX_CANVAS;
    let fragment_shader = shaders::VOXEL_RENDER;
    let post_processing_shader = shaders::POST_PROCESSING;

    let temp_pipeline_layout = device
        .create_pipeline_layout(&[world_set_layout], &[camera_data_buffer.layout()])
        .expect("Out of memory");
    let surface_pipeline_layout = device
        .create_pipeline_layout(&[temp_set_layout], &[])
        .expect("Out of memory");

    let temp_pipeline = make_pipeline::<backend::Backend>
        (&device, &render_pass, &temp_pipeline_layout, vertex_shader, fragment_shader);

    let surface_pipeline = make_pipeline::<backend::Backend>
        (&device, &render_pass, &surface_pipeline_layout, vertex_shader, post_processing_shader);

    let submission_complete_fence = device.create_fence(true).expect("Out of memory");
    let rendering_complete_semaphore = device.create_semaphore().expect("Out of memory");

    let global_command_buffer = command_pool.allocate_one(Level::Primary);

    let mut resources = RenderInfo {
        instance,
        surface,
        device,
        render_passes: vec![render_pass],
        pipeline_layouts: vec![temp_pipeline_layout, surface_pipeline_layout],
        render_pipelines: vec![temp_pipeline, surface_pipeline],
        compute_pipelines: vec![],
        image_views: vec![temp_image_view],
        description_sets: vec![temp_description_set, world_description_set],
        samplers: vec![temp_sampler],
        buffers: vec![world_buffer],
        buffer_views: vec![],
        command_pool,
        submission_complete_fence,
        rendering_complete_semaphore,
        surface_extent,
        adapter,
        queue_group,
        surface_color_format,
        render_resolution: (width, height),
    };

    configure_swapchain(&mut resources);

    app.insert_resource(camera_data_buffer);
    app.insert_resource(resources);
    app.insert_resource(global_command_buffer);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    let world = &mut app.world;
                    let resources = world.remove_resource::<RenderInfo<backend::Backend>>().unwrap();
                    resources.destroy_all();

                    *control_flow = ControlFlow::Exit
                },
                WindowEvent::KeyboardInput {ref input, ..} => {
                    let world = app.world.cell();
                    let mut keyboard_input_events =
                        world.get_resource_mut::<Events<bevy::input::keyboard::KeyboardInput>>().unwrap();
                    keyboard_input_events.send(bevy_to_winit::convert_keyboard_input(input));
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                app.update();
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                let world = app.world.cell();
                let camera_data_buffer = world.get_resource::<DataBuffer<CameraData>>().unwrap();

                let mut render_event_writer = world.get_resource_mut::<Events<RenderEvent>>().unwrap();
                render_event_writer.send(RenderEvent{time: Instant::now()});

                let mut resources = world.get_resource_mut::<RenderInfo<backend::Backend>>().unwrap();

                let render_result = render_draw::<backend::Backend>(&mut *resources,
                                                                    &mut command_buffer,
                                                                    &*camera_data_buffer,
                );

                match render_result {
                    Err(_) => configure_swapchain(&mut *resources),
                    _ => (),
                }
            },
            _ => (),
        }
    });
}

unsafe fn configure_swapchain<B: gfx_hal::Backend>(
    res: &mut RenderInfo<B>,
) {
    let caps = res.surface.capabilities(&res.adapter.physical_device);

    let mut swapchain_config =
        SwapchainConfig::from_caps(&caps, res.surface_color_format, res.surface_extent);

    // This seems to fix some fullscreen slowdown on macOS.
    if caps.image_count.contains(&3) {
        swapchain_config.image_count = 3;
    }

    res.surface_extent = swapchain_config.extent;
    res.surface_extent = Extent2D {
        width: res.surface_extent.width,
        height: res.surface_extent.height,
    };

    res.surface
        .configure_swapchain(&res.device, swapchain_config)
        .expect("Failed to configure swapchain");
}
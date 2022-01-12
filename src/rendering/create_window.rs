use bevy::prelude::*;
use std::mem::ManuallyDrop;
use bevy::app::Events;
use gfx_hal::prelude::*;
use gfx_hal::{
    device::Device,
    window::{Extent2D, PresentationSurface, Surface},
    Instance,
};
use gfx_hal::adapter::Adapter;
use gfx_hal::format::Format;
use gfx_hal::pso::{ImageDescriptorType, ShaderStageFlags};
use gfx_hal::queue::QueueGroup;
use shaderc::ShaderKind;
use winit::dpi::{LogicalSize, PhysicalSize};
use bytemuck::Pod;
use time::Instant;
use shaderc::{CompileOptions, OptimizationLevel};
use crate::rendering::bevy_to_winit;
use crate::rendering::render::RenderEvent;
use crate::world::world_data::WorldData;
use super::data_buffer::{DataBuffer, DataForBuffer};
use super::render::render_draw;
use super::camera_data_buffer::CameraData;
use super::shaders;

pub struct Resources<B: gfx_hal::Backend> {
    pub instance: B::Instance,
    pub surface: B::Surface,
    pub device: B::Device,
    pub render_passes: Vec<B::RenderPass>,
    pub pipeline_layouts: Vec<B::PipelineLayout>,
    pub pipelines: Vec<B::GraphicsPipeline>,
    pub command_pool: B::CommandPool,
    pub submission_complete_fence: B::Fence,
    pub rendering_complete_semaphore: B::Semaphore,
    pub image_view: B::ImageView,
    pub description_set: B::DescriptorSet,
    pub sampler: B::Sampler,
}

pub struct ResourceHolder<B: gfx_hal::Backend>(pub ManuallyDrop<Resources<B>>);

impl<B: gfx_hal::Backend> Drop for ResourceHolder<B> {
    fn drop(&mut self) {
        unsafe {
            let Resources {
                instance,
                mut surface,
                device,
                command_pool,
                render_passes,
                pipeline_layouts,
                pipelines,
                submission_complete_fence,
                rendering_complete_semaphore,
                image_view,
                description_set,
                sampler,
            } = ManuallyDrop::take(&mut self.0);

            device.destroy_semaphore(rendering_complete_semaphore);
            device.destroy_fence(submission_complete_fence);
            for pipeline in pipelines {
                device.destroy_graphics_pipeline(pipeline);
            }
            for pipeline_layout in pipeline_layouts {
                device.destroy_pipeline_layout(pipeline_layout);
            }
            for render_pass in render_passes {
                device.destroy_render_pass(render_pass);
            }
            device.destroy_command_pool(command_pool);
            device.destroy_sampler(sampler);
            surface.unconfigure_swapchain(&device);
            instance.destroy_surface(surface);
            //TODO: dispose of image_view memory
        }
    }
}

pub struct RenderInfo<B: gfx_hal::Backend> {
    pub surface_extent: Extent2D,
    pub adapter: Adapter<B>,
    pub queue_group: QueueGroup<B>,
    pub surface_color_format: Format,
    pub render_resolution: (u32, u32),
}

#[derive(Default)]
pub struct CtklrWindowPlugin;

impl Plugin for CtklrWindowPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(create_window);
    }
}

fn create_window(
    mut app: App,
) {
    const APP_NAME: &'static str = "gfx test";
    const WINDOW_SIZE: [u32; 2] = [512, 512];

    let camera_data_buffer = DataBuffer::<CameraData>::new(ShaderStageFlags::FRAGMENT);
    let world_data_buffer = DataBuffer::<WorldData>::new(ShaderStageFlags::FRAGMENT);

    let event_loop = winit::event_loop::EventLoop::new();

    let (logical_window_size, physical_window_size) = {
        let dpi = event_loop.primary_monitor().unwrap().scale_factor();
        let logical: LogicalSize<u32> = WINDOW_SIZE.into();
        let physical: PhysicalSize<u32> = logical.to_physical(dpi);

        (logical, physical)
    };

    let mut surface_extent = Extent2D {
        width: physical_window_size.width,
        height: physical_window_size.height,
    };

    let mut window = winit::window::WindowBuilder::new()
        .with_title(APP_NAME)
        .with_inner_size(logical_window_size)
        .with_decorations(false)
        .build(&event_loop)
        .expect("Failed to create window");

    let (instance, surface, adapter) = unsafe {
        let instance = backend::Instance::create(APP_NAME, 1).expect("Backend not supported");
        let surface = instance.create_surface(&window).expect("Failed to create window surface");
        let adapter = instance.enumerate_adapters().remove(0);

        (instance, surface, adapter)
    };

    let (device, mut queue_group) = unsafe {
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
    };

    let (command_pool, mut command_buffer) = unsafe {
        use gfx_hal::command::Level;
        use gfx_hal::pool::CommandPoolCreateFlags;

        let mut command_pool = device
            .create_command_pool(queue_group.family, CommandPoolCreateFlags::empty())
            .expect("Out of memory");

        let command_buffer = command_pool.allocate_one(Level::Primary);

        (command_pool, command_buffer)
    };

    let surface_color_format = {
        use gfx_hal::format::ChannelType;

        let supported_formats = surface
            .supported_formats(&adapter.physical_device)
            .unwrap_or(vec![]);

        let default_format = *supported_formats.get(0).unwrap_or(&Format::Rgba8Srgb);

        supported_formats.into_iter()
            .find(|format| format.base_format().1 == ChannelType::Srgb)
            .unwrap_or(default_format)
    };

    let render_pass = unsafe {
        use gfx_hal::image::{
            Layout, Access
        };
        use gfx_hal::pass::{
            Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDesc, SubpassId, SubpassDependency
        };
        use gfx_hal::pso::PipelineStage;
        use gfx_hal::memory::Dependencies;

        device
            .create_render_pass(&[
                Attachment {
                    format: Some(surface_color_format),
                    samples: 1,
                    ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
                    stencil_ops: AttachmentOps::DONT_CARE,
                    layouts: Layout::General..Layout::General,
                }
            ], &[
                SubpassDesc {
                    colors: &[(0, Layout::ColorAttachmentOptimal)],
                    depth_stencil: None,
                    inputs: &[],
                    resolves: &[],
                    preserves: &[],
                }
            ], &[
                SubpassDependency {
                    passes: Some(0)..Some(1),
                    stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
                    accesses: Access::empty()
                        ..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
                    flags: Dependencies::BY_REGION,
                }
            ])
            .expect("Out of memory")
    };

    let (width, height) = (400, 400);

    let (image_view) = unsafe {
        use gfx_hal::image::{Kind, Tiling, Usage, ViewCapabilities, ViewKind, SubresourceRange, Size};
        use gfx_hal::format::Swizzle;
        use gfx_hal::MemoryTypeId;
        let mut image = device
            .create_image(
                Kind::D2(width, height, 1, 1),
                1,
                surface_color_format,
                Tiling::Linear,
                Usage::all(),
                ViewCapabilities::empty(),
            )
            .unwrap();

        let memory = device
            .allocate_memory(
                MemoryTypeId(0),
                device.get_image_requirements(&image).size,
            )
            .unwrap();

        device
            .bind_image_memory(&memory, 0, &mut image)
            .expect("failed to allocate image memory");

        let view = device
            .create_image_view(
                &image,
                ViewKind::D2,
                surface_color_format,
                Swizzle::default(),
                SubresourceRange::default(),
            )
            .expect("failed to link image to shader via image view");

        view
    };

    let vertex_shader = shaders::VERTEX_CANVAS;
    let fragment_shader = shaders::VOXEL_RENDER;
    let post_processing_shader = shaders::POST_PROCESSING;

    let (set_layout, description_set, sampler) = unsafe {
        use gfx_hal::pso;
        use gfx_hal::image;

        let set_layout =
            device.create_descriptor_set_layout(
                &[
                    pso::DescriptorSetLayoutBinding {
                        binding: 0,
                        ty: pso::DescriptorType::Image { ty: ImageDescriptorType::Sampled { with_sampler: true } },
                        count: 1,
                        stage_flags: ShaderStageFlags::FRAGMENT,
                        immutable_samplers: false,
                    },
                    pso::DescriptorSetLayoutBinding {
                        binding: 1,
                        ty: pso::DescriptorType::Sampler,
                        count: 1,
                        stage_flags: ShaderStageFlags::FRAGMENT,
                        immutable_samplers: false,
                    },
                ],
                &[],
            )
            .expect("Can't create descriptor set layout");

        let mut desc_pool = device.create_descriptor_pool(
                1, // sets
                &[
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::Image { ty: ImageDescriptorType::Sampled { with_sampler: true } },
                        count: 1,
                    },
                    pso::DescriptorRangeDesc {
                        ty: pso::DescriptorType::Sampler,
                        count: 1,
                    },
                ],
                gfx_hal::pso::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET,
            )
            .expect("Can't create descriptor pool");

        let desc_set = desc_pool.allocate_set(&set_layout).unwrap();

        let sampler =
            device
                .create_sampler(&image::SamplerDesc::new(image::Filter::Nearest, image::WrapMode::Clamp))
                .unwrap();

        println!("we are about to write descriptor sets...");
        device.write_descriptor_sets(vec![
            pso::DescriptorSetWrite {
                set: &desc_set,
                binding: 0,
                array_offset: 0,
                descriptors: Some(pso::Descriptor::Image(&image_view, image::Layout::General)),
            },
            pso::DescriptorSetWrite {
                set: &desc_set,
                binding: 1,
                array_offset: 0,
                descriptors: Some(pso::Descriptor::Sampler(&sampler)),
            },
        ]);
        println!("we have written the descriptor sets!");

        (set_layout, desc_set, sampler)
    };

    let (temp_pipeline_layout, surface_pipeline_layout) = unsafe {
        use gfx_hal::pso::ShaderStageFlags;

        let temp = device
            .create_pipeline_layout(&[], &[
                camera_data_buffer.layout(),
                world_data_buffer.layout(),
            ])
            .expect("Out of memory");

        let surface = device
            .create_pipeline_layout(&[
                set_layout,
            ], &[])
            .expect("Out of memory");

        (temp, surface)
    };

    let mut should_configure_swapchain = true;

    let temp_pipeline = unsafe {
        make_pipeline::<backend::Backend>(
            &device,
            &render_pass,
            &temp_pipeline_layout,
            vertex_shader,
            fragment_shader,
        )
    };

    let surface_pipeline = unsafe {
        make_pipeline::<backend::Backend>(
            &device,
            &render_pass,
            &surface_pipeline_layout,
            vertex_shader,
            post_processing_shader,
        )
    };

    let submission_complete_fence = device.create_fence(true).expect("Out of memory");
    let rendering_complete_semaphore = device.create_semaphore().expect("Out of memory");

    let mut resource_holder = ResourceHolder(ManuallyDrop::new(Resources {
            instance,
            surface,
            device,
            command_pool,
            render_passes: vec![render_pass],
            pipeline_layouts: vec![temp_pipeline_layout, surface_pipeline_layout],
            pipelines: vec![temp_pipeline, surface_pipeline],
            submission_complete_fence,
            rendering_complete_semaphore,
            image_view,
            description_set,
            sampler,
        }));

    let mut render_info = RenderInfo {
        surface_extent,
        adapter,
        queue_group,
        surface_color_format,
        render_resolution: (width, height),
    };

    app.insert_resource(camera_data_buffer);
    app.insert_resource(world_data_buffer);

    println!("about to enter event loop");
    // Note that this takes a `move` closure. This means it will take ownership
    // over any resources referenced within. It also means they will be dropped
    // only when the application is quit.
    event_loop.run(move |event, _, control_flow| {
        use winit::event::{Event, WindowEvent};
        use winit::event_loop::ControlFlow;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                // WindowEvent::Resized(new_dimensions) => {
                //     surface_extent = Extent2D {
                //         width: new_dimensions.width / 8,
                //         height: new_dimensions.height / 8,
                //     };
                //     should_configure_swapchain = true;
                //     println!("resized! new dimensions are:{},{}", surface_extent.width, surface_extent.height);
                // }
                // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                //     surface_extent = Extent2D {
                //         width: new_inner_size.width / 8,
                //         height: new_inner_size.height / 8,
                //     };
                //     should_configure_swapchain = true;
                //     println!("scale factor changed");
                // },
                WindowEvent::KeyboardInput {ref input, ..} => {
                    let world = app.world.cell();
                    let mut keyboard_input_events =
                        world.get_resource_mut::<Events<bevy::input::keyboard::KeyboardInput>>().unwrap();
                    keyboard_input_events.send(bevy_to_winit::convert_keyboard_input(input));
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                app.update();
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                let world = app.world.cell();
                let camera_data_buffer = world.get_resource::<DataBuffer<CameraData>>().unwrap();
                let world_data_buffer = world.get_resource::<DataBuffer<WorldData>>().unwrap();
                let mut render_event_writer = world.get_resource_mut::<Events<RenderEvent>>().unwrap();

                render_event_writer.send(RenderEvent{time: Instant::now()});

                render_draw(&mut render_info,
                            &mut resource_holder,
                            &mut command_buffer,
                            &*camera_data_buffer,
                            &*world_data_buffer,
                            should_configure_swapchain,
                );
            },
            _ => (),
        }
    });
}

/// Create a pipeline with the given layout and shaders.
unsafe fn make_pipeline<B: gfx_hal::Backend>(
    device: &B::Device,
    render_pass: &B::RenderPass,
    pipeline_layout: &B::PipelineLayout,
    vertex_shader: &str,
    fragment_shader: &str,
) -> B::GraphicsPipeline {
    use gfx_hal::pass::Subpass;
    use gfx_hal::pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, Face, GraphicsPipelineDesc,
        InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, Specialization,
    };

    let vertex_shader_module = device
        .create_shader_module(&compile_shader(vertex_shader, ShaderKind::Vertex))
        .expect("Failed to create vertex shader module");

    let fragment_shader_module = device
        .create_shader_module(&compile_shader(fragment_shader, ShaderKind::Fragment))
        .expect("Failed to create fragment shader module");

    let (vs_entry, fs_entry) = (
        EntryPoint {
            entry: "main",
            module: &vertex_shader_module,
            specialization: Specialization::default(),
        },
        EntryPoint {
            entry: "main",
            module: &fragment_shader_module,
            specialization: Specialization::default(),
        },
    );

    let primitive_assembler = PrimitiveAssemblerDesc::Vertex {
        buffers: &[],
        attributes: &[],
        input_assembler: InputAssemblerDesc::new(Primitive::TriangleList),
        vertex: vs_entry,
        tessellation: None,
        geometry: None,
    };

    let mut pipeline_desc = GraphicsPipelineDesc::new(
        primitive_assembler,
        Rasterizer {
            cull_face: Face::BACK,
            ..Rasterizer::FILL
        },
        Some(fs_entry),
        pipeline_layout,
        Subpass {
            index: 0,
            main_pass: render_pass,
        },
    );

    pipeline_desc.blender.targets.push(ColorBlendDesc {
        mask: ColorMask::ALL,
        blend: Some(BlendState::ALPHA),
    });

    let pipeline = device
        .create_graphics_pipeline(&pipeline_desc, None)
        .expect("Failed to create graphics pipeline");

    device.destroy_shader_module(vertex_shader_module);
    device.destroy_shader_module(fragment_shader_module);

    pipeline
}

fn compile_shader(glsl: &str, shader_kind: ShaderKind) -> Vec<u32> {
    let mut compiler = shaderc::Compiler::new().unwrap();

    let mut compiler_options = shaderc::CompileOptions::new().unwrap();
    compiler_options.set_optimization_level(OptimizationLevel::Performance);

    let compiled_shader = compiler
        .compile_into_spirv(glsl, shader_kind, "unnamed", "main", Some(&compiler_options))
        .expect("Failed to compile shader");

    compiled_shader.as_binary().to_vec()
}
use bevy::ecs::world::WorldBorrow;
use winit::window::Window;
use super::shaders;
use super::camera_data_buffer::CameraData;
use super::data_buffer::*;
use crate::world::world_data::WorldData;

pub struct RenderInfo {
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl RenderInfo {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self {
            size,
            surface,
            device,
            queue,
            config,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

pub struct Renderer {
    pub pipeline: wgpu::RenderPipeline,
    pub is_rendering: bool,
}

impl Renderer {
    pub fn new(
        render_info: &RenderInfo,
        camera_data_buffer: &DataBuffer::<CameraData>,
        world_data_buffer: &DataBuffer::<WorldData>,
    ) -> Self {
        let vertex_canvas_shader = render_info.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Canvas Shader"),
            source: wgpu::ShaderSource::Wgsl(shaders::VERTEX_CANVAS.into()),
        });

        let voxel_render_shader = render_info.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Voxel Render Shader"),
            source: wgpu::ShaderSource::Wgsl(shaders::VOXEL_RENDER.into()),
        });

        let pipeline_layout = render_info.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &camera_data_buffer.bind_group_layout,
                &world_data_buffer.bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let pipeline = render_info.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_canvas_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &voxel_render_shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: render_info.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            pipeline,
            is_rendering: false,
        }
    }
}


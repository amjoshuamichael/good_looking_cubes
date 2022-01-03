use bevy::prelude::*;
use bevy::reflect::DynamicMap;
use bevy::ecs::world::WorldBorrow;
use winit::window::Window;
use wgpu::util::DeviceExt;
use bytemuck::Pod;
use super::camera_data_buffer::CameraData;

pub struct RenderInfo {
    pub size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
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
    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new(render_info: &RenderInfo, camera_data_buffer: &DataBuffer::<CameraData>) -> Self {
        let vertex_canvas_shader = render_info.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Vertex Canvas Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/vertex_canvas.wgsl").into()),
        });

        let voxel_render_shader = render_info.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Voxel Render Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/voxel_render.wgsl").into()),
        });

        let pipeline_layout = render_info.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &camera_data_buffer.bind_group_layout
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
        }
    }
}

/// Data that is shared between both the CPU and GPU.
pub struct DataBuffer<T> {
    pub data: T,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

pub trait DataForBuffer {
    fn create() -> Self;
}

impl<T: DataForBuffer + Pod> DataBuffer<T> {
    pub fn new(render_info: &RenderInfo) -> Self {
        let mut data = T::create();

        let buffer = render_info.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Buffer"),
                contents: bytemuck::cast_slice(&[data]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = render_info.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("Bind Group Layout"),
        });

        let bind_group = render_info.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("Bind Group"),
        });

        DataBuffer::<T> { data, buffer, bind_group_layout, bind_group }
    }

    pub fn write_buffer(render_info: &RenderInfo) {
        render_info.queue.write_buffer(
            &Self.buffer,
            0,
            bytemuck::cast_slice(&[Self.data]),
        );
    }
}

pub fn render(
    render_info: &mut RenderInfo,
    renderer: &Renderer,
    camera_data_buffer: WorldBorrow<DataBuffer<CameraData>>,
    
) -> Result<(), wgpu::SurfaceError> {
    let output = render_info.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = render_info.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 1.0,
                    a: 0.5,
                }),
                store: true,
            },
        }],
        depth_stencil_attachment: None,
    });

    render_pass.set_pipeline(&renderer.pipeline);


    render_pass.set_bind_group(0, &camera_data_buffer.bind_group, &[]);

    render_pass.draw(0..6, 0..1);

    drop(render_pass);

    // submit will accept anything that implements IntoIter
    render_info.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}
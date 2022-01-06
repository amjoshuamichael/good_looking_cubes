use bytemuck::*;
use super::render_system::RenderInfo;
use wgpu::util::DeviceExt;

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
    pub fn new(render_info: &RenderInfo, label: &str) -> Self {
        let mut data = T::create();

        let buffer = render_info.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{} {}", label, "Buffer")),
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
            label: Some(&format!("{} {}", label, "Bind Group Layout")),
        });

        let bind_group = render_info.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some(&format!("{} {}", label, "Bind Group")),
        });

        DataBuffer::<T> { data, buffer, bind_group_layout, bind_group }
    }

    pub fn write_buffer(&self, render_info: &RenderInfo) {
        render_info.queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&[self.data]),
        );
    }
}
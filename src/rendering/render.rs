use bevy::prelude::*;
use bevy::ecs::world::WorldBorrow;
use super::data_buffer::DataBuffer;
use crate::world::world_data::WorldData;
use super::render_system::{RenderInfo, Renderer};
use super::camera_data_buffer::CameraData;

pub fn render(
    mut render_info: ResMut<RenderInfo>,
    mut renderer: ResMut<Renderer>,
    camera_data_buffer: Res<DataBuffer<CameraData>>,
    world_data_buffer: Res<DataBuffer<WorldData>>,
) {
    if !renderer.is_rendering {
        renderer.is_rendering = true;

        //TODO: handle wgpu::SurfaceError:Lost and wgpu::SurfaceError::OutOfMemory
        #[allow(unused_must_use)]
            _render(&mut *render_info, &mut *renderer, &*camera_data_buffer, &*world_data_buffer);
    }
}

fn _render(
    render_info: &mut RenderInfo,
    renderer: &mut Renderer,
    camera_data_buffer: &DataBuffer<CameraData>,
    world_data_buffer: &DataBuffer<WorldData>,
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

    camera_data_buffer.write_buffer(&render_info);
    render_pass.set_bind_group(0, &camera_data_buffer.bind_group, &[]);
    world_data_buffer.write_buffer(&render_info);
    render_pass.set_bind_group(1, &world_data_buffer.bind_group, &[]);

    render_pass.draw(0..6, 0..1);

    drop(render_pass);

    renderer.is_rendering = false;

    // submit will accept anything that implements IntoIter
    render_info.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use bevy::prelude::*;
use bevy::ecs::event::*;
use crate::CameraData;
use super::render_system::DataBuffer;
use super::render_system::RenderInfo;
use super::render_system::Renderer;
use super::render_system::render;
use super::bevy_to_winit;

#[derive(Default)]
pub struct CtklrWindowPlugin;

impl Plugin for CtklrWindowPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(create_window_with);
    }
}

fn create_window_with(mut app: App) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut render_info = pollster::block_on(RenderInfo::new(&window));

    let mut camera_data_buffer = DataBuffer::<CameraData>::new(&render_info);

    let renderer = Renderer::new(&render_info, &camera_data_buffer);

    app.insert_resource::<DataBuffer::<CameraData>>(camera_data_buffer);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id, } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    },
                    WindowEvent::Resized(physical_size) => {
                        render_info.resize(*physical_size)
                    },
                    WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {
                        render_info.resize(**new_inner_size)
                    },
                    WindowEvent::KeyboardInput { ref input, .. } => {
                        let world = app.world.cell();
                        let mut keyboard_input_events =
                            world.get_resource_mut::<Events<bevy::input::keyboard::KeyboardInput>>().unwrap();
                        keyboard_input_events.send(bevy_to_winit::convert_keyboard_input(input));
                    }
                    _ => {}
                }
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let world = app.world.cell();
                let camera_data_buffer = world.get_resource::<DataBuffer::<CameraData>>().unwrap();

                match render(&mut render_info, &renderer, camera_data_buffer) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => render_info.resize(render_info.size),
                    // The system is out of memory, quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                app.update();
                window.request_redraw();
            }
            _ => {}
        }
    });
}
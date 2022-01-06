use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use bevy::prelude::*;
use bevy::ecs::event::*;
use super::camera_data_buffer::CameraData;
use super::render_system::*;
use super::data_buffer::*;
use crate::world::world_data::WorldData;
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

    let mut camera_data_buffer = DataBuffer::<CameraData>::new(&render_info, "camera");
    let mut world_data_buffer = DataBuffer::<WorldData>::new(&render_info, "world");

    let renderer = Renderer::new(&render_info, &camera_data_buffer, &world_data_buffer);

    app.insert_resource::<DataBuffer<CameraData>>(camera_data_buffer);
    app.insert_resource::<DataBuffer<WorldData>>(world_data_buffer);
    app.insert_resource::<Renderer>(renderer);
    app.insert_resource::<RenderInfo>(render_info);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { ref event, window_id, } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    },

                    WindowEvent::Resized(physical_size) => {
                        let world = app.world.cell();
                        let mut render_info = world.get_resource_mut::<RenderInfo>().unwrap();
                        render_info.resize(*physical_size);
                    },

                    WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {
                        let world = app.world.cell();
                        let mut render_info = world.get_resource_mut::<RenderInfo>().unwrap();
                        render_info.resize(**new_inner_size);
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

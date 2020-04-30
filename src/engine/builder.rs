use std::collections::HashMap;

use crate::point::PointFormat;
use crate::window::{Window, WindowBuilder};

use futures::executor::block_on;
use nalgebra::Point3;
use slam_cv::Number;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window,
};

pub struct EngineBuilder<N>
where
    N: 'static + Number,
    Point3<N>: PointFormat<N>,
{
    pub windows: Vec<WindowBuilder<N>>,
}

impl<N> EngineBuilder<N>
where
    N: 'static + Number,
    Point3<N>: PointFormat<N>,
{
    pub fn run(self) -> ! {
        let event_loop = EventLoop::new();

        let windows = self
            .windows
            .into_iter()
            .map(|builder| block_on(builder.build(&event_loop)))
            .collect::<HashMap<_, _>>();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } => {
                    if let Some(window) = windows.get(&window_id) {
                        if !window.input(event) {
                            match event {
                                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                                WindowEvent::KeyboardInput { input, .. } => match input {
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    } => *control_flow = ControlFlow::Exit,
                                    _ => {}
                                },
                                WindowEvent::Resized(physical_size) => {
                                    window.resize(*physical_size);
                                }
                                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                    // new_inner_size is &mut so w have to dereference it twice
                                    window.resize(**new_inner_size);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Event::RedrawRequested(window_id) => {
                    if let Some(window) = windows.get(&window_id) {
                        window.update();
                        window.render();
                    }
                }
                Event::MainEventsCleared => {
                    for engine_window in windows.values() {
                        engine_window.window.request_redraw();
                    }
                }
                _ => {}
            }
        })
    }
}

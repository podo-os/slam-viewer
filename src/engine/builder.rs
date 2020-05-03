use std::collections::HashMap;

use super::base::Engine;
use super::timer::Timer;
use crate::pipes::{PipelineBuilder, VertexFormat};
use crate::window::{WindowBuilder, WindowEventState};

#[cfg(not(target_arch = "wasm32"))]
use futures::executor::block_on;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local as block_on;

use nalgebra::Point3;
use slam_cv::Number;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

pub struct EngineBuilder<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub windows: Vec<(WindowBuilder<N>, Box<dyn PipelineBuilder + Send>)>,
}

impl<N> EngineBuilder<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub fn run(self) {
        let event_loop = EventLoop::new();
        block_on(self.run_forever(event_loop))
    }

    pub fn spawn(self) -> Engine {
        Engine::new(move || {
            let event_loop = super::event_loop::new_event_loop();
            block_on(self.run_forever(event_loop))
        })
    }

    async fn run_forever(self, event_loop: EventLoop<()>) {
        let mut windows = HashMap::new();
        for (builder, pipe) in self.windows {
            let (id, window) = builder.build(&event_loop, pipe).await;
            windows.insert(id, window);
        }

        let mut timer = windows
            .values()
            .filter_map(|w| w.framerate)
            .min()
            .map(|f| {
                let micros = (1_000_000.0 / f as f64) as u64 - 500;
                std::time::Duration::from_micros(micros)
            })
            .and_then(Timer::try_new);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } => {
                    if let Some(window) = windows.get_mut(&window_id) {
                        if window.input(event) == WindowEventState::Unused {
                            match event {
                                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                                WindowEvent::KeyboardInput { input, .. } => {
                                    if let KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    } = input
                                    {
                                        *control_flow = ControlFlow::Exit
                                    }
                                }
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
                    if let Some(window) = windows.get_mut(&window_id) {
                        window.update();
                        window.render();
                    }
                }
                Event::MainEventsCleared => {
                    if let Some(timer) = &mut timer {
                        timer.sync();
                    }

                    for engine_window in windows.values() {
                        engine_window.request_redraw();
                    }
                }
                _ => {}
            }
        })
    }
}

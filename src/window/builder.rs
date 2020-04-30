use super::base::Window;
use super::camera::{Camera, CameraController};
use crate::pipes::{PipelineBuilder, VertexFormat};

use nalgebra::Point3;
use slam_cv::Number;
use winit::{event_loop::EventLoop, window};

pub struct WindowBuilder<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub camera: Camera<N>,
    pub camera_controller: CameraController<N>,

    pub pipeline_builder: Box<dyn PipelineBuilder>,
}

impl<N> WindowBuilder<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub async fn build<T>(self, event_loop: &EventLoop<T>) -> (window::WindowId, Window<N>) {
        let window = window::WindowBuilder::new().build(&event_loop).unwrap();
        let id = window.id();

        let engine_window = Window::new(window, self).await;
        (id, engine_window)
    }
}

use super::base::Window;
use super::camera::{CameraControllerConfig, CameraFrustum};
use crate::pipes::{PipelineBuilder, VertexFormat};

use nalgebra::Point3;
use slam_cv::Number;
use winit::{event_loop::EventLoop, window};

pub struct WindowBuilder<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub title: Option<String>,
    pub framerate: Option<u64>,

    pub camera: CameraFrustum<N>,
    pub camera_controller: CameraControllerConfig<N>,
}

pub trait WindowBuilderDefault<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn default_window() -> WindowBuilder<N>;
}

impl<N> WindowBuilder<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub async fn build<T>(
        self,
        event_loop: &EventLoop<T>,
        pipeline_builder: Box<dyn PipelineBuilder>,
    ) -> (window::WindowId, Window<N>) {
        let window = window::WindowBuilder::new().build(&event_loop).unwrap();
        let id = window.id();

        if let Some(title) = &self.title {
            window.set_title(title);
        }

        let engine_window = Window::new(window, self, pipeline_builder).await;
        (id, engine_window)
    }
}

use super::renderer::PipelineRenderer;
use super::vertex::VertexFormat;
use crate::window::WindowBuilder;

use nalgebra::Point3;
use slam_cv::Number;

pub trait PipelineBuilder<N>
where
    Self: Send,
    N: 'static + Number,
{
    fn build(
        self: Box<Self>,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Box<dyn PipelineRenderer>;
}

pub trait PipelineDataBuilder<N>
where
    Self: Send,
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    type Builder: 'static + PipelineBuilder<N> + Send;

    fn default_window(&self) -> WindowBuilder<N>;

    fn build_data(self) -> Self::Builder;
}

use core::marker::PhantomData;

use super::builder::PointsBuilder;
use super::point::Point;
use super::source::PointSource;
use crate::pipes::{GpuVec, PipelineBuilder, PipelineRenderer, VertexFormat};

use nalgebra::Point3;
use slam_cv::Number;

pub struct PointsRendener<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: 'static + PointSource<N>,
{
    pub render_pipeline: wgpu::RenderPipeline,
    pub buffer: GpuVec<Point<N>>,

    pub number: PhantomData<N>,
    pub source: S,
}

impl<N, S> PipelineBuilder for PointsBuilder<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: 'static + PointSource<N>,
{
    fn build(
        self: Box<Self>,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Box<dyn PipelineRenderer> {
        Box::new((*self).build(device, texture_format, uniform_bind_group_layout))
    }
}

impl<N, S> PipelineRenderer for PointsRendener<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: PointSource<N>,
{
    fn render<'a>(&'a mut self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        let color = S::COLOR.into();

        let points = self
            .source
            .collect_visual_points()
            .into_iter()
            .map(|p| Point { position: p, color })
            .collect();

        self.buffer.update(device, points);
        self.buffer.set_buffer(render_pass);
    }
}

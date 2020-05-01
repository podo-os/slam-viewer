use core::marker::PhantomData;

use super::builder::PointsBuilder;
use super::source::PointSource;
use crate::pipes::{PipelineBuilder, PipelineRenderer, VertexFormat};

use nalgebra::Point3;
use slam_cv::Number;

pub struct PointsRendener<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: 'static + PointSource<N>,
{
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,

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
        let points = self.source.collect_visual_points();
        let num_points = points.len() as u32;

        // TODO: more efficient buffer
        self.vertex_buffer = device
            .create_buffer_with_data(bytemuck::cast_slice(&points), wgpu::BufferUsage::VERTEX);

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        render_pass.draw(0..num_points, 0..1);
    }
}

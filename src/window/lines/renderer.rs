use core::marker::PhantomData;

use super::super::points::Point;
use super::builder::LinesBuilder;
use super::line::Line;
use super::source::LineSource;
use crate::pipes::{GpuVec, PipelineBuilder, PipelineRenderer, VertexFormat};

use nalgebra::Point3;
use slam_cv::Number;

pub struct LinesRendener<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: 'static + LineSource<N>,
{
    pub render_pipeline: wgpu::RenderPipeline,
    pub buffer: GpuVec<Line<N>>,

    pub number: PhantomData<N>,
    pub source: S,
}

impl<N, S> PipelineBuilder<N> for LinesBuilder<N, S>
where
    Self: Send,
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: 'static + LineSource<N>,
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

impl<N, S> PipelineRenderer for LinesRendener<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: LineSource<N>,
{
    fn render<'a>(&'a mut self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        let color = S::COLOR.into();

        let lines = self
            .source
            .collect_visual_lines()
            .into_iter()
            .map(|[p1, p2]| Line {
                start: Point {
                    position: p1,
                    color,
                },
                end: Point {
                    position: p2,
                    color,
                },
            })
            .collect();

        self.buffer.update(device, lines);
        self.buffer.set_buffer(render_pass);
    }
}

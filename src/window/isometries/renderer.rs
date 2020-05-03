use core::marker::PhantomData;

use super::builder::IsometriesBuilder;
use super::isometry::Isometry;
use super::source::IsometrySource;
use crate::pipes::{GpuVec, PipelineBuilder, PipelineRenderer, VertexFormat};

use nalgebra::Point3;
use slam_cv::Number;

pub struct IsometriesRendener<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: 'static + IsometrySource<N>,
{
    pub render_pipeline: wgpu::RenderPipeline,
    pub buffer: GpuVec<Isometry<N>>,

    pub number: PhantomData<N>,
    pub source: S,
}

impl<N, S> PipelineBuilder for IsometriesBuilder<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: 'static + IsometrySource<N>,
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

impl<N, S> PipelineRenderer for IsometriesRendener<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: IsometrySource<N>,
{
    fn render<'a>(&'a mut self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        let size = S::SIZE.into();
        let color = S::COLOR.into();

        let isometries = self
            .source
            .collect_visual_isometries()
            .into_iter()
            .map(|i| Isometry::from_iso(i, size, color))
            .collect();

        self.buffer.update(device, isometries);
        self.buffer.set_buffer(render_pass);
    }
}

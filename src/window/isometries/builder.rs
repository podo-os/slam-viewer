use core::marker::PhantomData;

use super::super::lines::build_render_pipeline;
use super::renderer::IsometriesRendener;
use super::source::IsometrySource;
use crate::pipes::{GpuVec, VertexFormat};

use nalgebra::Point3;
use slam_cv::Number;

pub struct IsometriesBuilder<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: IsometrySource<N>,
{
    pub source: S,

    number: PhantomData<N>,
}

impl<N, S> IsometriesBuilder<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: IsometrySource<N>,
{
    pub fn new(source: S) -> Self {
        Self {
            source,

            number: Default::default(),
        }
    }

    pub fn build(
        self,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> IsometriesRendener<N, S> {
        let render_pipeline =
            build_render_pipeline(device, texture_format, uniform_bind_group_layout);

        IsometriesRendener {
            render_pipeline,
            buffer: GpuVec::new(wgpu::BufferUsage::VERTEX),

            number: Default::default(),
            source: self.source,
        }
    }
}

use super::super::points::Point;
use crate::pipes::{GpuVertex, VertexFormat};

use nalgebra::Point3;
use slam_cv::Number;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Line<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub start: Point<N>,
    pub end: Point<N>,
}

type Attributes = [wgpu::VertexAttributeDescriptor; 2];

impl<N> Line<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub fn attributes() -> Attributes {
        Point::attributes()
    }

    pub fn desc(attributes: &Attributes) -> wgpu::VertexBufferDescriptor<'_> {
        Point::desc(attributes)
    }
}

impl<N> GpuVertex for Line<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn weight() -> u64 {
        2
    }
}

unsafe impl<N> bytemuck::Pod for Line<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
}
unsafe impl<N> bytemuck::Zeroable for Line<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
}

use core::mem;

use nalgebra::Point3;
use slam_cv::Number;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Point<N>
where
    N: 'static + Number,
    Point3<N>: PointFormat<N>,
{
    position: Point3<N>,
}

impl<N> Point<N>
where
    N: 'static + Number,
    Point3<N>: PointFormat<N>,
{
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttributeDescriptor {
                offset: 0,
                shader_location: 0,
                format: Point3::<N>::format(),
            }],
        }
    }
}

pub trait PointFormat<N>
where
    N: Number,
{
    fn format() -> wgpu::VertexFormat;
}

impl PointFormat<f32> for Point3<f32> {
    fn format() -> wgpu::VertexFormat {
        wgpu::VertexFormat::Float3
    }
}

unsafe impl<N> bytemuck::Pod for Point<N>
where
    N: 'static + Number,
    Point3<N>: PointFormat<N>,
{
}
unsafe impl<N> bytemuck::Zeroable for Point<N>
where
    N: 'static + Number,
    Point3<N>: PointFormat<N>,
{
}

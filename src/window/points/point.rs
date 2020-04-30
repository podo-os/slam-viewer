use core::mem;

use crate::pipes::VertexFormat;

use nalgebra::Point3;
use slam_cv::Number;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Point<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub position: Point3<N>,
    pub color: Point3<f32>,
}

type Attributes = [wgpu::VertexAttributeDescriptor; 2];

impl<N> Point<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub fn attributes() -> Attributes {
        [
            wgpu::VertexAttributeDescriptor {
                offset: 0,
                shader_location: 0,
                format: Point3::<N>::format(),
            },
            wgpu::VertexAttributeDescriptor {
                offset: mem::size_of::<Point3<N>>() as wgpu::BufferAddress,
                shader_location: 1,
                format: Point3::<f32>::format(),
            },
        ]
    }

    pub fn desc<'a>(attributes: &'a Attributes) -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes,
        }
    }
}

unsafe impl<N> bytemuck::Pod for Point<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
}
unsafe impl<N> bytemuck::Zeroable for Point<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
}

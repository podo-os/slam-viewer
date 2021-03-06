use core::mem;

use crate::pipes::{GpuVertex, VertexFormat};

use nalgebra::Point3;
use slam_cv::{Colors, Number};

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

impl<N> Default for Point<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn default() -> Self {
        Self {
            position: Point3::new(N::zero(), N::zero(), N::zero()),
            color: Colors::red().into(),
        }
    }
}

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

    pub fn desc(attributes: &Attributes) -> wgpu::VertexBufferDescriptor<'_> {
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes,
        }
    }
}

impl<N> GpuVertex for Point<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn weight() -> u64 {
        1
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

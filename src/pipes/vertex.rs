use slam_cv::Number;

pub trait VertexFormat<N>
where
    N: Number,
{
    fn format() -> wgpu::VertexFormat;
}

impl VertexFormat<f32> for nalgebra::Point3<f32> {
    fn format() -> wgpu::VertexFormat {
        wgpu::VertexFormat::Float3
    }
}

pub trait GpuVertex {
    fn weight() -> u64;
}

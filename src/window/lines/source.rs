use crate::pipes::VertexFormat;

use nalgebra::Point3;
use slam_cv::Number;

pub trait LineSource<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    const COLOR: [f32; 3];

    fn collect_visual_lines(&self) -> Vec<[Point3<N>; 2]>;
}

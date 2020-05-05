use crate::pipes::VertexFormat;

use nalgebra::Point3;
use slam_cv::{Colors, Number};

pub trait PointSource<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    const COLOR: [f32; 3] = Colors::red();

    fn collect_visual_points(&self) -> Vec<Point3<N>>;
}

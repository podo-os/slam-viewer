use super::point::Point;
use crate::pipes::VertexFormat;

use nalgebra::Point3;
use slam_cv::Number;

pub trait PointSource<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn collect_visual_points(&self) -> Vec<Point<N>>;
}

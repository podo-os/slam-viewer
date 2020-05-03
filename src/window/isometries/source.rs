use crate::pipes::VertexFormat;

use nalgebra::{Isometry3, Point3};
use slam_cv::Number;

pub trait IsometrySource<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    const COLOR: [f32; 3];
    const SIZE: [N; 2];

    fn collect_visual_isometries(&self) -> Vec<Isometry3<N>>;
}

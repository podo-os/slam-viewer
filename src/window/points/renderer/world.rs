use super::super::point::Point;
use super::super::source::PointSource;
use crate::pipes::VertexFormat;

use nalgebra::Point3;
use slam_cv::{feature::Landmark, vo::World, Colors, Number};

impl<N, F, W> PointSource<N> for W
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N>,
    W: 'static + World<Landmark = F>,
{
    fn collect_visual_points(&self) -> Vec<Point<N>> {
        self.collect_landmarks(|lm| Point {
            position: lm.point_world(),
            color: Colors::red(),
        })
    }
}

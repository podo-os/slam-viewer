use super::super::super::builder::{WindowBuilder, WindowBuilderDefault};
use super::super::super::camera::{Camera, CameraController};
use super::super::point::Point;
use super::super::source::PointSource;
use crate::pipes::VertexFormat;

use nalgebra::{Point3, Vector3};
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

impl<F, W> WindowBuilderDefault<f32> for W
where
    F: 'static + Landmark<Number = f32>,
    W: 'static + World<Landmark = F>,
{
    fn default_window() -> WindowBuilder<f32> {
        WindowBuilder {
            title: Some("Map Viewer".to_string()),
            framerate: Some(30),

            camera: Camera {
                eye: Point3::new(0., 2., 5.),
                target: Point3::new(0., 0., 0.),
                up: Vector3::y(),
                fovy: std::f32::consts::FRAC_PI_4,
                znear: 0.1,
                zfar: 100.0,
            },
            camera_controller: CameraController::new(0.01),
        }
    }
}

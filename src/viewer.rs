use crate::{
    engine::EngineBuilder,
    window::{Camera, CameraController, PointsBuilder, WindowBuilder},
};

use nalgebra::{Point3, Vector3};
use slam_cv::prelude::*;

/// TODO: one or **more** windows
pub struct Viewer<N>
where
    N: 'static + Number,
{
    pub title: String,
    pub framerate: Option<u64>,

    pub camera: Camera<N>,
    pub camera_controller: CameraController<N>,
}

impl Default for Viewer<f32> {
    fn default() -> Self {
        Self {
            title: "SLAM Map Viewer".to_string(),
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

impl Viewer<f32> {
    pub fn run<F, W>(self, world: W) -> !
    where
        F: 'static + Landmark<Number = f32>,
        W: 'static + World<Landmark = F>,
    {
        let window = WindowBuilder {
            camera: self.camera,
            camera_controller: self.camera_controller,
            pipeline_builder: Box::new(PointsBuilder::new(world)),
        };

        let engine = EngineBuilder {
            windows: vec![window],
        };

        engine.run()
    }
}

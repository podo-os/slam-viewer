use super::base::Camera;
use super::coord::CoordSystemRh;

use nalgebra::{Point3, Vector3};
use slam_cv::Number;

pub struct CameraFrustum<N>
where
    N: Number,
{
    pub eye: Point3<N>,
    pub at: Point3<N>,

    pub fovy: N,
    pub znear: N,
    pub zfar: N,
}

impl<N> Into<Camera<N>> for CameraFrustum<N>
where
    N: Number,
{
    fn into(self) -> Camera<N> {
        let mut camera = Camera {
            eye: Vector3::zeros().into(),
            yaw: N::zero(),
            pitch: N::zero(),

            fovy: self.fovy,
            znear: self.znear,
            zfar: self.zfar,

            coord_system: CoordSystemRh::from_up_axis(Vector3::y_axis()),
        };

        camera.look_at(Some(self.eye), self.at);

        camera
    }
}

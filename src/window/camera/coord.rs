use nalgebra::{Unit, UnitQuaternion, Vector3};
use slam_cv::Number;

#[derive(Clone, Copy, Debug)]
pub struct CoordSystemRh<N>
where
    N: 'static + Number,
{
    pub up_axis: Unit<Vector3<N>>,
    pub rotation_to_y_up: UnitQuaternion<N>,
}

impl<N> CoordSystemRh<N>
where
    N: 'static + Number,
{
    #[inline]
    pub fn from_up_axis(up_axis: Unit<Vector3<N>>) -> Self {
        let pi = N::from(std::f32::consts::PI).unwrap();

        let rotation_to_y_up = UnitQuaternion::rotation_between_axis(&up_axis, &Vector3::y_axis())
            .unwrap_or_else(|| UnitQuaternion::from_axis_angle(&Vector3::x_axis(), pi));
        Self {
            up_axis,
            rotation_to_y_up,
        }
    }
}

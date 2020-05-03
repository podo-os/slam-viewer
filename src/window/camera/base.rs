//! Many of this code is from [kiss3d](https://github.com/sebcrozet/kiss3d)
//! https://github.com/sebcrozet/kiss3d/blob/master/src/camera/first_person.rs

use super::coord::CoordSystemRh;

use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Vector2, Vector3};
use num::Float;
use slam_cv::Number;

#[derive(Debug)]
pub struct Camera<N>
where
    N: Number,
{
    pub eye: Point3<N>,
    pub yaw: N,
    pub pitch: N,

    pub fovy: N,
    pub znear: N,
    pub zfar: N,

    pub coord_system: CoordSystemRh<N>,
}

impl<N> Camera<N>
where
    N: Number,
{
    /// Changes the orientation and position of the camera to look at the specified point.
    pub fn look_at(&mut self, eye: Option<Point3<N>>, at: Point3<N>) {
        let eye = eye.or_else(|| Some(self.eye)).unwrap();
        let dist = (eye - at).norm();

        let view_eye = self.coord_system.rotation_to_y_up * eye;
        let view_at = self.coord_system.rotation_to_y_up * at;
        let pitch = Float::acos((view_at.y - view_eye.y) / dist);
        let yaw = Float::atan2(view_at.z - view_eye.z, view_at.x - view_eye.x);

        self.eye = eye;
        self.yaw = yaw;
        self.pitch = pitch;
    }

    pub fn compute_view_proj(&self, aspect: N) -> Matrix4<N> {
        let proj = Perspective3::new(aspect, self.fovy, self.znear, self.zfar);

        let view = self.view_transform().to_homogeneous();

        proj.as_matrix() * view
    }

    pub fn rotate(&mut self, dpos: Vector2<N>) {
        self.yaw += dpos.x;
        self.pitch += dpos.y;

        self.update_restrictions();
    }

    pub fn move_to(&mut self, dpos: Vector2<N>) {
        let at = self.at();
        let dir = (at - self.eye).normalize();
        let tangent = self.coord_system.up_axis.cross(&dir).normalize();
        let bitangent = dir.cross(&tangent);

        self.eye += tangent * dpos.x + bitangent * dpos.y;
    }

    pub fn scale(&mut self, yoff: N) {
        let front = self.observer_frame() * Vector3::z();

        self.eye += front * yoff;
    }

    /// The point the camera is looking at.
    fn at(&self) -> Point3<N> {
        let view_eye = self.coord_system.rotation_to_y_up * self.eye;
        let ax = view_eye.x + Float::cos(self.yaw) * Float::sin(self.pitch);
        let ay = view_eye.y + Float::cos(self.pitch);
        let az = view_eye.z + Float::sin(self.yaw) * Float::sin(self.pitch);
        self.coord_system.rotation_to_y_up.inverse() * Point3::new(ax, ay, az)
    }

    /// The camera view transformation (i-e transformation without projection).
    fn view_transform(&self) -> Isometry3<N> {
        Isometry3::look_at_rh(&self.eye, &self.at(), &self.coord_system.up_axis)
    }

    /// The camera observer local frame.
    fn observer_frame(&self) -> Isometry3<N> {
        Isometry3::face_towards(&self.eye, &self.at(), &self.coord_system.up_axis)
    }

    fn update_restrictions(&mut self) {
        let thr = N::from(0.01).unwrap();

        if self.pitch <= thr {
            self.pitch = thr;
        }

        let pi = N::from(std::f32::consts::PI).unwrap();
        if self.pitch > pi - thr {
            self.pitch = pi - thr;
        }
    }
}

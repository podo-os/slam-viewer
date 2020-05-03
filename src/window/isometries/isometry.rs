use super::super::lines::Line;
use super::super::points::Point;
use crate::pipes::{GpuVertex, VertexFormat};

use nalgebra::{Isometry3, Matrix4, Point2, Point3, Vector4};
use slam_cv::Number;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Isometry<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    lines: [Line<N>; 6],
}

impl<N> Isometry<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub fn from_iso(iso: Isometry3<N>, camera_size: Point2<N>, color: Point3<f32>) -> Self {
        Self::from_homogeneous(iso.to_homogeneous(), camera_size, color)
    }

    pub fn from_homogeneous(iso: Matrix4<N>, camera_size: Point2<N>, color: Point3<f32>) -> Self {
        let size = Point2::new(camera_size.x, camera_size.y);
        let pose = iso;

        Self {
            lines: [
                Line {
                    start: Point {
                        position: map_pose(&pose, -size.x, -size.y),
                        color,
                    },
                    end: Point {
                        position: map_pose(&pose, size.x, size.y),
                        color,
                    },
                },
                Line {
                    start: Point {
                        position: map_pose(&pose, -size.x, size.y),
                        color,
                    },
                    end: Point {
                        position: map_pose(&pose, size.x, -size.y),
                        color,
                    },
                },
                Line {
                    start: Point {
                        position: map_pose(&pose, size.x, size.y),
                        color,
                    },
                    end: Point {
                        position: map_pose(&pose, size.x, -size.y),
                        color,
                    },
                },
                Line {
                    start: Point {
                        position: map_pose(&pose, -size.x, size.y),
                        color,
                    },
                    end: Point {
                        position: map_pose(&pose, -size.x, -size.y),
                        color,
                    },
                },
                Line {
                    start: Point {
                        position: map_pose(&pose, size.x, size.y),
                        color,
                    },
                    end: Point {
                        position: map_pose(&pose, -size.x, size.y),
                        color,
                    },
                },
                Line {
                    start: Point {
                        position: map_pose(&pose, size.x, -size.y),
                        color,
                    },
                    end: Point {
                        position: map_pose(&pose, -size.x, -size.y),
                        color,
                    },
                },
            ],
        }
    }
}

fn map_pose<N>(pose: &Matrix4<N>, x: N, y: N) -> Point3<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    let m = pose * Vector4::new(x, y, N::zero(), N::one());
    Point3::new(m.x, m.y, m.z)
}

impl<N> GpuVertex for Isometry<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn weight() -> u64 {
        12
    }
}

unsafe impl<N> bytemuck::Pod for Isometry<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
}
unsafe impl<N> bytemuck::Zeroable for Isometry<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
}

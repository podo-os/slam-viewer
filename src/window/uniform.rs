use core::fmt::Debug;

use super::camera::Camera;

use nalgebra::Matrix4;
use slam_cv::Number;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms<N>
where
    N: 'static + Number,
{
    view_proj: Matrix4<N>,
}

impl<N> Default for Uniforms<N>
where
    N: 'static + Number,
{
    fn default() -> Self {
        Self {
            view_proj: Matrix4::identity(),
        }
    }
}

impl<N> Uniforms<N>
where
    N: 'static + Number,
{
    pub fn update_view_proj(&mut self, camera: &Camera<N>, aspect: N) {
        self.view_proj = camera.build_view_projection_matrix(aspect);
    }
}

unsafe impl<N> bytemuck::Pod for Uniforms<N> where N: 'static + Number {}
unsafe impl<N> bytemuck::Zeroable for Uniforms<N> where N: 'static + Number {}

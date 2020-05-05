use super::super::builder::WindowBuilder;
use super::super::camera::{CameraControllerConfig, CameraFrustum};
use super::super::points::{PointSource, PointsBuilder};
use crate::pipes::{PipelineBuilder, PipelineDataBuilder, PipelineRenderer};

use nalgebra::{base::allocator::Allocator, DefaultAllocator, DimName, Point, Point3, U2, U3};
use slam_cv::Number;

pub struct PointsModel<N, D>
where
    N: 'static + Number,
    D: DimName,
    DefaultAllocator: Allocator<N, D>,
{
    points: Vec<Point<N, D>>,
}

impl<N, D> PointsModel<N, D>
where
    N: 'static + Number,
    D: DimName,
    DefaultAllocator: Allocator<N, D>,
{
    pub fn new(points: Vec<Point<N, D>>) -> Self {
        Self { points }
    }
}

impl<D> PipelineDataBuilder<f32> for PointsModel<f32, D>
where
    Self: PipelineBuilder<f32> + Send,
    D: DimName,
    DefaultAllocator: Allocator<f32, D>,
{
    type Builder = Self;

    fn default_window(&self) -> WindowBuilder<f32> {
        WindowBuilder {
            title: Some("2d Points Viewer".to_string()),
            framerate: Some(120),

            camera: CameraFrustum {
                eye: Point3::new(0., 2., 5.),
                at: Point3::new(0., 0., 0.),

                fovy: std::f32::consts::FRAC_PI_4,
                znear: 0.1,
                zfar: 100.0,
            },
            camera_controller: CameraControllerConfig::default(),
        }
    }

    fn build_data(self) -> Self::Builder {
        self
    }
}

impl<N, D> PipelineBuilder<N> for PointsModel<N, D>
where
    Self: PointSource<f32> + Send,
    N: 'static + Number,
    D: DimName,
    DefaultAllocator: Allocator<N, D>,
{
    fn build(
        self: Box<Self>,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Box<dyn PipelineRenderer> {
        let points = *self;
        Box::new(PointsBuilder::new(points).build(
            device,
            texture_format,
            uniform_bind_group_layout,
        ))
    }
}

impl PointSource<f32> for PointsModel<f32, U2> {
    fn collect_visual_points(&self) -> Vec<Point3<f32>> {
        self.points
            .iter()
            .map(|p| Point3::new(p.x, p.y, 0.0))
            .collect()
    }
}

impl PointSource<f32> for PointsModel<f64, U2> {
    fn collect_visual_points(&self) -> Vec<Point3<f32>> {
        self.points
            .iter()
            .map(|p| Point3::new(p.x as f32, p.y as f32, 0.0))
            .collect()
    }
}

impl PointSource<f32> for PointsModel<f32, U3> {
    fn collect_visual_points(&self) -> Vec<Point3<f32>> {
        self.points.clone()
    }
}

impl PointSource<f32> for PointsModel<f64, U3> {
    fn collect_visual_points(&self) -> Vec<Point3<f32>> {
        self.points
            .iter()
            .map(|p| Point3::new(p.x as f32, p.y as f32, p.z as f32))
            .collect()
    }
}

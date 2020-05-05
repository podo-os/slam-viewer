use super::super::builder::WindowBuilder;
use super::super::camera::{CameraControllerConfig, CameraFrustum};
use super::super::lines::{LineSource, LinesBuilder, LinesRendener};
use super::super::points::{PointSource, PointsBuilder, PointsRendener};
use crate::pipes::{PipelineBuilder, PipelineDataBuilder, PipelineRenderer, VertexFormat};

use cv_core::FeatureMatch;
use nalgebra::{base::allocator::Allocator, DefaultAllocator, DimName, Point, Point3, U2, U3};
use slam_cv::Number;

#[derive(Clone)]
pub struct MatchesModel<N, D>
where
    N: 'static + Number,
    D: DimName,
    DefaultAllocator: Allocator<N, D>,
{
    matches: Vec<FeatureMatch<Point<N, D>>>,
}

impl<N, D> MatchesModel<N, D>
where
    N: 'static + Number,
    D: DimName,
    DefaultAllocator: Allocator<N, D>,
{
    pub fn new(matches: Vec<FeatureMatch<Point<N, D>>>) -> Self {
        Self { matches }
    }
}

impl<D> PipelineDataBuilder<f32> for MatchesModel<f32, D>
where
    Self: PointSource<f32> + LineSource<f32> + Send,
    D: DimName,
    DefaultAllocator: Allocator<f32, D>,
{
    type Builder = Self;

    fn default_window(&self) -> WindowBuilder<f32> {
        WindowBuilder {
            title: Some("Matches Viewer".to_string()),
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

impl<N, D> PipelineBuilder<N> for MatchesModel<N, D>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    D: DimName,
    DefaultAllocator: Allocator<N, D>,
    MatchesModel<N, D>: PipelineDataBuilder<N> + PointSource<N> + LineSource<N>,
{
    fn build(
        self: Box<Self>,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Box<dyn PipelineRenderer> {
        let matches = *self;
        Box::new(MatchesRenderer {
            points_1: PointsBuilder::new(matches.clone()).build(
                device,
                texture_format,
                uniform_bind_group_layout,
            ),
            points_2: PointsBuilder::new(matches.clone()).build(
                device,
                texture_format,
                uniform_bind_group_layout,
            ),
            lines: LinesBuilder::new(matches).build(
                device,
                texture_format,
                uniform_bind_group_layout,
            ),
        })
    }
}

pub struct MatchesRenderer<N, D>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    D: DimName,
    DefaultAllocator: Allocator<N, D>,
    MatchesModel<N, D>: PointSource<N> + LineSource<N>,
{
    points_1: PointsRendener<N, MatchesModel<N, D>>,
    points_2: PointsRendener<N, MatchesModel<N, D>>,
    lines: LinesRendener<N, MatchesModel<N, D>>,
}

impl<N, D> PipelineRenderer for MatchesRenderer<N, D>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    D: DimName,
    DefaultAllocator: Allocator<N, D>,
    MatchesModel<N, D>: PointSource<N> + LineSource<N>,
{
    fn render<'a>(&'a mut self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>) {
        self.points_1.render(device, render_pass);
        self.points_2.render(device, render_pass);
        self.lines.render(device, render_pass);
    }
}

impl<N> PointSource<N> for MatchesModel<N, U2>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn collect_visual_points(&self) -> Vec<Point3<N>> {
        self.matches
            .iter()
            .map(|&m| {
                let p1 = Point3::new(m.0.x, m.0.y, N::zero());
                let p2 = Point3::new(m.1.x, m.1.y, N::zero());
                vec![p1, p2]
            })
            .flatten()
            .collect()
    }
}

impl<N> PointSource<N> for MatchesModel<N, U3>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn collect_visual_points(&self) -> Vec<Point3<N>> {
        self.matches
            .iter()
            .map(|&m| vec![m.0, m.1])
            .flatten()
            .collect()
    }
}

impl<N> LineSource<N> for MatchesModel<N, U2>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn collect_visual_lines(&self) -> Vec<[Point3<N>; 2]> {
        self.matches
            .iter()
            .map(|&m| {
                let p1 = Point3::new(m.0.x, m.0.y, N::zero());
                let p2 = Point3::new(m.1.x, m.1.y, N::zero());
                [p1, p2]
            })
            .collect()
    }
}

impl<N> LineSource<N> for MatchesModel<N, U3>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn collect_visual_lines(&self) -> Vec<[Point3<N>; 2]> {
        self.matches.iter().map(|&m| [m.0, m.1]).collect()
    }
}

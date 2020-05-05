use core::marker::PhantomData;

use super::super::builder::WindowBuilder;
use super::super::camera::{CameraControllerConfig, CameraFrustum};
use super::super::isometries::{IsometriesBuilder, IsometriesRendener, IsometrySource};
use super::super::lines::{LineSource, LinesBuilder, LinesRendener};
use super::super::points::{PointSource, PointsBuilder, PointsRendener};
use crate::pipes::{PipelineBuilder, PipelineDataBuilder, PipelineRenderer, VertexFormat};

use nalgebra::{Isometry3, Point3};
use slam_cv::{feature::Landmark, frame::KeyFrame, vo::World, Number};

#[derive(Clone)]
pub struct WorldModel<N, F, KF, W>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N> + Clone,
    KF: 'static + KeyFrame<Number = N, Feature = F> + Clone,
    W: 'static + World<Number = N, KeyFrame = KF, Landmark = F> + Clone,
{
    world: W,

    _feature: PhantomData<F>,
    _keyframees: PhantomData<KF>,
}

impl<N, F, KF, W> WorldModel<N, F, KF, W>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N> + Clone,
    KF: 'static + KeyFrame<Number = N, Feature = F> + Clone,
    W: 'static + World<Number = N, KeyFrame = KF, Landmark = F> + Clone,
{
    pub fn new(world: W) -> Self {
        Self {
            world,

            _feature: Default::default(),
            _keyframees: Default::default(),
        }
    }
}

impl<F, KF, W> PipelineDataBuilder<f32> for WorldModel<f32, F, KF, W>
where
    Self: Send,
    F: 'static + Landmark<Number = f32> + Clone,
    KF: 'static + KeyFrame<Number = f32, Feature = F> + Clone,
    W: 'static + World<Number = f32, KeyFrame = KF, Landmark = F> + Clone,
{
    type Builder = Self;

    fn default_window(&self) -> WindowBuilder<f32> {
        WindowBuilder {
            title: Some("Map Viewer".to_string()),
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

impl<N, F, KF, W> PipelineBuilder<N> for WorldModel<N, F, KF, W>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N> + Clone,
    KF: 'static + KeyFrame<Number = N, Feature = F> + Clone,
    W: 'static + World<Number = N, KeyFrame = KF, Landmark = F> + Clone,
    WorldModel<N, F, KF, W>:
        PipelineDataBuilder<N> + PointSource<N> + LineSource<N> + IsometrySource<N>,
{
    fn build(
        self: Box<Self>,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Box<dyn PipelineRenderer> {
        let world = *self;
        Box::new(WorldRenderer {
            points: PointsBuilder::new(world.clone()).build(
                device,
                texture_format,
                uniform_bind_group_layout,
            ),
            lines: LinesBuilder::new(world.clone()).build(
                device,
                texture_format,
                uniform_bind_group_layout,
            ),
            isometries: IsometriesBuilder::new(world).build(
                device,
                texture_format,
                uniform_bind_group_layout,
            ),
        })
    }
}

pub struct WorldRenderer<N, F, KF, W>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N> + Clone,
    KF: 'static + KeyFrame<Number = N, Feature = F> + Clone,
    W: 'static + World<Number = N, KeyFrame = KF, Landmark = F> + Clone,
    WorldModel<N, F, KF, W>: PointSource<N> + LineSource<N> + IsometrySource<N>,
{
    points: PointsRendener<N, WorldModel<N, F, KF, W>>,
    lines: LinesRendener<N, WorldModel<N, F, KF, W>>,
    isometries: IsometriesRendener<N, WorldModel<N, F, KF, W>>,
}

impl<N, F, KF, W> PipelineRenderer for WorldRenderer<N, F, KF, W>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N> + Clone,
    KF: 'static + KeyFrame<Number = N, Feature = F> + Clone,
    W: 'static + World<Number = N, KeyFrame = KF, Landmark = F> + Clone,
    WorldModel<N, F, KF, W>: PointSource<N> + LineSource<N> + IsometrySource<N>,
{
    fn render<'a>(&'a mut self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>) {
        self.points.render(device, render_pass);
        self.lines.render(device, render_pass);
        self.isometries.render(device, render_pass);
    }
}

impl<N, F, KF, W> PointSource<N> for WorldModel<N, F, KF, W>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N> + Clone,
    KF: 'static + KeyFrame<Number = N, Feature = F> + Clone,
    W: 'static + World<Number = N, KeyFrame = KF, Landmark = F> + Clone,
{
    fn collect_visual_points(&self) -> Vec<Point3<N>> {
        self.world.collect_landmarks(Landmark::point_world)
    }
}

impl<N, F, KF, W> LineSource<N> for WorldModel<N, F, KF, W>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N> + Clone,
    KF: 'static + KeyFrame<Number = N, Feature = F> + Clone,
    W: 'static + World<Number = N, KeyFrame = KF, Landmark = F> + Clone,
{
    fn collect_visual_lines(&self) -> Vec<[Point3<N>; 2]> {
        let mut prev = None;

        self.world.collect_keyframes(|kf| {
            let p = kf.isometry().translation.vector.into();
            let line = match prev {
                Some(prev) => [prev, p],
                None => [p, p],
            };
            prev = Some(p);
            line
        })
    }
}

impl<F, KF, W> IsometrySource<f32> for WorldModel<f32, F, KF, W>
where
    F: 'static + Landmark<Number = f32> + Clone,
    KF: 'static + KeyFrame<Number = f32, Feature = F> + Clone,
    W: 'static + World<Number = f32, KeyFrame = KF, Landmark = F> + Clone,
{
    const SIZE: [f32; 2] = [0.2, 0.16];

    fn collect_visual_isometries(&self) -> Vec<Isometry3<f32>> {
        self.world.collect_keyframes(KF::isometry)
    }
}

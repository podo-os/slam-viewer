use super::super::builder::{WindowBuilder, WindowBuilderDefault};
use super::super::camera::{CameraControllerConfig, CameraFrustum};
use super::super::isometries::{IsometriesBuilder, IsometriesRendener, IsometrySource};
use super::super::lines::{LineSource, LinesBuilder, LinesRendener};
use super::super::points::{PointSource, PointsBuilder, PointsRendener};
use crate::pipes::{PipelineBuilder, PipelineRenderer};

use nalgebra::{Isometry3, Point3};
use slam_cv::{feature::Landmark, frame::KeyFrame, vo::World, Colors};

pub struct WorldRenderer<F, KF, W>
where
    F: 'static + Landmark<Number = f32>,
    KF: 'static + KeyFrame<Number = f32, Feature = F>,
    W: 'static + World<Number = f32, KeyFrame = KF, Landmark = F>,
{
    points: PointsRendener<f32, W>,
    lines: LinesRendener<f32, W>,
    isometries: IsometriesRendener<f32, W>,
}

impl<F, KF, W> PipelineBuilder for W
where
    F: 'static + Landmark<Number = f32>,
    KF: 'static + KeyFrame<Number = f32, Feature = F>,
    W: 'static + World<Number = f32, KeyFrame = KF, Landmark = F> + Clone,
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

impl<F, KF, W> PipelineRenderer for WorldRenderer<F, KF, W>
where
    F: 'static + Landmark<Number = f32>,
    KF: 'static + KeyFrame<Number = f32, Feature = F>,
    W: 'static + World<Number = f32, KeyFrame = KF, Landmark = F>,
{
    fn render<'a>(&'a mut self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>) {
        self.points.render(device, render_pass);
        self.lines.render(device, render_pass);
        self.isometries.render(device, render_pass);
    }
}

impl<F, KF, W> PointSource<f32> for W
where
    F: 'static + Landmark<Number = f32>,
    KF: 'static + KeyFrame<Number = f32, Feature = F>,
    W: 'static + World<Number = f32, KeyFrame = KF, Landmark = F>,
{
    const COLOR: [f32; 3] = Colors::red();

    fn collect_visual_points(&self) -> Vec<Point3<f32>> {
        self.collect_landmarks(Landmark::point_world)
    }
}

impl<F, KF, W> LineSource<f32> for W
where
    F: 'static + Landmark<Number = f32>,
    KF: 'static + KeyFrame<Number = f32, Feature = F>,
    W: 'static + World<Number = f32, KeyFrame = KF, Landmark = F>,
{
    const COLOR: [f32; 3] = Colors::blue();

    fn collect_visual_lines(&self) -> Vec<[Point3<f32>; 2]> {
        let mut prev = None;

        self.collect_keyframes(|kf| {
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

impl<F, KF, W> IsometrySource<f32> for W
where
    F: 'static + Landmark<Number = f32>,
    KF: 'static + KeyFrame<Number = f32, Feature = F>,
    W: 'static + World<Number = f32, KeyFrame = KF, Landmark = F>,
{
    const COLOR: [f32; 3] = Colors::green();
    const SIZE: [f32; 2] = [0.2, 0.16];

    fn collect_visual_isometries(&self) -> Vec<Isometry3<f32>> {
        self.collect_keyframes(KF::isometry)
    }
}

impl<F, W> WindowBuilderDefault<f32> for W
where
    F: 'static + Landmark<Number = f32>,
    W: 'static + World<Number = f32, Landmark = F>,
{
    fn default_window() -> WindowBuilder<f32> {
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
            camera_controller: CameraControllerConfig {
                mouse_left_speed: 1.0,
                mouse_right_speed: 5.0,
                scroll_speed: 1.0,
                keyboard_speed: 0.1,
            },
        }
    }
}

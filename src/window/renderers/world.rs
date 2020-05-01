use super::super::builder::{WindowBuilder, WindowBuilderDefault};
use super::super::camera::{Camera, CameraController};
use super::super::lines::{Line, LineSource, LinesBuilder, LinesRendener};
use super::super::points::{Point, PointSource, PointsBuilder, PointsRendener};
use crate::pipes::{PipelineBuilder, PipelineRenderer, VertexFormat};

use nalgebra::{Point3, Vector3};
use slam_cv::{feature::Landmark, vo::World, Colors, Number};

pub struct WorldRenderer<N, F, W>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N>,
    W: 'static + World<Landmark = F>,
{
    points: PointsRendener<N, W>,
    lines: LinesRendener<N, W>,
}

impl<N, F, W> PipelineBuilder for W
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N>,
    W: 'static + World<Landmark = F> + Clone,
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
            lines: LinesBuilder::new(world).build(
                device,
                texture_format,
                uniform_bind_group_layout,
            ),
        })
    }
}

impl<N, F, W> PipelineRenderer for WorldRenderer<N, F, W>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N>,
    W: 'static + World<Landmark = F>,
{
    fn render<'a>(&'a mut self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass<'a>) {
        self.points.render(device, render_pass);
        self.lines.render(device, render_pass);
    }
}

impl<N, F, W> PointSource<N> for W
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N>,
    W: 'static + World<Landmark = F>,
{
    fn collect_visual_points(&self) -> Vec<Point<N>> {
        self.collect_landmarks(|lm| Point {
            position: lm.point_world(),
            color: Colors::red(),
        })
    }
}

impl<N, F, W> LineSource<N> for W
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    F: 'static + Landmark<Number = N>,
    W: 'static + World<Landmark = F>,
{
    fn collect_visual_lines(&self) -> Vec<Line<N>> {
        self.collect_landmarks(|lm| Line {
            start: Point {
                position: lm.point_world(),
                color: Colors::red(),
            },
            end: Point::default(),
        })
    }
}

impl<F, W> WindowBuilderDefault<f32> for W
where
    F: 'static + Landmark<Number = f32>,
    W: 'static + World<Landmark = F>,
{
    fn default_window() -> WindowBuilder<f32> {
        WindowBuilder {
            title: Some("Map Viewer".to_string()),
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

use crate::{
    engine::{Engine, EngineBuilder},
    pipes::{PipelineBuilder, PipelineDataBuilder, VertexFormat},
    window::{models, IsometrySource, LineSource, PointSource, WindowBuilder},
};

use nalgebra::{allocator::Allocator, DefaultAllocator, DimName, Point, Point3};
use slam_cv::prelude::*;

/// **caution**: This function can only be called once per process.
pub fn alloc_thread<N>() -> Viewer<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    Viewer { windows: vec![] }
}

pub struct Viewer<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    windows: Vec<(WindowBuilder<N>, Box<dyn PipelineBuilder<N>>)>,
}

impl<N> Viewer<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub fn add_world<F, KF, W>(self, world: W) -> Self
    where
        F: 'static + Landmark<Number = N> + Clone,
        KF: 'static + KeyFrame<Number = N, Feature = F> + Clone,
        W: 'static + World<Number = N, KeyFrame = KF, Landmark = F> + Clone,
        models::WorldModel<N, F, KF, W>:
            PipelineDataBuilder<N> + PointSource<N> + LineSource<N> + IsometrySource<N>,
    {
        self.add(models::WorldModel::new(world))
    }

    pub fn add_points<D>(self, points: Vec<Point<N, D>>) -> Self
    where
        D: DimName,
        DefaultAllocator: Allocator<N, D>,
        models::PointsModel<N, D>: PipelineBuilder<N> + PipelineDataBuilder<N> + PointSource<f32>,
    {
        self.add(models::PointsModel::new(points))
    }

    #[cfg(feature = "rust-cv")]
    pub fn add_matches<D>(self, matches: Vec<cv_core::FeatureMatch<Point<N, D>>>) -> Self
    where
        D: DimName,
        DefaultAllocator: Allocator<N, D>,
        models::MatchesModel<N, D>:
            PipelineBuilder<N> + PipelineDataBuilder<N> + PointSource<f32> + LineSource<f32>,
    {
        self.add(models::MatchesModel::new(matches))
    }
}

impl<N> Viewer<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    fn add<D>(self, data: D) -> Self
    where
        D: 'static + PipelineBuilder<N> + PipelineDataBuilder<N>,
    {
        self.add_window_pipe(data.default_window(), data)
    }

    fn add_window_pipe<P>(mut self, window: WindowBuilder<N>, pipe: P) -> Self
    where
        P: 'static + PipelineBuilder<N>,
    {
        self.windows.push((window, Box::new(pipe)));
        self
    }

    pub fn run(self) {
        self.compile().run()
    }

    /// TODO: cross-platform compatibility
    pub fn spawn(self) -> Engine {
        self.compile().spawn()
    }

    fn compile(self) -> EngineBuilder<N> {
        EngineBuilder {
            windows: self.windows,
        }
    }
}

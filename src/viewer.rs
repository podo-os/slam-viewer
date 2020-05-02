use crate::{
    engine::{Engine, EngineBuilder},
    pipes::{PipelineBuilder, PipelineDataBuilder, VertexFormat},
    window::{PointsBuilder, WindowBuilder, WindowBuilderDefault},
};

use nalgebra::Point3;
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
    windows: Vec<(WindowBuilder<N>, Box<dyn PipelineBuilder + Send>)>,
}

impl<N> Viewer<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub fn add<D>(self, data: D) -> Self
    where
        D: 'static + WindowBuilderDefault<N> + PipelineDataBuilder,
        D::Builder: Send,
    {
        self.add_pipe::<D, D::Builder>(data.build_data())
    }

    pub fn add_window_world<F, W>(self, window: WindowBuilder<N>, world: W) -> Self
    where
        F: 'static + Landmark<Number = N>,
        W: 'static + World<Landmark = F> + WindowBuilderDefault<N> + Send,
    {
        self.add_window_pipe(window, PointsBuilder::new(world))
    }

    fn add_pipe<D, P>(self, pipe: P) -> Self
    where
        D: 'static + WindowBuilderDefault<N>,
        P: 'static + PipelineBuilder + Send,
    {
        self.add_window_pipe(D::default_window(), pipe)
    }

    fn add_window_pipe<P>(mut self, window: WindowBuilder<N>, pipe: P) -> Self
    where
        P: 'static + PipelineBuilder + Send,
    {
        self.windows.push((window, Box::new(pipe)));
        self
    }

    pub fn run(self) -> ! {
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

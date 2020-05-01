use crate::{
    engine::EngineBuilder,
    pipes::{PipelineBuilder, PipelineDataBuilder, VertexFormat},
    window::{PointsBuilder, WindowBuilder, WindowBuilderDefault},
};

use nalgebra::Point3;
use slam_cv::prelude::*;

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
    windows: Vec<(WindowBuilder<N>, Box<dyn PipelineBuilder>)>,
}

impl<N> Viewer<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub fn add<D>(self, data: D) -> Self
    where
        D: 'static + WindowBuilderDefault<N> + PipelineDataBuilder,
    {
        self.add_pipe::<D, D::Builder>(data.build_data())
    }

    pub fn add_window_world<F, W>(self, window: WindowBuilder<N>, world: W) -> Self
    where
        F: 'static + Landmark<Number = N>,
        W: 'static + World<Landmark = F> + WindowBuilderDefault<N>,
    {
        self.add_window_pipe(window, PointsBuilder::new(world))
    }

    fn add_pipe<D, P>(self, pipe: P) -> Self
    where
        D: 'static + WindowBuilderDefault<N>,
        P: 'static + PipelineBuilder,
    {
        self.add_window_pipe(D::default_window(), pipe)
    }

    fn add_window_pipe<P>(mut self, window: WindowBuilder<N>, pipe: P) -> Self
    where
        P: 'static + PipelineBuilder,
    {
        self.windows.push((window, Box::new(pipe)));
        self
    }

    pub fn run(self) -> ! {
        let engine = EngineBuilder {
            windows: self.windows,
        };

        engine.run()
    }
}

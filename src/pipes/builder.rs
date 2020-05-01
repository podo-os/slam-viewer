use super::renderer::PipelineRenderer;

pub trait PipelineBuilder {
    fn build(
        self: Box<Self>,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Box<dyn PipelineRenderer>;
}

pub trait PipelineDataBuilder {
    type Builder: 'static + PipelineBuilder;

    fn build_data(self) -> Self::Builder;
}

impl<T> PipelineDataBuilder for T
where
    Self: 'static + PipelineBuilder,
{
    type Builder = Self;

    fn build_data(self) -> Self::Builder {
        self
    }
}

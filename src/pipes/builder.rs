use super::renderer::PipelineRenderer;

pub trait PipelineBuilder {
    fn build(
        self: Box<Self>,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Box<dyn PipelineRenderer>;
}

pub trait PipelineAutoBuilder<D>
where
    Self: PipelineBuilder,
    D: 'static,
{
    fn auto_build(data: D) -> Box<Self>;
}

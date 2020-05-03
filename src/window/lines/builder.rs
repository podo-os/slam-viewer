use core::marker::PhantomData;

use super::line::Line;
use super::renderer::LinesRendener;
use super::source::LineSource;
use crate::pipes::{GpuVec, StaticShaderModule, VertexFormat};

use nalgebra::Point3;
use slam_cv::Number;

pub struct LinesBuilder<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: LineSource<N>,
{
    pub source: S,

    number: PhantomData<N>,
}

impl<N, S> LinesBuilder<N, S>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
    S: LineSource<N>,
{
    pub fn new(source: S) -> Self {
        Self {
            source,

            number: Default::default(),
        }
    }

    pub fn build(
        self,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> LinesRendener<N, S> {
        let render_pipeline =
            build_render_pipeline(device, texture_format, uniform_bind_group_layout);

        LinesRendener {
            render_pipeline,
            buffer: GpuVec::new(wgpu::BufferUsage::VERTEX),

            number: Default::default(),
            source: self.source,
        }
    }
}

pub fn build_render_pipeline(
    device: &wgpu::Device,
    texture_format: wgpu::TextureFormat,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    const VS_SRC: StaticShaderModule = StaticShaderModule {
        spirv_source: include_bytes!("../points/shader.vert.spv"),
        entry_point: None,
    };
    const FS_SRC: StaticShaderModule = StaticShaderModule {
        spirv_source: include_bytes!("../points/shader.frag.spv"),
        entry_point: None,
    };

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&uniform_bind_group_layout],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &render_pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &VS_SRC.build(device),
            entry_point: VS_SRC.entry_point(),
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &FS_SRC.build(device),
            entry_point: FS_SRC.entry_point(),
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        color_states: &[wgpu::ColorStateDescriptor {
            format: texture_format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        primitive_topology: wgpu::PrimitiveTopology::LineList,
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[Line::desc(&Line::attributes())],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    })
}

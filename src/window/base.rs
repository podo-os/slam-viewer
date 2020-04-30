use super::builder::WindowBuilder;
use super::camera::{Camera, CameraController};
use super::event::WindowEventState;
use super::uniform::Uniforms;
use crate::pipes::{PipelineRenderer, VertexFormat};

use nalgebra::Point3;
use slam_cv::Number;
use winit::{event::*, window};

pub struct Window<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub window: window::Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    pipeline_rendener: Box<dyn PipelineRenderer>,

    // TODO move camera to ShaderPlugin
    camera: Camera<N>,
    camera_controller: CameraController<N>,

    uniforms: Uniforms<N>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl<N> Window<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub async fn new(window: window::Window, builder: WindowBuilder<N>) -> Self {
        let size = window.inner_size();

        let surface = wgpu::Surface::create(&window);

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            // Vulkan + Metal + DX12 + Browser WebGPU
            wgpu::BackendBit::PRIMARY,
        )
        .await
        .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: Default::default(),
            })
            .await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let mut uniforms = Uniforms::default();
        uniforms.update_view_proj(&builder.camera, Self::aspect(&sc_desc));

        let uniform_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniforms]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &uniform_buffer,
                    // FYI: you can share a single buffer between bindings.
                    range: 0..std::mem::size_of_val(&uniforms) as wgpu::BufferAddress,
                },
            }],
            label: Some("uniform_bind_group"),
        });

        let pipeline_rendener =
            builder
                .pipeline_builder
                .build(&device, sc_desc.format, &uniform_bind_group_layout);

        let camera = builder.camera;
        let camera_controller = builder.camera_controller;

        Self {
            window,
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,

            pipeline_rendener,

            camera,
            camera_controller,

            uniforms,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn input(&mut self, event: &WindowEvent) -> WindowEventState {
        match event {
            WindowEvent::KeyboardInput { .. } => self.camera_controller.process_events(event),
            _ => WindowEventState::Unused,
        }
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.uniforms
            .update_view_proj(&self.camera, Self::aspect(&self.sc_desc));

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("update encoder"),
            });
        let staging_buffer = self.device.create_buffer_with_data(
            bytemuck::cast_slice(&[self.uniforms]),
            wgpu::BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.uniform_buffer,
            0,
            std::mem::size_of::<Uniforms<N>>() as wgpu::BufferAddress,
        );

        // We need to remember to submit our CommandEncoder's output
        // otherwise we won't see any change.
        self.queue.submit(&[encoder.finish()]);
    }

    pub fn render(&mut self) {
        let frame = self
            .swap_chain
            .get_next_texture()
            .expect("Timeout getting texture");

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::BLACK,
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            self.pipeline_rendener
                .render(&self.device, &mut render_pass);
        }

        self.queue.submit(&[encoder.finish()]);
    }

    fn aspect(sc_desc: &wgpu::SwapChainDescriptor) -> N {
        N::from(sc_desc.width).unwrap() / N::from(sc_desc.height).unwrap()
    }
}

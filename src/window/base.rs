use super::builder::WindowBuilder;
use super::camera::{Camera, CameraController};
use super::event::WindowEventState;
use super::uniform::Uniforms;
use crate::pipes::{PipelineBuilder, PipelineRenderer, VertexFormat};

use nalgebra::{Point3, Vector2};
use slam_cv::Number;
use winit::{event::*, window};

pub struct Window<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    window: window::Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    pipeline_rendener: Box<dyn PipelineRenderer>,

    // TODO move camera to ShaderPlugin
    camera: Camera<N>,
    camera_controller: CameraController<N>,

    pub framerate: Option<u64>,

    uniforms: Uniforms<N>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl<N> Window<N>
where
    N: 'static + Number,
    Point3<N>: VertexFormat<N>,
{
    pub async fn new(
        window: window::Window,
        builder: WindowBuilder<N>,
        pipeline_builder: Box<dyn PipelineBuilder>,
    ) -> Self {
        let size = window.inner_size();

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;

            // On wasm, append the canvas to the document body
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
        }

        let instance = wgpu::Instance::new();
        let surface = unsafe { instance.create_surface(&window) };

        let adapter = instance
            .request_adapter(
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
            .request_device(
                &wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions {
                        anisotropic_filtering: false,
                    },
                    limits: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        #[cfg(not(target_arch = "wasm32"))]
        let sc_format = wgpu::TextureFormat::Bgra8UnormSrgb;

        #[cfg(target_arch = "wasm32")]
        let sc_format = wgpu::TextureFormat::Bgra8Unorm;
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: sc_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let camera = builder.camera.into();
        let mut camera_controller: CameraController<N> = builder.camera_controller.into();

        camera_controller.window_size =
            Vector2::new(N::from(size.width).unwrap(), N::from(size.height).unwrap());

        let mut uniforms = Uniforms::default();
        uniforms.update_view_proj(&camera, Self::aspect(&sc_desc));

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
            pipeline_builder.build(&device, sc_desc.format, &uniform_bind_group_layout);

        let framerate = builder.framerate;

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

            framerate,

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
        self.camera_controller.process_events(event)
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
        self.queue.submit(Some(encoder.finish()));
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

        self.queue.submit(Some(encoder.finish()));
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    fn aspect(sc_desc: &wgpu::SwapChainDescriptor) -> N {
        N::from(sc_desc.width).unwrap() / N::from(sc_desc.height).unwrap()
    }
}

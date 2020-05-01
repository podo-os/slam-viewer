use super::vertex::GpuVertex;

pub struct GpuVec<D>
where
    D: bytemuck::Pod + bytemuck::Zeroable + GpuVertex,
{
    cpu_vec: Option<Vec<D>>,

    gpu_buffer: Option<wgpu::Buffer>,
    gpu_buffer_size: u64,

    usage: wgpu::BufferUsage,
}

impl<D> GpuVec<D>
where
    D: bytemuck::Pod + bytemuck::Zeroable + GpuVertex,
{
    pub fn new(usage: wgpu::BufferUsage) -> Self {
        Self {
            cpu_vec: None,

            gpu_buffer: None,
            gpu_buffer_size: 0,

            usage,
        }
    }

    pub fn update(&mut self, device: &wgpu::Device, vec: Vec<D>) {
        self.cpu_vec = Some(vec);
        self.update_buffer(device);
    }

    pub fn set_buffer<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if let Some(buffer) = &self.gpu_buffer {
            let data_size = core::mem::size_of::<D>() as u64;
            let gpu_buffer_size = self.gpu_buffer_size * data_size;
            let vec_len = (D::weight() * self.gpu_buffer_size) as u32;

            render_pass.set_vertex_buffer(0, buffer, 0, gpu_buffer_size);
            render_pass.draw(0..vec_len, 0..1);
        }
    }

    fn update_buffer(&mut self, device: &wgpu::Device) {
        if let Some(cpu_vec) = &self.cpu_vec {
            let size = cpu_vec.len() as u64;
            if size > self.gpu_buffer_size {
                let cpu_data = bytemuck::cast_slice(&cpu_vec);
                self.gpu_buffer = Some(device.create_buffer_with_data(cpu_data, self.usage));
                self.gpu_buffer_size = size;
            } else {
                // TODO: more efficient write
                // FIXME: to be implemented
            }
        }
    }
}

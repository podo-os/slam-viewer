use std::io::Cursor;

pub struct StaticShaderModule {
    pub spirv_source: &'static [u8],
    pub entry_point: Option<&'static str>,
}

impl StaticShaderModule {
    pub fn build(&self, device: &wgpu::Device) -> wgpu::ShaderModule {
        let data = wgpu::read_spirv(Cursor::new(self.spirv_source)).unwrap();
        device.create_shader_module(&data)
    }

    pub fn entry_point(&self) -> &str {
        self.entry_point.unwrap_or("main")
    }
}

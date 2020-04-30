pub struct StaticShaderModule {
    pub glsl_source: &'static str,
    pub shader_type: glsl_to_spirv::ShaderType,
    pub entry_point: Option<&'static str>,
}

impl StaticShaderModule {
    pub fn build(&self, device: &wgpu::Device) -> wgpu::ShaderModule {
        let spirv = glsl_to_spirv::compile(self.glsl_source, self.shader_type.clone()).unwrap();
        let data = wgpu::read_spirv(spirv).unwrap();
        device.create_shader_module(&data)
    }

    pub fn entry_point(&self) -> &str {
        self.entry_point.unwrap_or("main")
    }
}

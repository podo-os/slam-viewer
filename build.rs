use std::error::Error;
use std::io::Read;
use std::path::PathBuf;

use glob::glob;
use glsl_to_spirv::ShaderType;

fn build_shader(path: PathBuf, shader_type: ShaderType) -> Result<(), Box<dyn Error>> {
    let out = {
        let mut path = path.clone();
        let filename = format!("{}.spv", path.file_name().unwrap().to_str().unwrap());
        path.pop();
        path.push(filename);
        path
    };

    let glsl = std::fs::read_to_string(path)?;

    let mut spirv_file = glsl_to_spirv::compile(&glsl, shader_type)?;

    let mut spirv_bytes = Vec::new();
    spirv_file.read_to_end(&mut spirv_bytes)?;

    std::fs::write(out, spirv_bytes)?;

    Ok(())
}

fn build_shaders(format: &str, shader_type: ShaderType) -> Result<(), Box<dyn Error>> {
    for entry in glob(format).unwrap() {
        if let Ok(path) = entry {
            build_shader(path, shader_type.clone())?;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Tell the build script to only run again if we change our source shaders
    // println!("cargo:rerun-if-changed=source_assets/shaders");

    build_shaders("src/**/*.vert", ShaderType::Vertex)?;
    build_shaders("src/**/*.frag", ShaderType::Fragment)?;

    Ok(())
}

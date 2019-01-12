use crate::Result;
use ash::{version::DeviceV1_0, vk};
use failure::bail;
use log::{debug, info};
use shaderc;
use std::fs;
use std::path::Path;

pub enum ShaderSet {
    Basic,
}

pub fn get_shader_modules(
    shader_set: ShaderSet,
    device: ash::Device,
) -> Result<Vec<vk::ShaderModule>> {
    let path = Path::new("hobby/src/renderer/pipelines/shaders");
    let file_names = match shader_set {
        ShaderSet::Basic => ["basic.vert", "basic.frag"],
    };

    //todo: do something with the option
    let mut compiler = shaderc::Compiler::new().unwrap();

    let results: Vec<Result<vk::ShaderModule>> = file_names
        .iter()
        .map(|file_name| {
            let full_path = path.join(file_name);
            let shader_type = full_path.extension().unwrap().to_str().unwrap();
            let shader_kind = match shader_type {
                "vert" => shaderc::ShaderKind::Vertex,
                "frag" => shaderc::ShaderKind::Fragment,
                _ => bail!("Unknown shader type: {}, file: {}", shader_type, file_name),
            };

            info!("Compiling: {}, shader type: {:?}", file_name, shader_kind);

            let code_str = fs::read_to_string(full_path).unwrap();
            let artifact =
                compiler.compile_into_spirv(&code_str, shader_kind, file_name, "main", None)?;

            let create_info = vk::ShaderModuleCreateInfo::builder().code(&artifact.as_binary());
            let module = unsafe { device.create_shader_module(&create_info, None).unwrap() };
            Ok(module)
        })
        .collect();

    let mut modules: Vec<vk::ShaderModule> = vec![];

    for result in results {
        match result {
            Ok(module) => modules.push(module),
            Err(e) => bail!("Unable to create Shader Module; Error: {}", e),
        }
    }

    Ok(modules)
}

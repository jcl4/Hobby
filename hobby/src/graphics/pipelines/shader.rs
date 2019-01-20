use crate::{core::MaterialType, Result};
use ash::{version::DeviceV1_0, vk};
use failure::bail;
use log::{debug, info};
use shaderc;
use std::fs;
use std::path::Path;

pub fn get_shader_modules(
    material_type: MaterialType,
    device: &ash::Device,
) -> Result<Vec<vk::ShaderModule>> {
    //todo: need to not hardcode path to shaders...
    let root = Path::new("./hobby/res/shaders");

    let file_names = match material_type {
        MaterialType::Basic => ["basic.vert", "basic.frag"],
    };

    //todo: do something with the option returned from new
    let mut compiler = shaderc::Compiler::new().unwrap();

    let results: Vec<Result<vk::ShaderModule>> = file_names
        .iter()
        .map(|file_name| {
            let path = root.join(file_name);
            let shader_type = path.extension().unwrap().to_str().unwrap();
            let shader_kind = match shader_type {
                "vert" => shaderc::ShaderKind::Vertex,
                "frag" => shaderc::ShaderKind::Fragment,
                _ => bail!("Unknown shader type: {}, file: {}", shader_type, file_name),
            };
            debug!("Full path to Shader: {:?}", path);

            info!("Compiling: {}, shader type: {:?}", file_name, shader_kind);

            let code_str = fs::read_to_string(path).unwrap();
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

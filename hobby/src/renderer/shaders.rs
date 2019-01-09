use crate::Result;
use ash::{util::read_spv, version::DeviceV1_0, vk};
use std::{fs::File, path::Path};

pub enum ShaderSet {
    Basic,
}

pub fn get_shader_modules(
    shader_set: ShaderSet,
    device: ash::Device,
) -> Result<(vk::ShaderModule, vk::ShaderModule)> {
    let path = Path::new("hobby/src/renderer/shaders");
    let file_names = match shader_set {
        ShaderSet::Basic => ["basic.vert.spv", "basic.frag.spv"],
    };

    let modules: Vec<vk::ShaderModule> = file_names
        .iter()
        .map(|file_name| {
            let mut spv_file = File::open(path.join(file_name)).unwrap();
            let code = read_spv(&mut spv_file).unwrap();
            let create_info = vk::ShaderModuleCreateInfo::builder().code(&code);

            unsafe { device.create_shader_module(&create_info, None).unwrap() }
        })
        .collect();

    Ok((modules[0], modules[1]))
}

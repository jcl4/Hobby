use ash::{version::DeviceV1_0, vk};
use std::{fs::File, path::Path};

mod solid_color;

pub use solid_color::SolidColor;

fn create_shader_module(shader_file: &Path, device: &ash::Device) -> vk::ShaderModule {
    log::debug!("Loading Shader File: {:?}", shader_file);
    let mut file = File::open(shader_file)
        .unwrap_or_else(|_| panic!("Unable to open Shader File: {:?}", shader_file));
    let spv = ash::util::read_spv(&mut file)
        .unwrap_or_else(|_| panic!("Unable to read SPV File: {:?}", shader_file));
    let create_info = vk::ShaderModuleCreateInfo::builder().code(&spv).build();
    unsafe {
        device
            .create_shader_module(&create_info, None)
            .unwrap_or_else(|_| panic!("Unable to read SPV File: {:?}", shader_file))
    }
}

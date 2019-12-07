use std::path::Path;

/// Solid Color pipeline
pub struct SolidColor {}

impl SolidColor {
    pub fn new(device: &ash::Device) -> Self {
        let vertex_file = Path::new("resources/shaders/solid_color.frag.spv");
        let fragment_file = Path::new("resources/shaders/solid_color.frag.spv");

        let vert_module = super::create_shader_module(vertex_file, device);
        let frag_module = super::create_shader_module(fragment_file, device);

        log::debug!("Solid Color Graphics Pipleline Built");

        SolidColor {}
    }
}

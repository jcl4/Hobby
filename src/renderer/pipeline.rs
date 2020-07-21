use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use ash::{version::DeviceV1_0, vk, Device};

use crate::{Material, Renderer};

pub struct Pipeline {
    vert_module: vk::ShaderModule,
    frag_module: vk::ShaderModule,
}

impl Pipeline {
    pub fn new(material: &Material, renderer: &Renderer) -> Pipeline {
        match material {
            Material::ColoredVertex => {
                log::debug!("Creating Colored Vertex Pipeline");
                let vert_code = include_bytes!("shaders/colored_vertex.vert.spv");
                let vert_module = create_shader_module(&renderer.device, vert_code);

                let frag_code = include_bytes!("shaders/colored_vertex.frag.spv");
                let frag_module = create_shader_module(&renderer.device, frag_code);

                let vertex_input_create_info = vk::PipelineVertexInputStateCreateInfo::builder();
                // .vertex_binding_descriptions() null since vertices are hard coded in the shader
                // .vertex_attribute_descriptions() same here

                let input_assembly_create_info =
                    vk::PipelineInputAssemblyStateCreateInfo::builder()
                        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
                        .primitive_restart_enable(false);

                Pipeline {
                    vert_module,
                    frag_module,
                }
            }
            _ => unimplemented!(),
        }

        // let vert_shader = create_shader_module(, code: &[u8]);
    }

    pub(crate) fn cleanup(&self, renderer: &Renderer) {
        unsafe {
            renderer
                .device
                .destroy_shader_module(self.vert_module, None);
            renderer
                .device
                .destroy_shader_module(self.frag_module, None);
        }
    }
}

fn create_shader_module(device: &Device, code: &[u8]) -> vk::ShaderModule {
    let mut code_cursor = Cursor::new(code);
    let spirv = ash::util::read_spv(&mut code_cursor).expect("Unable to create SPIRV Code");

    let create_info = vk::ShaderModuleCreateInfo::builder().code(&spirv);
    unsafe {
        device
            .create_shader_module(&create_info, None)
            .expect("Unable to create shader module")
    }
}

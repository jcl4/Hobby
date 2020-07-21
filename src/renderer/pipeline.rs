use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use ash::{version::DeviceV1_0, vk, Device};

use super::swapchain::SwapchainDetails;
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
                create_colored_vertex_pipeline(renderer)
            }
            _ => unimplemented!(),
        }
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

fn create_colored_vertex_pipeline(renderer: &Renderer) -> Pipeline {
    let vert_code = include_bytes!("shaders/colored_vertex.vert.spv");
    let vert_module = create_shader_module(&renderer.device, vert_code);

    let frag_code = include_bytes!("shaders/colored_vertex.frag.spv");
    let frag_module = create_shader_module(&renderer.device, frag_code);

    let vertex_input_create_info = vk::PipelineVertexInputStateCreateInfo::builder();
    // .vertex_binding_descriptions() null since vertices are hard coded in the shader
    // .vertex_attribute_descriptions() same here

    let input_assembly_create_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);
    let viewports_create_info = get_viewports_create_info(&renderer.swapchain_details);
    let rasterizer_create_info = get_rasterizer_create_info();
    let multisample_create_info = get_multisampline_create_info();
    let color_blending_create_info = get_color_blending_create_info();

    Pipeline {
        vert_module,
        frag_module,
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

fn get_viewports_create_info(details: &SwapchainDetails) -> vk::PipelineViewportStateCreateInfo {
    let viewport = vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: details.extent.width as _,
        height: details.extent.height as _,
        min_depth: 0.0,
        max_depth: 1.0,
    };
    let viewports = [viewport];
    let scissor = vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: details.extent,
    };
    let scissors = [scissor];

    vk::PipelineViewportStateCreateInfo::builder()
        .viewports(&viewports)
        .scissors(&scissors)
        .build()
}

fn get_rasterizer_create_info() -> vk::PipelineRasterizationStateCreateInfo {
    vk::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(vk::PolygonMode::FILL)
        .line_width(1.0)
        .cull_mode(vk::CullModeFlags::BACK)
        .front_face(vk::FrontFace::CLOCKWISE)
        .depth_bias_enable(false)
        .depth_bias_constant_factor(0.0)
        .depth_bias_clamp(0.0)
        .depth_bias_slope_factor(0.0)
        .build()
}

fn get_multisampline_create_info() -> vk::PipelineMultisampleStateCreateInfo {
    vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(vk::SampleCountFlags::TYPE_1)
        .min_sample_shading(1.0)
        // .sample_mask() // null
        .alpha_to_coverage_enable(false)
        .alpha_to_one_enable(false)
        .build()
}

fn get_color_blending_create_info() -> vk::PipelineColorBlendStateCreateInfo {
    let color_blend_attachment = vk::PipelineColorBlendAttachmentState::builder()
        .color_write_mask(vk::ColorComponentFlags::all())
        .blend_enable(false)
        .src_color_blend_factor(vk::BlendFactor::ONE)
        .dst_color_blend_factor(vk::BlendFactor::ZERO)
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
        .alpha_blend_op(vk::BlendOp::ADD)
        .build();
    let color_blend_attachments = [color_blend_attachment];

    vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .logic_op(vk::LogicOp::COPY)
        .attachments(&color_blend_attachments)
        .blend_constants([0.0, 0.0, 0.0, 0.0])
        .build()
}

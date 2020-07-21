use std::ffi::CString;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;

use ash::{version::DeviceV1_0, vk, Device};

use super::swapchain::SwapchainDetails;
use crate::{Material, Renderer};

pub struct Pipeline {
    layout: vk::PipelineLayout,
    graphics_pipeline: vk::Pipeline,
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
            renderer.device.destroy_pipeline_layout(self.layout, None);
        }
    }
}

fn create_colored_vertex_pipeline(renderer: &Renderer) -> Pipeline {
    let vert_code = include_bytes!("shaders/colored_vertex.vert.spv");
    let vert_module = create_shader_module(&renderer.device, vert_code);

    let frag_code = include_bytes!("shaders/colored_vertex.frag.spv");
    let frag_module = create_shader_module(&renderer.device, frag_code);

    let entry_point_name = CString::new("main").unwrap();
    let vertex_shader_stage_info = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::VERTEX)
        .module(vert_module)
        .name(&entry_point_name)
        .build();
    let frag_shader_stage_info = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::FRAGMENT)
        .module(frag_module)
        .name(&entry_point_name)
        .build();

    let shader_stage_infos = [vertex_shader_stage_info, frag_shader_stage_info];

    let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder();
    // .vertex_binding_descriptions() null since vertices are hard coded in the shader
    // .vertex_attribute_descriptions() same here

    let input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);
    let viewport_info = get_viewport_info(&renderer.swapchain_details);
    let rasterizer_info = get_rasterizer_info();
    let multisample_info = get_multisampline_info();
    let color_blending_info = get_color_blending_info();

    let layout = {
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
            // .set_layouts() null since we don't have uniforms in our shaders
            // .push_constant_ranges
            .build();

        unsafe {
            renderer
                .device
                .create_pipeline_layout(&pipeline_layout_info, None)
                .unwrap()
        }
    };

    let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(&shader_stage_infos)
        .vertex_input_state(&vertex_input_info)
        .input_assembly_state(&input_assembly_info)
        .viewport_state(&viewport_info)
        .rasterization_state(&rasterizer_info)
        .multisample_state(&multisample_info)
        // .depth_stencil_state() null since don't make use of depth/stencil tests
        .color_blend_state(&color_blending_info)
        // .dynamic_state() null since don't have any dynamic states
        .layout(layout)
        .render_pass(renderer.render_pass)
        .subpass(0)
        // .base_pipeline_handle() null since it is not derived from another
        // .base_pipeline_index(-1) same
        .build();
    let pipeline_infos = [pipeline_info];

    let graphics_pipeline = unsafe {
        renderer
            .device
            .create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_infos, None)
            .expect("Unable to create graphics pipelin")[0]
    };

    unsafe {
        renderer.device.destroy_shader_module(vert_module, None);
        renderer.device.destroy_shader_module(frag_module, None);
    }
    
    Pipeline {
        layout,
        graphics_pipeline,
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

fn get_viewport_info(details: &SwapchainDetails) -> vk::PipelineViewportStateCreateInfo {
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

fn get_rasterizer_info() -> vk::PipelineRasterizationStateCreateInfo {
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

fn get_multisampline_info() -> vk::PipelineMultisampleStateCreateInfo {
    vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(vk::SampleCountFlags::TYPE_1)
        .min_sample_shading(1.0)
        // .sample_mask() // null
        .alpha_to_coverage_enable(false)
        .alpha_to_one_enable(false)
        .build()
}

fn get_color_blending_info() -> vk::PipelineColorBlendStateCreateInfo {
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

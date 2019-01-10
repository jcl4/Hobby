use super::shaders;
use crate::renderer::ShaderSet;
use crate::Result;
use ash::{version::DeviceV1_0, vk};
use std::ffi::CString;

pub fn create_graphics_pipeline(
    device: ash::Device,
    swap_extent: &vk::Extent2D,
    render_pass: vk::RenderPass,
) -> Result<(vk::Pipeline, vk::PipelineLayout)> {
    let shader_set = ShaderSet::Basic;
    let modules = shaders::shader::get_shader_modules(shader_set, device.clone())?;
    let vert_module = modules[0];
    let frag_module = modules[1];
    let shader_entry_name = CString::new("main").unwrap();

    let shader_stage_create_infos = [
        vk::PipelineShaderStageCreateInfo::builder()
            .name(&shader_entry_name)
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_module)
            .build(),
        vk::PipelineShaderStageCreateInfo::builder()
            .name(&shader_entry_name)
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_module)
            .build(),
    ];

    let binding_descriptions: Vec<vk::VertexInputBindingDescription> = vec![];
    let attribute_descriptions: Vec<vk::VertexInputAttributeDescription> = vec![];

    let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder()
        .vertex_binding_descriptions(&binding_descriptions)
        .vertex_attribute_descriptions(&attribute_descriptions);

    let pipeline_input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
        .primitive_restart_enable(false)
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

    let viewport = vk::Viewport::builder()
        .x(0.0)
        .y(0.0)
        .width(swap_extent.width as f32)
        .height(swap_extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0)
        .build();

    let viewports = [viewport];

    let scissor = vk::Rect2D::builder()
        .extent(swap_extent.clone())
        .offset(vk::Offset2D::builder().x(0).y(0).build())
        .build();

    let scissors = [scissor];

    let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
        .scissors(&scissors)
        .viewports(&viewports);

    let rasterizer = vk::PipelineRasterizationStateCreateInfo::builder()
        .depth_bias_enable(false)
        .front_face(vk::FrontFace::CLOCKWISE)
        .cull_mode(vk::CullModeFlags::BACK)
        .line_width(1.0)
        .polygon_mode(vk::PolygonMode::FILL)
        .rasterizer_discard_enable(false)
        .depth_clamp_enable(false);

    let multi_sampling = vk::PipelineMultisampleStateCreateInfo::builder()
        .rasterization_samples(vk::SampleCountFlags::TYPE_1)
        .sample_shading_enable(false);

    let color_blend_attachment = vk::PipelineColorBlendAttachmentState::builder()
        .blend_enable(false)
        .color_write_mask(vk::ColorComponentFlags::all())
        .build();

    let color_blend_attachments = [color_blend_attachment];

    let color_blending = vk::PipelineColorBlendStateCreateInfo::builder()
        .attachments(&color_blend_attachments)
        .logic_op_enable(false);

    let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::default();

    let pipeline_layout =
        unsafe { device.create_pipeline_layout(&pipeline_layout_create_info, None)? };

    let pipeline_create_info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(&shader_stage_create_infos)
        .subpass(0)
        .render_pass(render_pass)
        .layout(pipeline_layout)
        .color_blend_state(&color_blending)
        .multisample_state(&multi_sampling)
        .rasterization_state(&rasterizer)
        .viewport_state(&viewport_state)
        .input_assembly_state(&pipeline_input_assembly)
        .vertex_input_state(&vertex_input_info);

    let pipeline = unsafe {
        device
            .create_graphics_pipelines(
                vk::PipelineCache::null(),
                &[pipeline_create_info.build()],
                None,
            )
            .unwrap()
    };

    unsafe {
        device.destroy_shader_module(vert_module, None);
        device.destroy_shader_module(frag_module, None);
    }
    Ok((pipeline[0], pipeline_layout))
}

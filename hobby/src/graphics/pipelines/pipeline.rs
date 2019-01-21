use crate::Result;
use ash::{version::DeviceV1_0, vk};

pub(crate) trait Pipeline {
    fn create_pipeline(
        &mut self,
        device: &ash::Device,
        swap_extent: vk::Extent2D,
        render_pass: vk::RenderPass,
        mvp_layout: vk::DescriptorSetLayout,
    ) -> Result<()>;

    fn cleanup(&self, device: &ash::Device) -> Result<()>;

    fn get_pipeline(&self) -> vk::Pipeline;
    fn get_layout(&self) -> vk::PipelineLayout;
}

pub(crate) fn create_graphics_pipeline(
    device: &ash::Device,
    swap_extent: vk::Extent2D,
    render_pass: vk::RenderPass,
    shader_stage_create_infos: &[vk::PipelineShaderStageCreateInfo],
    vertex_input_info: vk::PipelineVertexInputStateCreateInfo,
    descriptor_set_layout: vk::DescriptorSetLayout,
) -> Result<(vk::Pipeline, vk::PipelineLayout)> {
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
        .extent(swap_extent)
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

    let layouts = [descriptor_set_layout];

    let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::builder().set_layouts(&layouts);

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

    Ok((pipeline[0], pipeline_layout))
}

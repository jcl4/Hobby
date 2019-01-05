use crate::Result;
use ash::{version::DeviceV1_0, vk};

pub fn create_render_pass(format: vk::Format, device: ash::Device) -> Result<vk::RenderPass> {
    let color_attachment = vk::AttachmentDescription::builder()
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .store_op(vk::AttachmentStoreOp::STORE)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .samples(vk::SampleCountFlags::TYPE_1)
        .format(format)
        .build();

    let color_attachment_ref = vk::AttachmentReference::builder()
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .attachment(0)
        .build();

    let subpass = vk::SubpassDescription::builder()
        .color_attachments(&[color_attachment_ref])
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .build();

    let dependency = vk::SubpassDependency::builder()
        .dst_access_mask(
            vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        )
        .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(vk::AccessFlags::empty())
        .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_subpass(0)
        .src_subpass(vk::SUBPASS_EXTERNAL)
        .build();

    let attachments = [color_attachment];
    let subpasses = [subpass];
    let dependencies = [dependency];

    let render_pass_create_info = vk::RenderPassCreateInfo::builder()
        .dependencies(&dependencies)
        .attachments(&attachments)
        .subpasses(&subpasses);

    let render_pass = unsafe { device.create_render_pass(&render_pass_create_info, None)? };

    Ok(render_pass)
}

use super::base::QueueData;
use super::swapchain::SwapchainData;
use crate::Result;
use ash;
use ash::{version::DeviceV1_0, vk};

pub fn create_command_pool(
    queue_data: &QueueData,
    device: &ash::Device,
) -> Result<vk::CommandPool> {
    let command_pool_create_info =
        vk::CommandPoolCreateInfo::builder().queue_family_index(queue_data.graphics_queue_family);

    let command_pool = unsafe { device.create_command_pool(&command_pool_create_info, None)? };
    Ok(command_pool)
}

pub fn create_command_buffers(
    command_pool: vk::CommandPool,
    num_buffers: u32,
    render_pass: vk::RenderPass,
    pipeline: vk::Pipeline,
    swapchain_data: &SwapchainData,
    framebuffers: &Vec<vk::Framebuffer>,
    device: &ash::Device,
) -> Result<Vec<vk::CommandBuffer>> {
    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(num_buffers)
        .command_pool(command_pool);

    let command_buffers = unsafe { device.allocate_command_buffers(&allocate_info)? };

    let cb_begin_info =
        vk::CommandBufferBeginInfo::builder().flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

    let rect = vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: swapchain_data.extent,
    };

    let clear = vk::ClearValue {
        color: vk::ClearColorValue {
            float32: [0.0, 0.0, 0.0, 0.0],
        },
    };

    let clear_colors = [clear];

    unsafe {
        for (i, cb) in command_buffers.iter().enumerate() {
            device.begin_command_buffer(*cb, &cb_begin_info)?;

            let rp_begin_info = vk::RenderPassBeginInfo::builder()
                .clear_values(&clear_colors)
                .render_area(rect)
                .framebuffer(framebuffers[i])
                .render_pass(render_pass);

            device.cmd_begin_render_pass(*cb, &rp_begin_info, vk::SubpassContents::INLINE);
            device.cmd_bind_pipeline(*cb, vk::PipelineBindPoint::GRAPHICS, pipeline);
            device.cmd_draw(*cb, 3, 1, 0, 0);
            device.cmd_end_render_pass(*cb);
            device.end_command_buffer(*cb)?;
        }
    }

    Ok(command_buffers)
}

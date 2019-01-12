use super::{QueueData, Renderer, VkMesh};
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
    renderer: &Renderer,
    vk_mesh: &VkMesh,
) -> Result<Vec<vk::CommandBuffer>> {
    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(renderer.swapchain_data.image_views.len() as u32)
        .command_pool(renderer.command_pool);

    let command_buffers = unsafe { renderer.device.allocate_command_buffers(&allocate_info)? };

    let cb_begin_info =
        vk::CommandBufferBeginInfo::builder().flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

    let rect = vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: renderer.swapchain_data.extent,
    };

    let clear = vk::ClearValue {
        color: vk::ClearColorValue {
            float32: [0.2, 0.25, 0.2, 0.0],
        },
    };

    let clear_colors = [clear];

    unsafe {
        for (i, cb) in command_buffers.iter().enumerate() {
            renderer.device.begin_command_buffer(*cb, &cb_begin_info)?;

            let rp_begin_info = vk::RenderPassBeginInfo::builder()
                .clear_values(&clear_colors)
                .render_area(rect)
                .framebuffer(renderer.framebuffers[i])
                .render_pass(renderer.render_pass);

            renderer
                .device
                .cmd_begin_render_pass(*cb, &rp_begin_info, vk::SubpassContents::INLINE);
            renderer.device.cmd_bind_pipeline(
                *cb,
                vk::PipelineBindPoint::GRAPHICS,
                renderer.pipeline,
            );
            vk_mesh.add_draw_cmd(*cb);
            renderer.device.cmd_end_render_pass(*cb);
            renderer.device.end_command_buffer(*cb)?;
        }
    }

    Ok(command_buffers)
}

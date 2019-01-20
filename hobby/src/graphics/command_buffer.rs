use super::{base::QueueData, swapchain::SwapchainData};
use crate::{core::Model, Result};
use ash;
use ash::{version::DeviceV1_0, vk};

pub struct CommandBufferData {
    pub(crate) command_pool: vk::CommandPool,
    pub(crate) command_buffers: Vec<vk::CommandBuffer>,
}

impl CommandBufferData {
    pub(crate) fn new(
        device: &ash::Device,
        queue_data: &QueueData,
        swapchain_data: &SwapchainData,
    ) -> Result<CommandBufferData> {
        let command_pool = create_command_pool(queue_data, device)?;
        let command_buffers = create_command_buffers(device, command_pool, swapchain_data)?;

        Ok(CommandBufferData {
            command_pool,
            command_buffers,
        })
    }

    pub(crate) fn recreate_command_buffers(
        &mut self,
        device: &ash::Device,
        swapchain_data: &SwapchainData,
    ) -> Result<()> {
        self.command_buffers = create_command_buffers(device, self.command_pool, swapchain_data)?;
        Ok(())
    }

    pub(crate) fn build_cb(
        &mut self,
        device: &ash::Device,
        swapchain_data: &SwapchainData,
        framebuffers: &[vk::Framebuffer],
        render_pass: vk::RenderPass,
        models: &[Model],
    ) -> Result<()> {
        let cb_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE);

        let rect = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain_data.extent,
        };

        let clear = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.2, 0.25, 0.2, 0.0],
            },
        };

        let clear_colors = [clear];

        unsafe {
            for (i, cb) in self.command_buffers.iter().enumerate() {
                device.begin_command_buffer(*cb, &cb_begin_info)?;

                let rp_begin_info = vk::RenderPassBeginInfo::builder()
                    .clear_values(&clear_colors)
                    .render_area(rect)
                    .framebuffer(framebuffers[i])
                    .render_pass(render_pass);

                device.cmd_begin_render_pass(*cb, &rp_begin_info, vk::SubpassContents::INLINE);

                for model in models {
                    model.draw(*cb, device)?;
                }
                device.cmd_end_render_pass(*cb);
                device.end_command_buffer(*cb)?;
            }

            Ok(())
        }
    }

    pub(crate) fn cleanup_buffers(&self, device: &ash::Device) {
        unsafe {
            device.free_command_buffers(self.command_pool, &self.command_buffers);
        }
    }

    pub(crate) fn cleanup_command_pool(&self, device: &ash::Device) {
        unsafe {
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}

fn create_command_pool(queue_data: &QueueData, device: &ash::Device) -> Result<vk::CommandPool> {
    let command_pool_create_info =
        vk::CommandPoolCreateInfo::builder().queue_family_index(queue_data.graphics_queue_family);

    let command_pool = unsafe { device.create_command_pool(&command_pool_create_info, None)? };
    Ok(command_pool)
}

pub fn create_command_buffers(
    device: &ash::Device,
    command_pool: vk::CommandPool,
    swapchain_data: &SwapchainData,
) -> Result<Vec<vk::CommandBuffer>> {
    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(swapchain_data.image_views.len() as u32)
        .command_pool(command_pool);

    let command_buffers = unsafe { device.allocate_command_buffers(&allocate_info)? };

    Ok(command_buffers)
}

//     Ok(command_buffers)
// }

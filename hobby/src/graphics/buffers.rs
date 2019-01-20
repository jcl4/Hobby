use super::Renderer;
use crate::Result;
use ash::{version::DeviceV1_0, vk};
use log::info;

pub(crate) fn create_buffer(
    renderer: &Renderer,
    size: vk::DeviceSize,
    usage: vk::BufferUsageFlags,
    mem_properties: vk::MemoryPropertyFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory)> {
    let buffer_info = vk::BufferCreateInfo::builder()
        .size(size)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .usage(usage);

    let buffer;
    let buffer_memory;

    unsafe {
        buffer = renderer.device.create_buffer(&buffer_info, None)?;
        let mem_requirements = renderer.device.get_buffer_memory_requirements(buffer);
        info!(
            "Memory Requirements for Vertex Buffer: {:?}",
            mem_requirements
        );

        let memory_type =
            renderer.find_memory_type(mem_requirements.memory_type_bits, mem_properties)?;

        let alloc_info = vk::MemoryAllocateInfo::builder()
            .memory_type_index(memory_type)
            .allocation_size(mem_requirements.size);

        buffer_memory = renderer.device.allocate_memory(&alloc_info, None)?;
        renderer
            .device
            .bind_buffer_memory(buffer, buffer_memory, 0)?;
    }

    Ok((buffer, buffer_memory))
}

pub(crate) fn copy_buffer(
    renderer: &Renderer,
    src_buffer: vk::Buffer,
    dst_buffer: vk::Buffer,
    size: vk::DeviceSize,
) -> Result<()> {
    let alloc_info = vk::CommandBufferAllocateInfo::builder()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1)
        .command_pool(renderer.command_buffer_data.command_pool);

    unsafe {
        let command_buffers = renderer.device.allocate_command_buffers(&alloc_info)?;
        let cb = command_buffers[0];
        let cb_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        renderer.device.begin_command_buffer(cb, &cb_begin_info)?;

        let copy_region = vk::BufferCopy::builder()
            .src_offset(0)
            .dst_offset(0)
            .size(size)
            .build();

        let copy_regions = [copy_region];

        renderer
            .device
            .cmd_copy_buffer(cb, src_buffer, dst_buffer, &copy_regions);
        renderer.device.end_command_buffer(cb)?;

        let submit_info = vk::SubmitInfo::builder().command_buffers(&[cb]).build();

        renderer.device.queue_submit(
            renderer.queue_data.graphics_queue,
            &[submit_info],
            vk::Fence::null(),
        )?;

        renderer
            .device
            .queue_wait_idle(renderer.queue_data.graphics_queue)?;

        renderer
            .device
            .free_command_buffers(renderer.command_buffer_data.command_pool, &[cb]);
    }

    Ok(())
}

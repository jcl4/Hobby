use crate::{
    graphics::{buffers, Renderer},
    na, Result,
};
use ash::{version::DeviceV1_0, vk};
use log::debug;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub(crate) struct Mvp {
    pub(crate) model: na::Matrix4<f32>,
    pub(crate) view: na::Matrix4<f32>,
    pub(crate) proj: na::Matrix4<f32>,
}

#[derive(Default)]
pub(crate) struct MvpBuffers {
    pub(crate) buffers: Vec<vk::Buffer>,
    pub(crate) buffers_memory: Vec<vk::DeviceMemory>,
}

// impl Default for Mvp {
//     fn default() -> Mvp {
//         let model = na::Matrix4::identity();
//         let view = na::Matrix4::identity();
//         let proj = na::Matrix4::identity();
//         Mvp { model, view, proj }
//     }
// }

impl MvpBuffers {
    pub(crate) fn build(&mut self, renderer: &Renderer) -> Result<()> {
        self.create_buffers(renderer)?;
        Ok(())
    }

    fn create_buffers(&mut self, renderer: &Renderer) -> Result<()> {
        let num_buffers = renderer.swapchain_data.image_views.len();

        let buffer_size = std::mem::size_of::<Mvp>();

        self.buffers = vec![];
        self.buffers_memory = vec![];

        for _ in 0..num_buffers {
            let (buffer, buffers_memory) = buffers::create_buffer(
                renderer,
                buffer_size as u64,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            )?;

            self.buffers.push(buffer);
            self.buffers_memory.push(buffers_memory);
        }

        Ok(())
    }

    pub(crate) fn cleanup(&self, device: &ash::Device) {
        unsafe {
            for (buffer, buffer_memory) in self.buffers.iter().zip(self.buffers_memory.iter()) {
                device.destroy_buffer(*buffer, None);
                device.free_memory(*buffer_memory, None);
            }
        }
    }
}

pub(crate) fn create_mvp_layout(
    device: &ash::Device,
    binding: u32,
) -> Result<vk::DescriptorSetLayout> {
    let mvp_layout_binding = vk::DescriptorSetLayoutBinding::builder()
        .stage_flags(vk::ShaderStageFlags::VERTEX)
        .descriptor_count(1)
        .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
        .binding(binding)
        .build();

    let bindings = [mvp_layout_binding];

    let layout_info = vk::DescriptorSetLayoutCreateInfo::builder().bindings(&bindings);

    let mvp_layout;
    unsafe {
        mvp_layout = device.create_descriptor_set_layout(&layout_info, None)?;
    }

    debug!("Descriptor Layout Created");

    Ok(mvp_layout)
}

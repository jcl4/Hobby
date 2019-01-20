use crate::{
    core::Vertex,
    graphics::{buffers, Renderer},
    Result,
};
use ash::{util::Align, version::DeviceV1_0, vk, Device};
use log::debug;
use std::mem;

pub struct VkMesh {
    device: Device,
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    index_buffer: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,
    num_indices: u32,
}

pub(crate) trait VkVertex {
    fn get_binding_description() -> Vec<vk::VertexInputBindingDescription>;
    fn get_attribute_descriptions() -> Vec<vk::VertexInputAttributeDescription>;
    fn check_vertex(vertex: &Vertex) -> Result<()>;
    fn from_vertex(vertex: &Vertex) -> Self;
}

impl VkMesh {
    #![allow(clippy::new_ret_no_self)]
    pub(crate) fn new<T>(
        renderer: &Renderer,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Result<VkMesh>
    where
        T: VkVertex + Copy,
    {
        let device = renderer.device.clone();

        let (vertex_buffer, vertex_buffer_memory) = create_vertex_buffer::<T>(renderer, vertices)?;
        let (index_buffer, index_buffer_memory) = create_index_buffer(renderer, &indices)?;
        Ok(VkMesh {
            device,
            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
            num_indices: indices.len() as u32,
        })
    }

    pub fn draw(&self, cb: vk::CommandBuffer) {
        let vert_buffers = [self.vertex_buffer];
        let offsets = [0];
        unsafe {
            self.device
                .cmd_bind_vertex_buffers(cb, 0, &vert_buffers, &offsets);
            self.device
                .cmd_bind_index_buffer(cb, self.index_buffer, 0, vk::IndexType::UINT32);
            self.device
                .cmd_draw_indexed(cb, self.num_indices, 1, 0, 0, 0);
        }
    }

    pub fn cleanup(&self) {
        debug!("VkMesh Cleaned up");
        unsafe {
            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device.free_memory(self.vertex_buffer_memory, None);
            self.device.destroy_buffer(self.index_buffer, None);
            self.device.free_memory(self.index_buffer_memory, None);
        }
    }
}

fn create_index_buffer(
    renderer: &Renderer,
    indices: &[u32],
) -> Result<(vk::Buffer, vk::DeviceMemory)> {
    let size = (mem::size_of::<u32>() * indices.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = buffers::create_buffer(
        renderer,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    )?;

    let index_buffer_data = unsafe {
        let data_ptr = renderer.device.map_memory(
            staging_buffer_memory,
            0,
            size,
            vk::MemoryMapFlags::empty(),
        )?;

        let mut index_align = Align::new(data_ptr, mem::align_of::<u32>() as u64, size);
        index_align.copy_from_slice(&indices);

        renderer.device.unmap_memory(staging_buffer_memory);

        let (index_buffer, index_buffer_memory) = buffers::create_buffer(
            renderer,
            size,
            vk::BufferUsageFlags::INDEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        buffers::copy_buffer(renderer, staging_buffer, index_buffer, size)?;

        renderer.device.destroy_buffer(staging_buffer, None);
        renderer.device.free_memory(staging_buffer_memory, None);

        (index_buffer, index_buffer_memory)
    };

    Ok(index_buffer_data)
}

fn create_vertex_buffer<T>(
    renderer: &Renderer,
    vertices: &[Vertex],
) -> Result<(vk::Buffer, vk::DeviceMemory)>
where
    T: VkVertex + Copy,
{
    T::check_vertex(&vertices[0])?;

    let vertices: Vec<T> = vertices
        .iter()
        .map(|vertex| T::from_vertex(vertex))
        .collect();

    let stride = mem::size_of::<T>();
    let size = (stride * vertices.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = buffers::create_buffer(
        renderer,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    )?;

    let vertex_buffer_data = unsafe {
        let data_ptr = renderer.device.map_memory(
            staging_buffer_memory,
            0,
            size,
            vk::MemoryMapFlags::empty(),
        )?;

        let mut vert_align = Align::new(data_ptr, mem::align_of::<T>() as u64, size);
        vert_align.copy_from_slice(&vertices);

        renderer.device.unmap_memory(staging_buffer_memory);

        let (vertex_buffer, vertex_buffer_memory) = buffers::create_buffer(
            renderer,
            size,
            vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        buffers::copy_buffer(renderer, staging_buffer, vertex_buffer, size)?;

        renderer.device.destroy_buffer(staging_buffer, None);
        renderer.device.free_memory(staging_buffer_memory, None);

        (vertex_buffer, vertex_buffer_memory)
    };

    Ok(vertex_buffer_data)
}

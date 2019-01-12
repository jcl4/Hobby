use crate::renderer::{buffers, Renderer};
use crate::Result;
use ash::{util::Align, version::DeviceV1_0, vk, Device};
use log::info;
use memoffset::offset_of;
use std::mem;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn get_binding_description() -> [vk::VertexInputBindingDescription; 1] {
        let input_rate = vk::VertexInputRate::VERTEX;
        let binding = vk::VertexInputBindingDescription::builder()
            .stride(mem::size_of::<Self>() as u32)
            .binding(0)
            .input_rate(input_rate)
            .build();
        [binding]
    }

    pub fn get_attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let pos_description = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(offset_of!(Vertex, position) as u32)
            .build();

        let color_description = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(offset_of!(Vertex, color) as u32)
            .build();

        [pos_description, color_description]
    }
}

struct Triangle {
    vertices: Vec<Vertex>,
}

impl Triangle {
    fn new() -> Triangle {
        let vertices = vec![
            Vertex {
                position: [0.0, -0.5, 0.0],
                color: [1.0, 1.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];
        Triangle { vertices }
    }
}

pub struct VkMesh {
    device: Device,
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    vertices: Vec<Vertex>,
}

impl VkMesh {
    #![allow(clippy::new_ret_no_self)]
    pub fn new(renderer: &Renderer) -> Result<VkMesh> {
        let device = renderer.device.clone();

        // Todo: Temporary implimentation
        let triangle = Triangle::new();
        let vertices = triangle.vertices;
        let size = mem::size_of_val(&vertices) as u64;

        let (staging_buffer, staging_buffer_memory) = buffers::create_buffer(
            renderer,
            size,
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        let (vertex_buffer, vertex_buffer_memory) = unsafe {
            let data_ptr =
                device.map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())?;

            let mut vert_align = Align::new(data_ptr, mem::align_of::<Vertex>() as u64, size);
            vert_align.copy_from_slice(&vertices);

            device.unmap_memory(staging_buffer_memory);

            let (vertex_buffer, vertex_buffer_memory) = buffers::create_buffer(
                renderer,
                size,
                vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )?;

            buffers::copy_buffer(renderer, staging_buffer, vertex_buffer, size)?;

            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);

            (vertex_buffer, vertex_buffer_memory)
        };

        Ok(VkMesh {
            device,
            vertex_buffer,
            vertex_buffer_memory,
            vertices,
        })
    }

    pub fn add_draw_cmd(&self, cb: vk::CommandBuffer) {
        let vert_buffers = [self.vertex_buffer];
        let offsets = [0];
        unsafe {
            self.device
                .cmd_bind_vertex_buffers(cb, 0, &vert_buffers, &offsets);
            self.device
                .cmd_draw(cb, self.vertices.len() as u32, 1, 0, 0);
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device.free_memory(self.vertex_buffer_memory, None);
        }
    }
}

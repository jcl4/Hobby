use super::Context;
use bytemuck::Pod;

pub fn create_vertex_index_buffers<T: Pod>(
    vertices: &[T],
    indices: &[u16],
    context: &Context,
) -> (wgpu::Buffer, wgpu::Buffer) {
    let vertex_buffer = context
        .device
        .create_buffer_with_data(bytemuck::cast_slice(vertices), wgpu::BufferUsage::VERTEX);

    let index_buffer = context
        .device
        .create_buffer_with_data(bytemuck::cast_slice(indices), wgpu::BufferUsage::INDEX);

    (vertex_buffer, index_buffer)
}

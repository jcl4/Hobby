use std::mem;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BasicVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl BasicVertex {
    pub(crate) fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<BasicVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

unsafe impl bytemuck::Pod for BasicVertex {}
unsafe impl bytemuck::Zeroable for BasicVertex {}

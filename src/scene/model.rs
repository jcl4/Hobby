use super::Material;
use crate::renderer::pipelines::BasicVertex;
use crate::Renderer;

pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    num_indices: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Mesh {
        let num_indices = indices.len() as u32;
        Mesh {
            vertices,
            indices,
            num_indices,
        }
    }
}

pub struct Model {
    pub(crate) mesh: Mesh,
    pub(crate) material: Material,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
}

impl Model {
    pub fn new(mesh: Mesh, material: Material, renderer: &mut Renderer) -> Model {
        let vertices = match material {
            Material::Basic => {
                let vertices: Vec<_> = mesh
                    .vertices
                    .iter()
                    .map(|vert| BasicVertex {
                        position: vert.position,
                        color: vert.color,
                    })
                    .collect();
                vertices
            }
        };

        let vertex_buffer = renderer
            .device
            .create_buffer_with_data(bytemuck::cast_slice(&vertices), wgpu::BufferUsage::VERTEX);
        let index_buffer = renderer.device.create_buffer_with_data(
            bytemuck::cast_slice(&mesh.indices),
            wgpu::BufferUsage::INDEX,
        );

        Model {
            mesh,
            material,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        render_pass.set_index_buffer(&self.index_buffer, 0, 0);
        render_pass.draw_indexed(0..self.mesh.num_indices, 0, 0..1);
    }
}

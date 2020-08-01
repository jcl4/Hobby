use super::Material;
use crate::renderer::pipelines::BasicVertex;
use crate::Renderer;
use std::mem;

pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    num_vertices: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Mesh {
        let num_vertices = vertices.len() as u32;
        Mesh {
            vertices,
            indices,
            num_vertices,
        }
    }
}

pub struct Model {
    pub(crate) mesh: Mesh,
    pub(crate) material: Material,
    pub(crate) vertex_buffer: wgpu::Buffer,
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

        Model {
            mesh,
            material,
            vertex_buffer,
        }
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.draw(0..self.mesh.num_vertices, 0..1);
    }
}

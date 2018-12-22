use crate::core::Vertex;
use std::sync::Arc;
use vulkano::buffer::{BufferAccess, TypedBufferAccess};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    vertex_buffer: Option<Arc<BufferAccess + Send + Sync>>,
    index_buffer: Option<Arc<TypedBufferAccess<Content = [u32]> + Send + Sync>>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Mesh {
        Mesh {
            vertices,
            indices,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub fn set_vertex_buffer(&mut self, vertex_buffer: Arc<BufferAccess + Send + Sync>) {
        self.vertex_buffer = Some(vertex_buffer);
    }

    pub fn set_index_buffer(
        &mut self,
        index_buffer: Arc<TypedBufferAccess<Content = [u32]> + Send + Sync>,
    ) {
        self.index_buffer = Some(index_buffer);
    }

    pub fn vertex_buffer(&self) -> Arc<BufferAccess + Send + Sync> {
        self.vertex_buffer.clone().unwrap()
    }

    pub fn index_buffer(&self) -> Arc<TypedBufferAccess<Content = [u32]> + Send + Sync> {
        self.index_buffer.clone().unwrap()
    }
}

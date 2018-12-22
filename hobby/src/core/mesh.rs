use crate::core::Vertex;
use std::sync::Arc;
use vulkano::buffer::{BufferAccess, TypedBufferAccess};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub(crate) vertex_buffer: Option<Arc<BufferAccess + Send + Sync>>,
    pub(crate) index_buffer: Option<Arc<TypedBufferAccess<Content = [u32]> + Send + Sync>>,
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
}

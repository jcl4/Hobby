use super::vertex::Vertex;

pub struct Mesh {
	vertices: Vec<Vertex>,
	indices: Vec<u16>,
}

impl Mesh {
	pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
		Mesh {
			vertices,
			indices
		}
	}
}
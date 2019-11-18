mod object;
mod mesh;
mod vertex;

pub use object::{ObjectBuilder, Object};
pub use mesh::Mesh;

pub struct Scene {}

impl Scene {
	pub fn new() -> Scene {
		Scene{}
	}

	pub fn add_object(&mut self, object: Object) {
		
	}
}
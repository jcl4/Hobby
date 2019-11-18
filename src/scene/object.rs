use crate::Application;
use crate::Mesh;

pub struct ObjectBuilder{}

impl ObjectBuilder {
	pub fn new() -> Self{
		ObjectBuilder {}
	}

	pub fn with_mesh(self, mesh: Mesh) -> Self {
		self
	}

	pub fn with_transform(self) -> Self {
		self
	}

	pub fn with_material(self) -> Self {
		self
	}

	pub fn build(self, app: &Application) -> Object {
		Object{}
	}

}

pub struct Object {} 
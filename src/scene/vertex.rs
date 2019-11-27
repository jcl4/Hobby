
pub struct VertexBuilder{
	pos: [f32; 3],
	color: Option<[f32; 4]>,
}

impl VertexBuilder{
	pub fn new(pos: [f32; 3]) -> Self {
		VertexBuilder{
			pos,
			color: None,
		}
	}

	pub fn with_color(mut self, color: [f32; 4]) -> Self {
		self.color = Some(color);
		self
	}

	pub fn build(self) -> Vertex {
		Vertex{
			pos: self.pos,
			color: self.color,
		}
	}
}

pub struct Vertex {
	pub(crate) pos: [f32; 3],
	pub(crate) color: Option<[f32; 4]>,
}
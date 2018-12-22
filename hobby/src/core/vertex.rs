pub struct Vertex {
    pub position: [f32; 3],
    pub color: Option<[f32; 4]>,
}

impl Vertex {
    pub fn builder() -> VertexBuilder {
        VertexBuilder::new()
    }
}

pub struct VertexBuilder {
    position: [f32; 3],
    color: Option<[f32; 4]>,
}

impl VertexBuilder {
    pub fn new() -> VertexBuilder {
        let vertex = Vertex::default();
        VertexBuilder {
            position: vertex.position,
            color: vertex.color,
        }
    }

    pub fn with_position(self, position: [f32; 3]) -> VertexBuilder {
        VertexBuilder {
            position: position,
            color: self.color,
        }
    }

    pub fn with_color(self, color: [f32; 4]) -> VertexBuilder {
        VertexBuilder {
            position: self.position,
            color: Some(color),
        }
    }

    pub fn build(self) -> Vertex {
        Vertex {
            position: self.position,
            color: self.color,
        }
    }
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            position: [0.0, 0.0, 0.0],
            color: None,
        }
    }
}

#[derive(Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: Option<[f32; 4]>,
    pub normal: Option<[f32; 3]>,
    pub tangent: Option<[f32; 4]>,
    pub tex_coord: Option<[f32; 2]>,
}

impl Vertex {
    pub fn builder() -> VertexBuilder {
        VertexBuilder::new()
    }
}

pub struct VertexBuilder {
    position: [f32; 3],
    color: Option<[f32; 4]>,
    normal: Option<[f32; 3]>,
    tangent: Option<[f32; 4]>,
    tex_coord: Option<[f32; 2]>,
}

impl VertexBuilder {
    pub fn new() -> VertexBuilder {
        let vertex = Vertex::default();
        VertexBuilder {
            position: vertex.position,
            color: vertex.color,
            normal: vertex.normal,
            tangent: vertex.tangent,
            tex_coord: vertex.tex_coord,
        }
    }

    pub fn with_position(self, position: [f32; 3]) -> VertexBuilder {
        VertexBuilder {
            position: position,
            color: self.color,
            normal: self.normal,
            tangent: self.tangent,
            tex_coord: self.tex_coord,
        }
    }

    pub fn with_color(self, color: [f32; 4]) -> VertexBuilder {
        VertexBuilder {
            position: self.position,
            color: Some(color),
            normal: self.normal,
            tangent: self.tangent,
            tex_coord: self.tex_coord,
        }
    }

    pub fn with_normal(self, normal: [f32; 3]) -> VertexBuilder {
        VertexBuilder {
            position: self.position,
            color: self.color,
            normal: Some(normal),
            tangent: self.tangent,
            tex_coord: self.tex_coord,
        }
    }

    pub fn with_tangent(self, tangent: [f32; 4]) -> VertexBuilder {
        VertexBuilder {
            position: self.position,
            color: self.color,
            normal: self.normal,
            tangent: Some(tangent),
            tex_coord: self.tex_coord,
        }
    }

    pub fn with_tex_coord(self, tex_coord: [f32; 2]) -> VertexBuilder {
        VertexBuilder {
            position: self.position,
            color: self.color,
            normal: self.normal,
            tangent: self.tangent,
            tex_coord: Some(tex_coord),
        }
    }

    pub fn build(self) -> Vertex {
        Vertex {
            position: self.position,
            color: self.color,
            normal: self.normal,
            tangent: self.tangent,
            tex_coord: self.tex_coord,
        }
    }
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            position: [0.0, 0.0, 0.0],
            color: None,
            normal: None,
            tangent: None,
            tex_coord: None,
        }
    }
}

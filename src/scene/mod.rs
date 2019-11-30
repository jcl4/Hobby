mod mesh;
mod object;
mod vertex;

pub use mesh::Mesh;
pub use object::{Object, ObjectBuilder};
pub use vertex::{Vertex, VertexBuilder};

pub(crate) use object::ObjectBufferGroup;

#[derive(Debug, Default)]
pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: vec![] }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn update(&mut self) {
        self.objects.iter_mut().for_each(|object| object.update());
    }
}

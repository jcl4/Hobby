pub mod material;
pub mod mesh;

use material::Material;
use crate::{Pipeline, Renderer};

pub struct Model {
    // pub mesh: mesh::Mesh
    pub(crate) pipeline: Pipeline,

}


impl Model{
    pub fn new(material: &Material, renderer: &Renderer) -> Model {
        let pipeline = Pipeline::new(material, renderer);
         Model{pipeline}
    }

    pub fn cleanup(&self, renderer: &Renderer) {
        self.pipeline.cleanup(renderer)
    }
}

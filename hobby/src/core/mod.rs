mod mesh;
mod model;
mod vertex;

pub use self::mesh::Mesh;
pub use self::model::Model;
pub use self::vertex::Vertex;



pub enum MaterialType {
    Basic,
}
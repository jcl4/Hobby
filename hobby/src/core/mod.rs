mod mesh;
mod model;
mod texture;
mod transform;
mod vertex;

pub use self::mesh::Mesh;
pub use self::model::Model;
pub use self::texture::Texture;
pub use self::transform::Transform;
pub use self::vertex::Vertex;

pub enum MaterialType {
    Basic,
    Textured(Texture),
}

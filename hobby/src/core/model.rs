use crate::core::{MaterialType, Mesh};
use crate::glm;

pub struct Model {
    pub mesh: Mesh,
    pub material_type: MaterialType,
    transform: glm::TMat4<f32>,
}

impl Model {
    pub fn new(mesh: Mesh, material_type: MaterialType) -> Model {
        let scale_vec = glm::vec3(1.0, 1.0, 1.0);
        let scale = glm::scaling(&scale_vec);

        Model {
            mesh,
            material_type,
            transform: scale,
        }
    }
}

use crate::glm;

pub struct Transform {
    model: glm::TMat4<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        let scale = glm::vec3(0.5, 0.5, 0.5);
        let model = glm::scaling(&scale);
        // println!("Model: {:?}", model);
        Transform { model }
    }

    pub fn array(&self) -> [[f32; 4]; 4] {
        self.model.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Transform;

    #[test]
    fn test_array() {
        let transform = Transform::new();
        let array = transform.array();

        let test_array = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        assert_eq!(array, test_array);
    }
}

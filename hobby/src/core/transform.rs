use crate::na;

#[derive(Clone, Debug)]
pub struct Transform {
    position: na::Vector3<f32>,
    scale: na::Vector3<f32>,
    orientation: na::UnitQuaternion<f32>,
    model: Option<na::Matrix4<f32>>,
}

impl Transform {
    pub fn new(
        position: na::Vector3<f32>,
        scale: na::Vector3<f32>,
        orientation: na::UnitQuaternion<f32>,
    ) -> Transform {
        let mut transform = Transform {
            position,
            scale,
            orientation,
            model: None,
        };

        transform.calc_model_matrix();
        transform
    }

    pub fn set_scale(&mut self, scale: na::Vector3<f32>) {
        self.scale = scale;
        self.model = None;
    }

    pub fn scale(&mut self, scale: na::Vector3<f32>) {
        self.scale.component_mul_assign(&scale);
        self.model = None;
    }

    pub fn set_orientation(&mut self, orientation: na::UnitQuaternion<f32>) {
        self.orientation = orientation;
        self.model = None;
    }

    pub fn rotate(&mut self, rotation: na::UnitQuaternion<f32>) {
        self.orientation *= rotation;
        self.model = None;
    }

    pub fn get_orientation(&self) -> &na::UnitQuaternion<f32> {
        &self.orientation
    }

    pub fn set_position(&mut self, position: na::Vector3<f32>) {
        self.position = position;
        self.model = None;
    }

    pub fn translate(&mut self, translation: na::Vector3<f32>) {
        self.position += translation;
        self.model = None;
    }

    pub fn array(&mut self) -> [[f32; 4]; 4] {
        self.calc_model_matrix();
        self.model.unwrap().into()
    }

    pub fn get_model_matrix(&mut self) -> na::Matrix4<f32> {
        self.calc_model_matrix();
        self.model.unwrap()
    }

    pub fn transform_vector(&mut self, vector: &na::Vector3<f32>) -> na::Vector3<f32> {
        let vec = na::Vector4::new(vector[0], vector[1], vector[2], 1.0);
        self.calc_model_matrix();
        let result = self.model.unwrap() * vec;
        na::Vector3::new(result[0], result[1], result[2])
    }

    fn calc_model_matrix(&mut self) {
        //  Scale, rotate, translate

        if self.model.is_none() {
            let mut model = na::Matrix4::new_nonuniform_scaling(&self.scale);
            let rotation = na::Matrix4::from(self.orientation.to_rotation_matrix());
            model = rotation * model;
            model.append_translation_mut(&self.position);

            self.model = Some(model);
        }
    }
}

impl Default for Transform {
    fn default() -> Transform {
        let position = na::Vector3::new(0.0, 0.0, 0.0);
        let scale = na::Vector3::new(1.0, 1.0, 1.0);
        let orientation = na::UnitQuaternion::from_axis_angle(&na::Vector3::z_axis(), 0.0);

        let mut transform = Transform {
            position,
            scale,
            orientation,
            model: None,
        };

        transform.calc_model_matrix();

        transform
    }
}

#[cfg(test)]
mod tests {
    use crate::core::Transform;
    use crate::glm;
    use crate::na;

    #[test]
    fn test_default() {
        let mut transform = Transform::default();
        let array = transform.array();

        let test_array = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        assert_eq!(array, test_array);
    }

    #[test]
    fn test_scale() {
        let test_vec = na::Vector3::new(1.0, 1.0, 1.0);

        let scale_vec = na::Vector3::new(0.5, 0.5, 0.5);
        let mut answer = scale_vec;

        let mut transform = Transform::default();
        transform.set_scale(scale_vec);
        let mut result_vec = transform.transform_vector(&test_vec);

        let result_test = glm::equal(&result_vec, &answer);
        let true_vec = na::Vector3::new(true, true, true);
        assert_eq!(result_test, na::Vector3::new(true, true, true));

        transform.scale(scale_vec);
        result_vec = transform.transform_vector(&test_vec);
        answer = na::Vector3::new(0.25, 0.25, 0.25);
        let result = glm::equal(&result_vec, &answer);

        assert_eq!(result, true_vec);
    }

    #[test]
    fn test_rotate() {
        let test_vec = na::Vector3::new(1.0, 0.0, 0.0);
        let orientation: na::UnitQuaternion<f32> =
            na::UnitQuaternion::from_axis_angle(&na::Vector3::z_axis(), glm::quarter_pi());
        let mut answer = na::Vector3::new(
            glm::root_two::<f32>() / 2.0,
            glm::root_two::<f32>() / 2.0,
            0.0,
        );
        let true_vec = na::Vector3::new(true, true, true);

        let mut transform = Transform::default();
        transform.set_orientation(orientation);
        let mut result_vec = transform.transform_vector(&test_vec);
        let mut result_test = glm::equal_eps(&result_vec, &answer, 0.000001);
        assert_eq!(result_test, true_vec);

        transform.rotate(orientation);
        answer = na::Vector3::new(0.0, 1.0, 0.0);
        result_vec = transform.transform_vector(&test_vec);
        result_test = glm::equal_eps(&result_vec, &answer, 0.000001);
        assert_eq!(result_test, true_vec);
    }

    #[test]
    fn test_translation() {
        let test_vec = na::Vector3::new(1.0, 1.0, 0.0);
        let position = na::Vector3::new(1.0, 0.0, 0.0);
        let mut answer = na::Vector3::new(2.0, 1.0, 0.0);

        let true_vec = na::Vector3::new(true, true, true);

        let mut transform = Transform::default();
        transform.set_position(position);
        let mut results_vec = transform.transform_vector(&test_vec);
        let mut result_test = glm::equal(&results_vec, &answer);
        assert_eq!(result_test, true_vec);

        let translate = na::Vector3::new(1.0, 1.0, 1.0);
        transform.translate(translate);
        answer = na::Vector3::new(3.0, 2.0, 1.0);
        results_vec = transform.transform_vector(&test_vec);
        result_test = glm::equal(&results_vec, &answer);
        assert_eq!(result_test, true_vec);
    }

    #[test]
    fn test_combined() {
        let mut transform = Transform::default();
        let rotate = na::UnitQuaternion::from_axis_angle(&na::Vector3::x_axis(), glm::half_pi());
        let scale = na::Vector3::new(2.0, 4.0, 1.0);
        let translate = na::Vector3::new(-1.0, 1.0, -1.0);

        transform.scale(scale);
        transform.rotate(rotate);
        transform.translate(translate);

        let test_vec = na::Vector3::new(0.5, 0.25, 1.0);
        let answer = na::Vector3::new(0.0, 0.0, 0.0);
        let result = transform.transform_vector(&test_vec);
        println!("Transformed Vector: {}", &result);

        let true_vec = na::Vector3::new(true, true, true);
        let result_test = glm::equal_eps(&result, &answer, 0.0000001);
        assert_eq!(result_test, true_vec);
    }

}

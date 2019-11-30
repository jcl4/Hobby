use super::{Matrix4, Quaternion, Vector3, Vector4};

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3,
    pub orientation: Quaternion,
    pub scale: Vector3,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            position: Vector3::zero(),
            orientation: Quaternion::zero(),
            scale: Vector3::one(),
        }
    }
}

impl Transform {
    // rotates orienation by quaternion relative to orientation
    pub fn rotate(&mut self, rotation: Quaternion) {
        self.orientation = self.orientation * rotation;
    }

    pub fn translate(&mut self, translation: Vector3) {
        self.position += translation;
    }

    pub fn scale(&mut self, scale_factor: Vector3) {
        self.scale *= scale_factor;
    }

    pub fn transform_vec(&self, vec_in: Vector3) -> Vector3 {
        let vec = Vector4::from_vec3(vec_in, 1.0);
        let mat = self.get_model_matrix();
        let vec = mat * vec;
        Vector3::new(vec[0], vec[1], vec[2])
    }

    pub fn get_model_matrix(&self) -> Matrix4 {
        let scale_mat = Matrix4::from_scale(self.scale);
        let rot_mat = Matrix4::from_rotation(self.orientation);
        let trans_mat = Matrix4::from_translation(self.position);
        trans_mat * rot_mat * scale_mat
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Degree;

    #[test]
    fn transform_rotate() {
        let mut transform = Transform::default();
        let orientation =
            Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Degree::new(90.0));
        transform.orientation = orientation;

        let rotation = Quaternion::from_axis_angle(Vector3::new(1.0, 0.0, 0.0), Degree::new(90.0));

        transform.rotate(rotation);
        transform.orientation.normalize();

        let quat_test =
            Quaternion::from_axis_angle(Vector3::new(1.0, 1.0, 1.0), Degree::new(120.0));

        assert_eq!(transform.orientation, quat_test);
    }

    #[test]
    fn transform_translate() {
        let translation = Vector3::new(1.0, 2.0, 3.0);

        let mut transform = Transform::default();
        transform.translate(translation);

        let vec = Vector3::one();
        let vec = transform.transform_vec(vec);

        let test_vec = Vector3::new(2.0, 3.0, 4.0);

        assert_eq!(vec, test_vec);
    }

    #[test]
    fn transform_scale() {
        let mut transform = Transform::default();
        let scale = Vector3::new(1.0, 2.0, 3.0);
        transform.scale(scale);

        let vec = Vector3::one();
        let vec = transform.transform_vec(vec);

        let test_vec = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(vec, test_vec);
    }

    #[test]
    fn transform_rotation() {
        let mut transform = Transform::default();
        let orientation =
            Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Degree::new(45.0));
        transform.orientation = orientation;

        let vec = Vector3::new(1.0, 0.0, 3.0);
        let vec = transform.transform_vec(vec);

        let test_vec = Vector3::new(
            std::f32::consts::FRAC_1_SQRT_2,
            std::f32::consts::FRAC_1_SQRT_2,
            3.0,
        );

        assert_eq!(vec, test_vec);
    }

    #[test]
    fn transform() {
        let mut transform = Transform::default();
        let orientation =
            Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Degree::new(45.0));
        let scale = Vector3::new(2.0, 2.0, 2.0);
        let translation = Vector3::new(1.0, 1.0, 1.0);

        transform.translate(translation);
        transform.scale(scale);
        transform.orientation = orientation;

        let vec = Vector3::new(0.0, 0.5, 1.0);
        let vec = transform.transform_vec(vec);

        let test_vec = Vector3::new(
            -std::f32::consts::FRAC_1_SQRT_2 + 1.0,
            std::f32::consts::FRAC_1_SQRT_2 + 1.0,
            3.0,
        );

        assert_eq!(vec, test_vec);
    }
}

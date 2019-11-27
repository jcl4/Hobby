use super::{Quaternion, Vector3};

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Degree;

    #[test]
    fn tranform_rotate() {
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
        let mut transform = Transform::default();
        let translation = Vector3::new(1.0, 2.0, 3.0);
        transform.translate(translation);
        let pos = transform.position;

        let pos_test = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(pos, pos_test);
    }

    #[test]
    fn transform_scale() {
        let mut transform = Transform::default();
        let translation = Vector3::new(1.0, 2.0, 3.0);
        transform.translate(translation);
        let pos = transform.position;

        let pos_test = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(pos, pos_test);
    }
}

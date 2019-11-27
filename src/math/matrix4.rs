use super::{Quaternion, Vector3, Vector4};
use std::ops::{Index, Mul};
// Column Major Axis
#[derive(Debug)]
pub struct Matrix4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
}

impl Matrix4 {
    pub fn from_cols(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Self {
        Matrix4 { x, y, z, w }
    }

    pub fn from_scale(scale: Vector3) -> Self {
        let col_0 = Vector4::new(scale[0], 0.0, 0.0, 0.0);
        let col_1 = Vector4::new(0.0, scale[1], 0.0, 0.0);
        let col_2 = Vector4::new(0.0, 0.0, scale[2], 0.0);
        let col_3 = Vector4::new(0.0, 0.0, 0.0, 1.0);

        Matrix4::from_cols(col_0, col_1, col_2, col_3)
    }

    pub fn from_translation(translation: Vector3) -> Self {
        let col_0 = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let col_1 = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let col_2 = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let col_3 = Vector4::from_vec3(translation, 1.0);

        Matrix4::from_cols(col_0, col_1, col_2, col_3)
    }

    pub fn from_rotation(rotation: Quaternion) -> Matrix4 {
        rotation.into()
    }

    pub fn identity() -> Self {
        let col_0 = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let col_1 = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let col_2 = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let col_3 = Vector4::new(0.0, 0.0, 0.0, 1.0);

        Matrix4::from_cols(col_0, col_1, col_2, col_3)
    }
}

impl Index<u8> for Matrix4 {
    type Output = Vector4;

    fn index(&self, idx: u8) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index {:?}, out of range for Matrix 4", idx),
        }
    }
}

impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, vec: Vector4) -> Self::Output {
        self[0] * vec[0] + self[1] * vec[1] + self[2] * vec[2] + self[3] * vec[3]
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_mat4() {
        let mat4 = Matrix4::identity();
        let col_0 = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let col_1 = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let col_2 = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let col_3 = Vector4::new(0.0, 0.0, 0.0, 1.0);

        assert_eq!(mat4[0], col_0);
        assert_eq!(mat4[1], col_1);
        assert_eq!(mat4[2], col_2);
        assert_eq!(mat4[3], col_3);
    }

    #[test]
    fn mul_mat4_vec4() {
        let mat = Matrix4::identity();
        let vec = Vector4::new(1.0, 1.0, 1.0, 1.0);
        let vec_mul = mat * vec;

        assert_eq!(vec_mul, vec);
    }
}

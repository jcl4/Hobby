use float_cmp::ApproxEq;
use std::ops::{Add, Index, Mul};

use crate::math::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 { x, y, z, w }
    }

    pub fn from_vec3(vec: Vector3, w: f32) -> Self {
        Vector4 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
            w,
        }
    }
}

impl Index<u8> for Vector4 {
    type Output = f32;

    fn index(&self, idx: u8) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index: {:?}, out of range for Vector4", idx),
        }
    }
}

impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Add for Vector4 {
    type Output = Self;

    fn add(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, rhs: &Self) -> bool {
        let eps = 1e-7;
        let ulps = 0;

        self.x.approx_eq(rhs.x, (eps, ulps))
            && self.y.approx_eq(rhs.y, (eps, ulps))
            && self.z.approx_eq(rhs.z, (eps, ulps))
            && self.w.approx_eq(rhs.w, (eps, ulps))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec4_index() {
        let vec = Vector4::new(1.0, 2.0, 3.0, 4.0);

        assert!(vec[0].approx_eq(1.0, (1e-9, 2)));
        assert!(vec[1].approx_eq(2.0, (1e-9, 2)));
        assert!(vec[2].approx_eq(3.0, (1e-9, 2)));
        assert!(vec[3].approx_eq(4.0, (1e-9, 2)));
    }

    #[test]
    fn vec4_mul() {
        let vec = Vector4::new(1.0, 2.0, 3.0, 4.0) * 2.0;
        let vec_test = Vector4::new(2.0, 4.0, 6.0, 8.0);
        assert_eq!(vec, vec_test);
    }

    #[test]
    fn vec4_add() {
        let vec = Vector4::new(1.0, 2.0, 3.0, 4.0) + Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vec_test = Vector4::new(2.0, 4.0, 6.0, 8.0);
        assert_eq!(vec, vec_test);
    }
}

use float_cmp::ApproxEq;
use std::ops::{Add, AddAssign, Index, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Self {
        Vector3::new(1.0, 1.0, 1.0)
    }

    pub fn normalize(&mut self) {
        let n = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x *= n;
        self.y *= n;
        self.z *= n;
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        *self = Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Index<u8> for Vector3 {
    type Output = f32;

    fn index(&self, idx: u8) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index: {:?}, out of range for Vec3", idx),
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, rhs: &Self) -> bool {
        let eps = 1e-6;
        let ulps = 0;

        self.x.approx_eq(rhs.x, (eps, ulps))
            && self.y.approx_eq(rhs.y, (eps, ulps))
            && self.z.approx_eq(rhs.z, (eps, ulps))
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_normalize() {
        let mut vec = Vector3::new(2.0, 0.0, 0.0);
        vec.normalize();

        let vec_test = Vector3::new(1.0, 0.0, 0.0);

        assert_eq!(vec, vec_test);

        let mut vec = Vector3::new(1.0, 1.0, 1.0);
        vec.normalize();

        let vec_test = Vector3::new(1.0 / 3f32.sqrt(), 1.0 / 3f32.sqrt(), 1.0 / 3f32.sqrt());

        assert_eq!(vec, vec_test);
    }

    #[test]
    fn vec3_mul() {
        let vec = Vector3::new(3.0, 2.0, 1.0) * 2.0;
        let test_vec = Vector3::new(6.0, 4.0, 2.0);
        assert_eq!(vec, test_vec);
    }

    #[test]
    fn vec3_index() {
        let vec = Vector3::new(1.0, 2.0, 3.0);

        assert!(vec[0].approx_eq(1.0, (1e-9, 2)));
        assert!(vec[1].approx_eq(2.0, (1e-9, 2)));
        assert!(vec[2].approx_eq(3.0, (1e-9, 2)));
    }

    #[test]
    #[should_panic]
    fn vec3_index_oor() {
        let vec = Vector3::new(1.0, 2.0, 3.0);
        let _ = vec[3];
    }

    #[test]
    fn vec3_add() {
        let vec = Vector3::new(1.0, 2.0, 3.0);
        let vec = vec + vec;

        let vec_test = Vector3::new(2.0, 4.0, 6.0);
        assert_eq!(vec, vec_test);
    }

    #[test]
    fn vec3_add_assign() {
        let mut vec = Vector3::new(1.0, 2.0, 3.0);
        vec += vec;

        let vec_test = Vector3::new(2.0, 4.0, 6.0);
        assert_eq!(vec, vec_test);
    }
}

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

    /// panics if vec.len() is < 3
    /// only used first 4 elements if longer
    pub fn from_vec(vec: Vec<f32>) -> Self {
        if vec.len() < 3 {
            panic!(
                "Not enough elements in input vector: {:?} to create Vector3, min needed 3",
                vec.len()
            );
        }

        Vector3::new(vec[0], vec[1], vec[2])
    }

    pub fn normalize(&mut self) {
        let n = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x *= n;
        self.y *= n;
        self.z *= n;
    }
}

// See https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
impl IntoIterator for Vector3 {
    type Item = f32;
    type IntoIter = Vector3IntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        Vector3IntoIterator {
            vec3: self,
            index: 0,
        }
    }
}

pub struct Vector3IntoIterator {
    vec3: Vector3,
    index: usize,
}

impl Iterator for Vector3IntoIterator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let result = match self.index {
            0 => self.vec3.x,
            1 => self.vec3.y,
            2 => self.vec3.z,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Vector3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, other: Vector3) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
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
    fn eq(&self, other: &Self) -> bool {
        let mut iter = self.into_iter().zip(other.into_iter());
        iter.all(|(a, b)| a.approx_eq(b, (1e-7, 0)))
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Vector3) -> Self::Output {
        let iter = self.into_iter().zip(other.into_iter());
        let vec: Vec<f32> = iter.map(|(a, b)| a + b).collect();
        Vector3::from_vec(vec)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
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

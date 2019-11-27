use float_cmp::ApproxEq;
use std::iter::FromIterator;
use std::ops::{Add, Index, Mul};

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

    /// panics if vec.len() is < 4
    /// only uses first 4 elements if longer
    pub fn from_vec(vec: Vec<f32>) -> Self {
        if vec.len() < 4 {
            panic!(
                "Not enough elements in input vector: {:?} to create Vector4, min needed 4",
                vec.len()
            );
        }
        Vector4::new(vec[0], vec[1], vec[2], vec[3])
    }
}

// See https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
impl IntoIterator for Vector4 {
    type Item = f32;
    type IntoIter = Vector4IntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        Vector4IntoIterator {
            vec3: self,
            index: 0,
        }
    }
}

pub struct Vector4IntoIterator {
    vec3: Vector4,
    index: usize,
}

impl Iterator for Vector4IntoIterator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let result = match self.index {
            0 => self.vec3.x,
            1 => self.vec3.y,
            2 => self.vec3.z,
            3 => self.vec3.w,
            _ => return None,
        };
        self.index += 1;
        Some(result)
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
            _ => panic!("Index: {:?}, out of range for Vec3", idx),
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
        let iter = self.into_iter().zip(rhs.into_iter());
        let vec: Vec<f32> = iter.map(|(a, b)| a + b).collect();
        Vector4::from_vec(vec)
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        let mut iter = self.into_iter().zip(other.into_iter());
        iter.all(|(a, b)| a.approx_eq(b, (1e-7, 0)))
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

use super::{Degree, Matrix4, Radian, Vector3, Vector4};
use float_cmp::ApproxEq;
use std::ops::Mul;

#[derive(Clone, Copy)]
pub struct Quaternion {
    // Scalor
    s: f32,
    // Vector
    v: Vector3,

    normalized: bool,
}

impl Quaternion {
    pub fn from_axis_angle(mut axis: Vector3, angle: Degree) -> Quaternion {
        let angle = Radian::from(angle);
        let angle = angle.value * 0.5;
        let (sin, cos) = angle.sin_cos();

        axis.normalize();
        let v = axis * sin;

        Quaternion {
            s: cos,
            v,
            normalized: true,
        }
    }

    pub fn zero() -> Self {
        Quaternion {
            s: 0.0,
            v: Vector3::zero(),
            normalized: false,
        }
    }

    pub fn magnitude(&self) -> f32 {
        let mag2 =
            self.s * self.s + self.v.x * self.v.x + self.v.y * self.v.y + self.v.z * self.v.z;
        mag2.sqrt()
    }

    pub fn normalize(&mut self) {
        let n = 1.0 / self.magnitude();

        self.s *= n;
        self.v.x *= n;
        self.v.y *= n;
        self.v.z *= n;

        self.normalized = true;
    }

    pub fn axis_angle(&self) -> (Vector3, Degree) {
        let angle = Degree::from_radian(2.0 * f32::acos(self.s));
        let a = 1.0 / (1.0 - self.s * self.s);
        let mut axis = self.v * a;
        axis.normalize();
        (axis, angle)
    }

    pub fn s(&self) -> f32 {
        self.s
    }

    pub fn vec(&self) -> Vector3 {
        self.v
    }
}

impl From<Quaternion> for Matrix4 {
    fn from(mut quat: Quaternion) -> Matrix4 {
        if !quat.normalized {
            quat.normalize();
        }

        let x2 = quat.v.x * quat.v.x;
        let y2 = quat.v.y * quat.v.y;
        let z2 = quat.v.z * quat.v.z;

        let xy = quat.v.x * quat.v.y;
        let xz = quat.v.x * quat.v.z;
        let yz = quat.v.y * quat.v.z;

        let sx = quat.s * quat.v.x;
        let sy = quat.s * quat.v.y;
        let sz = quat.s * quat.v.z;

        let col_1 = Vector4::new(
            1.0 - 2.0 * y2 - 2.0 * z2,
            2.0 * xy + 2.0 * sz,
            2.0 * xz - 2.0 * sy,
            0.0,
        );
        let col_2 = Vector4::new(
            2.0 * xy - 2.0 * sz,
            1.0 - 2.0 * x2 - 2.0 * z2,
            2.0 * yz + 2.0 * sx,
            0.0,
        );
        let col_3 = Vector4::new(
            2.0 * xz + 2.0 * sy,
            2.0 * yz - 2.0 * sx,
            1.0 - 2.0 * x2 - 2.0 * y2,
            0.0,
        );
        let col_4 = Vector4::new(0.0, 0.0, 0.0, 1.0);

        Matrix4::from_cols(col_1, col_2, col_3, col_4)
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        let a = self.s * rhs.s - self.v.x * rhs.v.x - self.v.y * rhs.v.y - self.v.z * rhs.v.z;
        let b = self.v.x * rhs.s + self.s * rhs.v.x + self.v.y * rhs.v.z - self.v.z * rhs.v.y;
        let c = self.s * rhs.v.y - self.v.x * rhs.v.z + self.v.y * rhs.s + self.v.z * rhs.v.x;
        let d = self.s * rhs.v.z + self.v.x * rhs.v.y - self.v.y * rhs.v.x + self.v.z * rhs.s;

        Quaternion {
            s: a,
            v: Vector3::new(b, c, d),
            normalized: false,
        }
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        self.s.approx_eq(other.s, (1e-7, 0))
            && self.v == other.v
            && self.normalized == other.normalized
    }
}

impl std::fmt::Debug for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (axis, angle) = self.axis_angle();
        write!(
            f,
            "Quaternion:{{s: {:#?}, v: {:#?}, normalized: {:?} }}\n{{ axis: {:?}, angle: {:?} }}",
            self.s, self.v, self.normalized, axis, angle
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quat_from_axis_angle() {
        let axis = Vector3::new(2.0, 1.0, 0.0);
        let quat = Quaternion::from_axis_angle(axis, Degree::new(90.0));

        let quat_test = Quaternion {
            s: std::f32::consts::FRAC_1_SQRT_2,
            v: Vector3::new(0.632_455_5, 0.316_227_76, 0.0),
            normalized: true,
        };

        assert_eq!(quat, quat_test);
    }

    #[test]
    fn quat_to_axis_angle() {
        let mut axis = Vector3::new(2.0, 1.0, 0.0);
        axis.normalize();
        let angle = Degree::new(90.0);
        let quat = Quaternion::from_axis_angle(axis, angle);

        let (axis_test, angle_test) = quat.axis_angle();
        assert_eq!(axis, axis_test);
        assert_eq!(angle, angle_test);
    }

    #[test]
    fn quat_magnitude() {
        let quat = Quaternion {
            s: 1.0,
            v: Vector3 {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            },
            normalized: false,
        };

        let mag = quat.magnitude();

        let mag_test = 5.477_226;
        assert!(mag.approx_eq(mag_test, (1e-9, 0)));
    }

    #[test]
    fn quat_normalize() {
        let mut quat = Quaternion {
            s: 1.0,
            v: Vector3 {
                x: 2.0,
                y: 3.0,
                z: 4.0,
            },
            normalized: false,
        };
        quat.normalize();

        let quat_test = Quaternion {
            s: 0.182_574_18,
            v: Vector3::new(0.365_148_37, 0.547_722_5, 0.730_296_7),
            normalized: true,
        };

        assert_eq!(quat, quat_test);
    }

    #[test]
    fn quat_mul() {
        let quat = Quaternion {
            s: 1.0,
            v: Vector3::new(2.0, 3.0, 4.0),
            normalized: false,
        };

        let quat_mul = quat * quat;

        let quat_test = Quaternion {
            s: -28.0,
            v: Vector3::new(4.0, 6.0, 8.0),
            normalized: false,
        };

        assert_eq!(quat_mul, quat_test);
    }

    #[test]
    fn quat_to_mat4() {
        let quat = Quaternion {
            s: 1.0,
            v: Vector3::new(2.0, 3.0, 4.0),
            normalized: false,
        };

        let quat_mat = Matrix4::from(quat);

        let test_mat = Matrix4::from_cols(
            Vector4::new(-0.666_666_7, 0.666_666_7, 0.333_333_34, 0.0),
            Vector4::new(0.133_333_34, -0.333_333_34, 0.933_333_34, 0.0),
            Vector4::new(0.733_333_35, 0.666_666_7, 0.133_333_34, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        );

        assert_eq!(quat_mat, test_mat);
    }
}

use float_cmp::ApproxEq;

#[derive(Debug, Clone, Copy)]
pub struct Radian {
    pub value: f32,
}

impl Radian {
    pub fn new(value: f32) -> Radian {
        Radian { value }
    }
    pub fn from_degree(deg: f32) -> Radian {
        let value = deg.to_radians();
        Radian { value }
    }
}

impl From<Degree> for Radian {
    fn from(angle: Degree) -> Radian {
        Radian {
            value: angle.value.to_radians(),
        }
    }
}

impl PartialEq for Radian {
    fn eq(&self, other: &Self) -> bool {
        self.value.approx_eq(other.value, (1e-9, 0))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Degree {
    pub value: f32,
}

impl Degree {
    pub fn new(value: f32) -> Degree {
        Degree { value }
    }

    pub fn from_radian(rad: f32) -> Degree {
        let value = rad.to_degrees();
        Degree { value }
    }
}

impl From<Radian> for Degree {
    fn from(angle: Radian) -> Self {
        Degree {
            value: angle.value.to_degrees(),
        }
    }
}

impl PartialEq for Degree {
    fn eq(&self, other: &Self) -> bool {
        self.value.approx_eq(other.value, (1e-9, 0))
    }
}

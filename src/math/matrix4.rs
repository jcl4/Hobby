pub struct Matrix4 {
    pub data: [f32; 16],
}

impl Matrix4 {
    pub fn identity() -> Matrix4 {
        #[rustfmt::skip]
        let data = [1.0, 0.0, 0.0, 0.0,
                    0.0, 1.0, 0.0, 0.0,
                    0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0];
        Matrix4 { data }
    }

    pub fn rotate_about_z(angle: f32) -> Matrix4 {
        let angle_rad = angle.to_radians();
        let cos = angle_rad.cos();
        let sin = angle_rad.sin();

        #[rustfmt::skip]
        let data = [cos, -sin, 0.0, 0.0,
                    sin,  cos, 0.0, 0.0,
                    0.0,  0.0, 1.0, 0.0,
                    0.0,  0.0, 0.0, 1.0];
        Matrix4 { data }
    }
}

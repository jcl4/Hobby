use na;

struct Camera {
    view: na::Isometry3<f32>,
}

impl Camera {
    fn new(
         let eye = na::Point3::new(0.0, 0.0, -5.0);
        let target = na::Point3::new(0.0, 0.0, 0.0);
        let up = na::Vector3::new(0.0, -1.0, 0.0);
        let view = na::Isometry3::new_observer_frame(&eye, &target, &up);
    )
}
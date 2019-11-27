use float_cmp::ApproxEq;
use hobby::math::{Degree, Quaternion, Radian, Transform, Vector3, Vector4};

// really used for testing varios math funcitons
fn main() {
    transform_test();
}

fn normalize_vector() {
    let mut vec = Vector3::new(1.0, 1.0, 1.0);
    vec.normalize();
    println!("Normalized Vector: {:?}", vec);
}

fn transform_test() {
    let mut transform = Transform::default();
    let orientation = Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Degree::new(90.0));
    transform.orientation = orientation;

    let rotation = Quaternion::from_axis_angle(Vector3::new(1.0, 0.0, 0.0), Degree::new(90.0));

    transform.rotate(rotation);

    let quat = transform.orientation;
    let quat_test = Quaternion::from_axis_angle(Vector3::new(1.0, 1.0, 1.0), Degree::new(120.0));

    let eps = 1e-7;
    let ulps = 0;
    println!(
        "Quat s test: {:?}",
        quat.s().approx_eq(quat_test.s(), (eps, ulps))
    );

    println!(
        "Quat Vec Test: {:?}, left: {:?}, right: {:?}",
        quat.vec() == quat_test.vec(),
        quat.vec(),
        quat_test.vec(),
    );

    println!(
        "Quat v.x test: {:?}, left: {:?}, right: {:?} ",
        quat.vec().x.approx_eq(0.5, (eps, ulps)),
        quat.vec().x,
        0.5
    );
}

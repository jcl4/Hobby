use failure;
use hobby::{
    core::{MaterialType, Mesh, Model, Transform, Vertex},
    AppInfo, Game, HobbySettings, Version,
};
use nalgebra as na;
use nalgebra_glm as glm;
use simplelog as sl;
use std::iter::Iterator;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let mut config = sl::Config::default();
    config.time_format = Some("%Y-%m-%d %H:%M:%S%.3f");
    sl::TermLogger::init(sl::LevelFilter::Debug, config)?;

    let app_name = env!("CARGO_PKG_NAME");

    let major = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();

    let version = Version::new(major, minor, patch);

    let app_info = AppInfo {
        app_name: app_name.to_string(),
        app_version: version,
    };

    let mut hobby_settings = HobbySettings::default();
    hobby_settings.app_info = app_info;

    let mut game = Game::new(&hobby_settings)?;

    let mut vertices = vec![];
    let positions = vec![
        [0.5, -0.5, 0.0],
        [0.5, 0.5, 0.0],
        [-0.5, 0.5, 0.0],
        [-0.5, -0.5, 0.0],
    ];
    let colors = vec![
        [1.0, 0.0, 1.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [1.0, 0.0, 0.0, 1.0],
    ];

    for (position, color) in positions.iter().zip(colors.iter()) {
        let vertex = Vertex::builder()
            .with_position(position.clone())
            .with_color(color.clone())
            .build();
        vertices.push(vertex);
    }

    let indices = vec![0, 1, 2, 2, 3, 0];

    let mesh = Mesh::new(vertices, indices);
    let material_type = MaterialType::Basic;
    let mut model = Model::new(mesh, material_type);
    let cube_update_fn = get_cube_update();
    model.add_update_fn(cube_update_fn);

    game.add_model(model)?;

    game.run()?;

    Ok(())
}

fn get_cube_update() -> Box<dyn FnMut(Transform, f32, bool) -> Transform> {
    Box::new(|mut transform, dt, debug_display| {
        let dt_sec = dt / 1000.0;

        let rot_vel = glm::quarter_pi::<f32>();

        let rotation =
            na::UnitQuaternion::from_axis_angle(&na::Vector3::z_axis(), rot_vel * dt_sec);

        transform.rotate(rotation);

        if debug_display {
            println!(
                "Orientation Angle: {:.10}",
                transform.get_orientation().angle()
            );
            println!(
                "Orientation Axis: {:?}",
                transform.get_orientation().axis().unwrap()
            );
        }

        transform
    })
}

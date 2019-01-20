use failure;
use hobby::{
    core::{MaterialType, Mesh, Model, Vertex},
    AppInfo, Game, HobbySettings, Version,
};
use simplelog as sl;
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

    let mut vertices = vec![];

    let basic_mat = MaterialType::Basic;

    let positions = vec![[0.0, -0.5, 0.0], [0.5, 0.5, 0.0], [-0.5, 0.5, 0.0]];
    let colors = vec![
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
    ];

    let indices = vec![0, 1, 2];

    for (position, color) in positions.iter().zip(colors.iter()) {
        let vertex = Vertex::builder()
            .with_position(position.clone())
            .with_color(color.clone())
            .build();
        vertices.push(vertex);
    }

    let tri_mesh = Mesh::new(vertices, indices);
    let tri_model = Model::new(tri_mesh, basic_mat);

    let mut game = Game::new(&hobby_settings)?;
    game.add_model(tri_model)?;

    game.run()?;

    Ok(())
}

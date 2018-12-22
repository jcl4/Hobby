// #[macro_use]
use failure;
use simplelog as sl;

use std::fs::File;
use std::result;

use hobby::core::{MaterialType, Mesh, Model, Vertex};
use hobby::{AppInfo, Game, HobbySettings};

pub type Result<T> = result::Result<T, failure::Error>;

static LOG_FILE_PATH: &str = "./logs/triangle.log";

fn main() -> Result<()> {
    setup_logging();

    let app_name = env!("CARGO_PKG_NAME");

    let major = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();

    let app_info = AppInfo {
        app_name: app_name.to_string(),
        app_version: (major, minor, patch),
    };

    let mut hobby_settings = HobbySettings::default();
    hobby_settings.app_info = app_info;

    let mut game = Game::new(hobby_settings)?;

    let mut vertices = vec![];

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

    let mesh = Mesh::new(vertices, indices);
    let material_type = MaterialType::Basic;
    let model = Model::new(mesh, material_type);
    game.add_model(model)?;

    game.run()?;

    Ok(())
}

fn setup_logging() {
    let mut config = sl::Config::default();
    config.time_format = Some("[%Z: %H:%M:%S%.3f]");

    let file = File::create(LOG_FILE_PATH).expect("Unable to create log file");

    sl::CombinedLogger::init(vec![
        sl::WriteLogger::new(sl::LevelFilter::Info, config, file),
        sl::TermLogger::new(sl::LevelFilter::Warn, config).unwrap(),
    ])
    .unwrap();
}

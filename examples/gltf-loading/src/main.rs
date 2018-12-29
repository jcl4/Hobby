// Load and display a gltf cube

use failure;
use hobby::tools::gltf::GltfLoader;
use hobby::{AppInfo, Game, HobbySettings};
use nalgebra as na;
use simplelog as sl;
use std::fs::File;
use std::path::PathBuf;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;

static LOG_FILE_PATH: &str = "./logs/gltf-loading.log";

fn main() -> Result<()> {
    setup_logging();
    let app_name = env!("CARGO_PKG_NAME");

    let major = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();

    let mut resource_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    resource_path.push("resources");

    let app_info = AppInfo {
        app_name: app_name.to_string(),
        app_version: (major, minor, patch),
    };

    let mut hobby_settings = HobbySettings::default();
    hobby_settings.app_info = app_info;
    hobby_settings.resource_path = resource_path.clone();

    let mut gltf_loader = GltfLoader::new(resource_path);
    // gltf_loader.load("Buggy/glTF-Binary/Buggy.glb")?;
    // gltf_loader.load("BoxVertexColors/glTF-Binary/BoxVertexColors.glb")?;
    let mut models = gltf_loader.load("cube.glb")?;

    let mut cube = models.pop().unwrap();

    // let translation = na::Vector3::new(0.0, 0.0, 0.0);
    // cube.transform.translate(translation);

    println!("{:#?}", cube.transform);
    println!("{:#?}", cube.mesh.vertices);

    let mut game = Game::new(hobby_settings)?;
    game.add_model(cube)?;
    game.run()?;

    Ok(())
}

fn setup_logging() {
    let mut config = sl::Config::default();
    config.time_format = Some("[%Z: %H:%M:%S%.3f]");

    let file = File::create(LOG_FILE_PATH).expect("Unable to create log file");

    sl::CombinedLogger::init(vec![
        sl::WriteLogger::new(sl::LevelFilter::Info, config, file),
        sl::TermLogger::new(sl::LevelFilter::Warn, config)
            .expect("unable to create terminal logger"),
    ])
    .expect("Can not create combined logger");
}

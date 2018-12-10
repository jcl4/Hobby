// #[macro_use]
use failure;
use simplelog as sl;

use std::fs::File;
use std::result;

use hobby::{AppInfo, Game, HobbySettings, WindowSettings};

pub type Result<T> = result::Result<T, failure::Error>;

static LOG_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/logs/triangle.log");

fn main() -> Result<()> {
    setup_logging();
    let window_settings = WindowSettings::default();

    let app_name = env!("CARGO_PKG_NAME");

    let major = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();

    let app_info = AppInfo {
        app_name: app_name.to_string(),
        app_version: (major, minor, patch),
    };

    let hobby_settings = HobbySettings {
        window_settings,
        app_info,
    };

    let mut game = Game::new(hobby_settings)?;
    game.run()?;
    // game.cleanup();

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

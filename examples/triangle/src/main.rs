// #[macro_use]
use failure;
use hobby::{AppInfo, Game, HobbySettings, Version};
use simplelog as sl;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let mut config = sl::Config::default();
    config.time_format = Some("%Y-%m-%d %H:%M:%S%.3f");
    sl::TermLogger::init(sl::LevelFilter::max(), config)?;

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

    let mut game = Game::new(hobby_settings)?;

    game.run()?;

    Ok(())
}

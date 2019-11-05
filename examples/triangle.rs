<<<<<<< HEAD
use simplelog::{
    CombinedLogger, ConfigBuilder, LevelFilter, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;

use hobby::{Application, WindowSettings};

fn main() {
    create_log_folder();
    setup_logging();

    let window_settings = WindowSettings::default();

    let app = Application::new(window_settings);
    app.start();
}

fn setup_logging() {
    let file = File::create("./logs/log.txt").expect("Log File Creation Error");

    let config = ConfigBuilder::new()
        .set_time_format_str("%Y-%m-%d %H:%M:%S%.3f")
        .build();

    let write_logger = WriteLogger::new(LevelFilter::Debug, config.clone(), file);
    let mut loggers = vec![write_logger as Box<dyn SharedLogger>];

    if let Some(logger) = TermLogger::new(LevelFilter::Warn, config, TerminalMode::Mixed) {
        loggers.push(logger as Box<dyn SharedLogger>);
    }

    CombinedLogger::init(loggers).expect("Unable to create combined logger");
}

fn create_log_folder() {
    let mut path = std::env::current_dir().unwrap();
    path.push("logs");
    if let false = path.as_path().exists() {
        println!("Creating Log File Folder: {}", path.display());
        std::fs::create_dir(path).unwrap();
    }
}
=======
use hobby::{HobbySettings, Version, WindowSettings, AppInfo};


fn main() {
	let app_name = String::from("Triangle");
	let version = Version::new(1, 0, 0);

	let app_info = AppInfo{
		app_name,
		app_version: version,
	};

	let window_settings = WindowSettings::default();

	let hobby_settings = HobbySettings {
		window_settings,
		app_info,
	};
	
}
>>>>>>> 5e450d96c31cd9d53e1b2c310689f766cb67c48b

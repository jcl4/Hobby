use std::{env, fs::File, path::Path};

use simplelog as sl;

use hobby::{
    config::{AppConfig, Config, WindowConfig},
    Hobby,
};

fn main() {
    let time_format = "%F %H:%M:%S.%3f";
    let log_config = sl::ConfigBuilder::new()
        .set_time_format_str(time_format)
        .build();

    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let example_dir = root_dir.join("examples").join("triangle");
    let log_file = example_dir.join("triangle.log");

    sl::CombinedLogger::init(vec![
        sl::TermLogger::new(
            sl::LevelFilter::Info,
            log_config.clone(),
            sl::TerminalMode::Mixed,
        ),
        sl::WriteLogger::new(
            sl::LevelFilter::Trace,
            log_config,
            File::create(log_file).unwrap(),
        ),
    ])
    .expect("Unable to create logger");

    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();

    let app_config = AppConfig::builder()
        .name("Triangle Example")
        .version([major, minor, patch])
        .build();
    let bg_color = [0.757, 0.258, 0.121, 1.0];
    let window_config = WindowConfig::builder()
        .bg_color(bg_color)
        .vsync(true)
        .build();

    let config = Config {
        window: window_config,
        application: app_config,
    };

    let hobby = Hobby::from_config(config);
    hobby.run();
}

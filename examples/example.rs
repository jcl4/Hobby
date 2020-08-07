use simplelog as sl;
use std::{env, fs::File, path::Path};

pub fn setup_logging(name: &str) {
    // name should be folder name
    let time_format = "%F %H:%M:%S.%3f";
    let log_config = sl::ConfigBuilder::new()
        .set_time_format_str(time_format)
        .build();

    let full_name = format!("{}.log", name);

    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let example_dir = root_dir.join("examples").join(name);
    let log_file = example_dir.join(full_name);

    sl::CombinedLogger::init(vec![
        sl::TermLogger::new(
            sl::LevelFilter::Warn,
            log_config.clone(),
            sl::TerminalMode::Mixed,
        ),
        sl::WriteLogger::new(
            sl::LevelFilter::Info,
            log_config,
            File::create(log_file).unwrap(),
        ),
    ])
    .expect("Unable to create logger");
}
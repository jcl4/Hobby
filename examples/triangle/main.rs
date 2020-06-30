use std::{path::Path, fs::File, env};

use simplelog as sl;

use hobby::Hobby;
use hobby::Result;

fn main() -> Result<()> {
    let time_format = "%F %H:%M:%S.%3f";
    let log_config = sl::ConfigBuilder::new().set_time_format_str(time_format).build();
    
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let example_dir = root_dir.join("examples").join("triangle");
    let config_file = example_dir.join("config.toml");
    let log_file = example_dir.join("triangle.log");

    sl::CombinedLogger::init(
        vec![
            sl::TermLogger::new(sl::LevelFilter::Info, log_config.clone(), sl::TerminalMode::Mixed),
            sl::WriteLogger::new(sl::LevelFilter::Trace, log_config, File::create(log_file).unwrap()),
        ]
    )?;

    
    
    let hobby = Hobby::from_file(&config_file)?;


    // hobby.run();
    Ok(())
}

use std::{fs::File, path::Path};

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use simplelog as sl;

pub mod config;
use config::Config;

mod input;
pub use input::InputState;

pub mod gpu;

pub fn get_window_and_event_loop(config: &Config) -> (Window, EventLoop<()>) {
    let (window, event_loop) = {
        let width = config.window.width;
        let height = config.window.height;

        let title = config.application.name.clone();

        let event_loop = EventLoop::new();
        let physical_size = PhysicalSize::new(width, height);

        let window = WindowBuilder::new()
            .with_inner_size(physical_size)
            .with_title(title)
            .build(&event_loop)
            .unwrap();
        (window, event_loop)
    };
    log::info!("Window and Event Loop Created");
    (window, event_loop)
}

pub fn setup_logging(log_file_path: &Path) {
    let time_format = "%F %H:%M:%S.%3f";
    let log_config = sl::ConfigBuilder::new()
        .set_time_format_str(time_format)
        .build();

    sl::CombinedLogger::init(vec![
        sl::TermLogger::new(
            sl::LevelFilter::Warn,
            log_config.clone(),
            sl::TerminalMode::Mixed,
        ),
        sl::WriteLogger::new(
            sl::LevelFilter::Info,
            log_config,
            File::create(log_file_path).unwrap(),
        ),
    ])
    .expect("Unable to create logger");
}

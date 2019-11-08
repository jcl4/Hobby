use simplelog::{
    CombinedLogger, ConfigBuilder, LevelFilter, SharedLogger, TermLogger, TerminalMode, WriteLogger,
};
use std::fs::File;

use hobby::{
    renderer::pipelines::{ColoredMeshModel, ColoredMeshPipeline, ColoredMeshVertex},
    Application, WindowSettings,
};

fn main() {
    create_log_folder();
    setup_logging();

    let window_settings = WindowSettings::default();

    let vertices = vec![
        ColoredMeshVertex::new([0.0, -0.5, 0.0], [1.0, 0.0, 0.0, 1.0]),
        ColoredMeshVertex::new([-0.5, -0.5, 0.0], [0.0, 1.0, 0.0, 1.0]),
        ColoredMeshVertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0, 1.0]),
    ];
    let indices: Vec<u16> = vec![0, 1, 2];
    let triangle_model = ColoredMeshModel::new(vertices, indices);
    let mut pipeline = ColoredMeshPipeline::new();
    pipeline.add_model(triangle_model);

    let app = Application::new(window_settings);
    app.start(pipeline);
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

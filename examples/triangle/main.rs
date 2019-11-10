#[path = "../base.rs"]
mod base;

use hobby::{
    renderer::pipelines::{ColoredMeshModel, ColoredMeshPipeline, ColoredMeshVertex},
    Application, WindowSettings,
};

fn main() {
    base::create_log_folder();
    base::setup_logging();

    let window_settings = WindowSettings::default();

    let vertices = vec![
        ColoredMeshVertex::new([0.0, -0.5, 0.0], [1.0, 0.0, 0.0, 1.0]),
        ColoredMeshVertex::new([-0.5, 0.5, 0.0], [0.0, 1.0, 0.0, 1.0]),
        ColoredMeshVertex::new([0.5, 0.5, 0.0], [0.0, 0.0, 1.0, 1.0]),
    ];
    let indices: Vec<u16> = vec![0, 1, 2];
    let model = ColoredMeshModel::new(vertices, indices);

    let app = Application::new(window_settings);
    app.start(model);
}

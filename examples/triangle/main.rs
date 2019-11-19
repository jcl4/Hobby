#[path = "../base.rs"]
mod base;

use hobby::{Application, ApplicationSettings, Mesh, ObjectBuilder, Scene};
use log::info;

fn main() {
    base::create_log_folder();
    base::setup_logging();

    let app_settings = ApplicationSettings::default();
    let app = Application::new(app_settings);

    let mesh = Mesh::new();
    let triangle = ObjectBuilder::new()
        .with_mesh(mesh)
        .with_transform()
        .with_material()
        .build(&app);

    info!("Triangle Created");

    let mut scene = Scene::new();
    scene.add_object(triangle);
    info!("Scene Created");

    app.run(scene);
}

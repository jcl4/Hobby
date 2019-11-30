#[path = "../base.rs"]
mod base;

use hobby::{
    core::MaterialType,
    math::Transform,
    scene::{Mesh, ObjectBuilder, Scene, VertexBuilder},
    Application, ApplicationSettings,
};
use log::info;

fn triangle_verts() -> (Vec<[f32; 3]>, Vec<u16>) {
    let positions = vec![[0.0, -0.5, 0.0], [-0.5, 0.5, 0.0], [0.5, 0.5, 0.5]];

    let indices = vec![0, 1, 2];

    (positions, indices)
}

fn main() {
    base::create_log_folder();
    base::setup_logging();

    let app_settings = ApplicationSettings::default();
    let mut app = Application::new(app_settings);

    let (positions, indices) = triangle_verts();
    let vertices = positions
        .into_iter()
        .map(|pos| VertexBuilder::new(pos).build())
        .collect();

    let mesh = Mesh::new(vertices, indices);

    let transform = Transform::default();
    let material = MaterialType::SolidColor;

    let triangle = ObjectBuilder::new()
        .with_mesh(mesh)
        .with_transform(transform)
        .with_material(material)
        .build(&mut app);

    info!("Triangle Created");

    let mut scene = Scene::new();
    scene.add_object(triangle);
    info!("Scene Created");

    app.run(scene);
}

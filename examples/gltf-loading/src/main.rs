// Load and display a gltf cube

use failure;
use hobby::tools::gltf::GltfLoader;
use hobby::{AppInfo, HobbySettings};
use std::path::PathBuf;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;

fn main() -> Result<()> {
    let app_name = env!("CARGO_PKG_NAME");

    let major = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();

    let mut resource_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    resource_path.push("resources");

    let app_info = AppInfo {
        app_name: app_name.to_string(),
        app_version: (major, minor, patch),
    };

    let mut hobby_settings = HobbySettings::default();
    hobby_settings.app_info = app_info;
    hobby_settings.resource_path = resource_path.clone();

    let mut gltf_loader = GltfLoader::new(resource_path);
    gltf_loader.load("Buggy/glTF-Binary/Buggy.glb")?;
    // gltf_loader.output_details("BoxVertexColors/glTF-Binary/BoxVertexColors.glb")?;
    // gltf_loader.output_details("cube.glb")?;

    // let mut game = Game::new(hobby_settings)?;

    Ok(())
}

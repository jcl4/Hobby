use crate::Result;
use gltf::{Gltf, Node, Scene};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct GltfLoader {
    pub resource_path: PathBuf,
}

impl GltfLoader {
    pub fn new(resource_path: PathBuf) -> GltfLoader {
        GltfLoader { resource_path }
    }

    pub fn output_details(&self, gltf_file: &str) -> Result<()> {
        let mut file = create_details_file(&self.resource_path, gltf_file)?;

        let full_file = self.resource_path.join(gltf_file);
        let gltf = Gltf::open(full_file)?;

        let scenes = gltf.scenes();

        write!(file, "Number of Scenes, {}\n", scenes.count())?;

        for scene in gltf.scenes() {
            write!(
                file,
                "Scene Index, {}, Name, {}\n",
                scene.index(),
                scene.name().expect("No Name")
            )?;
            let string = get_scene_details(&scene);
            write!(file, "{}", string)?;
        }

        Ok(())
    }
}

fn create_details_file(path: &Path, gltf_file: &str) -> Result<File> {
    let file = Path::new(gltf_file);
    let file_name = file
        .file_stem()
        .expect("Incorrectly formatted GLTF File Name");
    let mut file_name = file_name.to_os_string();
    file_name.push(".csv");

    let mut desc = String::from("Details for ");
    desc.push_str(gltf_file);

    let full_path = path.join(file_name);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(full_path)?;

    write!(file, "{}\n", desc)?;

    Ok(file)
}

fn get_scene_details(scene: &Scene) -> String {
    let nodes = scene.nodes();
    let mut output = format!(",Number of Nodes, {}\n", nodes.count());

    for node in scene.nodes() {
        let temp = process_node(node, 0, false);
        output.push_str(&temp);
    }

    output
}

fn process_node(node: Node, mut depth: u32, is_child: bool) -> String {
    let mut prefix = String::new();
    let mut output = String::new();

    depth += 1;

    for _i in 0..depth {
        prefix.push(',');
    }

    let mut temp: String;

    if is_child {
        temp = format!("{} Child Node, ", prefix,);
    } else {
        temp = format!("{} ", prefix,);
    }

    temp.push_str(&format!(
        "Node Index, {}, node name, {}\n",
        node.index(),
        node.name().expect("No Name")
    ));

    output.push_str(&temp);

    output
}

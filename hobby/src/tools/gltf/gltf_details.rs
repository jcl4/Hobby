use gltf;
use crate::Result;
use super::ImportData;
use std::path::{Path, PathBuf};
use std::fs::{OpenOptions, File};
use std::io::Write;

pub fn output_details(import_data: ImportData, full_path: &PathBuf) -> Result<()> {
        let folder = full_path.parent().unwrap();
        let file = full_path.file_name().unwrap();
     let (mut csv_file, json_file) = create_details_file(&folder, &Path::new(file))?;

      let scenes = import_data.doc.scenes();
      write!(csv_file, "Number of Scenes, {}\n", scenes.count())?;
      for scene in import_data.doc.scenes() {
          write!(
              csv_file,
              "Scene Index, {}, Name, {}\n",
              scene.index(),
              scene.name().unwrap_or("No Name")
          )?;
          let string = get_scene_details(&scene, &import_data.buffers);
          write!(csv_file, "{}", string)?;
      }
      for material in import_data.doc.materials() {
          let string = get_material_details(&material);
          write!(csv_file, "{}", string)?;
      }
      import_data.doc.into_json().to_writer_pretty(json_file)?;
      Ok(())
 }


fn create_details_file(path: &Path, gltf_file: &Path) -> Result<(File, File)> {
    // let file = Path::new(gltf_file);
    let file_name = gltf_file
        .file_stem()
        .expect("Incorrectly formatted GLTF File Name");
    let file_name = file_name.to_os_string();
    let csv_file_name = file_name.clone().into_string().unwrap() + ".csv";

    let json_file_name = file_name.into_string().unwrap() + ".json";

    let mut desc = String::from("Details for ");
    desc.push_str(&gltf_file.to_str().unwrap());

    let csv_full_path = path.join(csv_file_name);

    let mut csv_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(csv_full_path)?;

    write!(csv_file, "{}\n", desc)?;

    let json_full_path = path.join(json_file_name);
    let json_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(json_full_path)?;

    Ok((csv_file, json_file))
}

fn get_scene_details(scene: &gltf::Scene, buffers: &Vec<gltf::buffer::Data>) -> String {
    let nodes = scene.nodes();
    let mut output = format!(",Number of Nodes, {}\n", nodes.count());

    for node in scene.nodes() {
        let temp = process_node(node, 0, false, buffers);
        output.push_str(&temp);
    }



    output
}

fn process_node(
    node: gltf::Node,
    mut depth: u32,
    is_child: bool,
    buffers: &Vec<gltf::buffer::Data>,
) -> String {
    let mut prefix = String::new();
    let mut output;

    depth += 1;

    prefix = get_prefix(depth);

    if is_child {
        output = format!("{} Child Node, ", prefix,);
    } else {
        output = format!("{} ", prefix,);
    }

    output.push_str(&format!(
        "Node Index, {}, node name, {},",
        node.index(),
        node.name().unwrap_or("No Name")
    ));

    let (temp, camera_data) = match node.camera() {
        Some(camera) => (String::from(" Camera,"), get_camera_data(camera, depth)),
        None => (String::from(" No Camera,"), String::new()),
    };
    output.push_str(&temp);

    let (temp, mesh_data) = match node.mesh() {
        Some(mesh) => (String::from(" Mesh,"), get_mesh_data(mesh, depth, buffers)),
        None => (String::from(" No Mesh,"), String::new()),
    };

    output.push_str(&temp);
    output.push_str("\n");
    output.push_str(&camera_data);
    output.push_str(&mesh_data);
    for child in node.children() {
        output.push_str(&process_node(child, depth, true, buffers));
    }

    output
}

fn get_camera_data(camera: gltf::Camera, mut depth: u32) -> String {
    depth += 1;
    let prefix = get_prefix(depth);
    let mut output = format! {"{} Camera Data,", prefix};

    let temp = match camera.projection() {
        gltf::camera::Projection::Orthographic(projection) => {
            let temp = format! {" Othrographic Camera, X Mag, {}, Y Mag {}, Z Far {}, Z Near {},",
                projection.xmag(),
                projection.ymag(),
                projection.zfar(),
                projection.znear()
            };
            temp
        }
        gltf::camera::Projection::Perspective(projection) => {
            let temp = format! {" Perspective Camera, Aspect Ratio, {}, Y Fov, {}, Z Far, {}, Z Near, {},",
                projection.aspect_ratio().unwrap_or(0.0),
                projection.yfov(),
                projection.zfar().unwrap_or(0.0),
                projection.znear(),
            };
            temp
        }
    };
    output.push_str(&temp);
    output.push_str("\n");

    output
}

fn get_mesh_data(mesh: gltf::Mesh, mut depth: u32, buffers: &Vec<gltf::buffer::Data>) -> String {
    depth += 1;
    let prefix = get_prefix(depth);
    let mut output = format!(
        "{} Mesh Data, Name, {}, Number of Primitives, {},\n",
        prefix,
        mesh.name().unwrap_or("No Name"),
        mesh.primitives().count()
    );

    depth += 1;
    for primitive in mesh.primitives() {
        let temp = get_primitive_data(primitive, depth, buffers);
        output.push_str(&temp);
    }

    output
}

fn get_primitive_data(
    primitive: gltf::Primitive,
    depth: u32,
    buffers: &Vec<gltf::buffer::Data>,
) -> String {
    let mut output = String::new();
    let prefix = get_prefix(depth);
    output = format!("{} Primitive Data,", prefix);

    let temp = format!(
        " Material index, {}, Material Name, {},",
        primitive.material().index().unwrap_or(999),
        primitive.material().name().unwrap_or("No Name")
    );
    output.push_str(&temp);

    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

    let temp = match reader.read_positions() {
        Some(_) => " Positions,",
        None => " No Positions,",
    };
    output.push_str(&temp);

    let temp = match reader.read_normals() {
        Some(_) => " Normals,",
        None => " No Normals,",
    };
    output.push_str(&temp);

    let temp = match reader.read_tangents() {
        Some(_) => " Tangents,",
        None => " No Tangents,",
    };
    output.push_str(&temp);

    let temp = match reader.read_colors(0) {
        Some(_) => " Colors,",
        None => " No Colors,",
    };
    output.push_str(&temp);

    if reader.read_colors(1).is_some() {
        println!(
            "More then one color set is not supported, primitive index: {}",
            primitive.index()
        );
    }

    let temp = match reader.read_indices() {
        Some(_) => " Indices,",
        None => " No Indices,",
    };
    output.push_str(&temp);

    let temp = match reader.read_joints(0) {
        Some(_) => " Joints,",
        None => " No Joints,",
    };
    output.push_str(&temp);
    if reader.read_joints(1).is_some() {
        println!(
            "More then one joint set is not supported, primitive index: {}",
            primitive.index()
        );
    }

    let temp = match reader.read_tex_coords(0) {
        Some(_) => " Tex Coords,",
        None => " No Tex Coords,",
    };
    output.push_str(&temp);
    if reader.read_tex_coords(1).is_some() {
        println!(
            "More then one Texture Coordinate set in not supported, primitive index: {}",
            primitive.index()
        );
    }

    let temp = match reader.read_weights(0) {
        Some(_) => " Weights,",
        None => "No Weights",
    };
    output.push_str(&temp);
    if reader.read_weights(1).is_some() {
        println!(
            "More then one weight set is not supported, primitive index: {}",
            primitive.index()
        )
    }

    output.push_str("\n");
    output
}

fn get_material_details(material: &gltf::Material) -> String {
    let mut output = String::new();

    let temp = format!(
        " Material index, {}, Name, {},",
        material.index().unwrap_or(999),
        material.name().unwrap_or("No Name")
    );
    output.push_str(&temp);
    output.push_str(&format!(
        "\n, Alpha Cutoff, {}, Alpha Mode,",
        material.alpha_cutoff()
    ));

    let temp = match material.alpha_mode() {
        gltf::material::AlphaMode::Opaque => " Opaque,",
        gltf::material::AlphaMode::Mask => " Mask,",
        gltf::material::AlphaMode::Blend => " Blend,",
    };
    output.push_str(&temp);

    output.push_str(&format!(
        " Double Sided, {}\n, PBR Data,",
        material.double_sided()
    ));

    let pbr = material.pbr_metallic_roughness();
    output.push_str(&format!(
        " Base Color Factor, {}, {}, {}, {}, Metalic Factor, {}, Roughness Factor, {}\n, Texture Data", 
        pbr.base_color_factor()[0], 
        pbr.base_color_factor()[1], 
        pbr.base_color_factor()[2], 
        pbr.base_color_factor()[3], 
        pbr.metallic_factor(), 
        pbr.roughness_factor()
    ));

    let temp = match pbr.base_color_texture() {
        Some(_) => " Has Color Texture,",
        None => " No Color Texture,",
    };
    output.push_str(&temp);

    let temp = match pbr.metallic_roughness_texture() {
        Some(_) => " Has Metallic Roughness Texture,",
        None => " No Metallic Roughness Texture,",
    };
    output.push_str(&temp);


    let temp = match material.normal_texture() {
        Some(_) => " Has Normal Texture,",
        None => " No Normal Texture,",
    };
    output.push_str(&temp);

    let temp = match material.occlusion_texture() {
        Some(_) => " Has Occlusion Texture,",
        None => " No Occlusion Texture,",
    };
    output.push_str(&temp);

    let temp = match material.emissive_texture() {
        Some(_) => " Has Emissive Texture,",
        None => " No Emissive Texture,",
    };
    output.push_str(&temp);

    let ef = material.emissive_factor();
    output.push_str(&format!(
        " Emmisive Factor, {}, {}, {}\n",
        ef[0], ef[1], ef[2]
    ));

    output
}

fn get_prefix(depth: u32) -> String {
    let mut prefix = String::new();
    for _i in 0..depth {
        prefix.push(',');
    }

    prefix
}

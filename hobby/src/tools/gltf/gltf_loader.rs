use super::gltf_details::output_details;
use crate::core::{MaterialType, Mesh, Model, Transform, Vertex};
use crate::{na, Result};
use failure::bail;
use gltf;
use log::warn;
use std::path::PathBuf;

pub struct ImportData {
    pub doc: gltf::Document,
    pub buffers: Vec<gltf::buffer::Data>,
    pub images: Vec<gltf::image::Data>,
}

pub struct GltfLoader {
    pub resource_path: PathBuf,
}

impl GltfLoader {
    pub fn new(resource_path: PathBuf) -> GltfLoader {
        GltfLoader { resource_path }
    }

    pub fn load(&mut self, gltf_file: &str) -> Result<Vec<Model>> {
        let full_path = self.resource_path.join(gltf_file);
        let (doc, buffers, images) = gltf::import(full_path.clone())?;
        let import_data = ImportData {
            doc,
            buffers,
            images,
        };

        output_details(&import_data, &full_path)?;
        let models = create_models(&import_data)?;

        Ok(models)
    }
}

fn create_models(import_data: &ImportData) -> Result<Vec<Model>> {
    let nodes = import_data.doc.nodes();

    let mut models: Vec<Model> = vec![];

    for node in nodes {
        if let Some(mesh) = node.mesh() {
            let transform = create_transform(&node);
            let meshes = create_meshes(&mesh, &import_data.buffers)?;
            for mesh in meshes.into_iter() {
                let mut model = Model::new(mesh, MaterialType::Basic);
                model.transform = transform.clone();
                models.push(model);
            }
        }
    }

    Ok(models)
}

fn create_transform(node: &gltf::Node) -> Transform {
    let (translation, orientation, scale) = node.transform().decomposed();

    let pos = na::Vector3::from(translation);
    let orientation = na::Quaternion::from(na::Vector4::from(orientation));
    let scale = na::Vector3::from(scale);

    Transform::new(pos, scale, na::UnitQuaternion::from_quaternion(orientation))
}

fn create_meshes(g_mesh: &gltf::Mesh, buffers: &Vec<gltf::buffer::Data>) -> Result<Vec<Mesh>> {
    let primitives = g_mesh.primitives();

    let mut meshes: Vec<Mesh> = vec![];

    for primitive in primitives {
        let mesh = process_primitive(&primitive, buffers, g_mesh.index())?;
        meshes.push(mesh);
    }

    Ok(meshes)
}

fn process_primitive(
    primitive: &gltf::Primitive,
    buffers: &Vec<gltf::buffer::Data>,
    mesh_index: usize,
) -> Result<Mesh> {
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

    let mut positions: Vec<[f32; 3]> = vec![];
    match reader.read_positions() {
        Some(read_positions) => {
            for position in read_positions {
                positions.push(position);
            }
        }
        None => {
            warn!(
                "primitives must have the POSITION attribute (mesh: {}, primitive: {})",
                mesh_index,
                primitive.index()
            );
            bail!("Hobby renderer requires Position Attribures");
        }
    }

    let mut vertices: Vec<Vertex> = positions
        .into_iter()
        .map(|position| Vertex {
            position,
            ..Vertex::default()
        })
        .collect();

    if let Some(normals) = reader.read_normals() {
        for (i, normal) in normals.enumerate() {
            vertices[i].normal = Some(normal);
        }
    };

    if let Some(tangents) = reader.read_tangents() {
        for (i, tangent) in tangents.enumerate() {
            vertices[i].tangent = Some(tangent);
        }
    };

    if let Some(tex_coords) = reader.read_tex_coords(0) {
        for (i, tex_coord) in tex_coords.into_f32().enumerate() {
            vertices[i].tex_coord = Some(tex_coord);
        }
    };

    if reader.read_tex_coords(1).is_some() {
        warn!(
            "More then one Texture Coordinate set is not supported (mesh: {}, primitive: {})",
            mesh_index,
            primitive.index()
        );
    }

    if let Some(colors) = reader.read_colors(0) {
        let colors = colors.into_rgba_f32();
        for (i, color) in colors.enumerate() {
            vertices[i].color = Some(color);
        }
    }

    if reader.read_colors(1).is_some() {
        warn!(
            "More then one Color set is not supported (mesh: {}, primitive: {})",
            mesh_index,
            primitive.index()
        );
    }

    let mut indices: Vec<u32> = vec![];

    match reader.read_indices() {
        Some(read_indices) => {
            for ind in read_indices.into_u32() {
                indices.push(ind);
            }
        }
        None => {
            warn!(
                "Hobby renderer required indices (mesh {}, primitieve: {}",
                mesh_index,
                primitive.index()
            );
            bail!("Hobby Renderer requires indices")
        }
    }

    Ok(Mesh::new(vertices, indices))
}

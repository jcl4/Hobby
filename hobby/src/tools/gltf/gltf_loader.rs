use super::gltf_details::output_details;
use crate::Result;
use gltf;
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

    pub fn load(&mut self, gltf_file: &str) -> Result<()> {
        let full_path = self.resource_path.join(gltf_file);
        let (doc, buffers, images) = gltf::import(full_path.clone())?;
        let import_data = ImportData {
            doc,
            buffers,
            images,
        };

        output_details(import_data, &full_path)?;

        Ok(())
    }
}

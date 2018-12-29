use crate::renderer::Renderer;
use crate::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use vulkano::format::Format;
use vulkano::image::{Dimensions, ImmutableImage};
use vulkano::sampler::Sampler;

pub struct Texture {
    pub name: String,
    path: PathBuf,
    texture: Option<Arc<ImmutableImage<Format>>>,
    sampler: Option<Arc<Sampler>>,
}

impl Texture {
    pub fn new(path: &Path) -> Result<Texture> {
        let file_name = path.file_stem().expect("No file name in texture path");
        let name = file_name
            .to_str()
            .expect("Invalide file name format for texture");

        Ok(Texture {
            name: name.into(),
            path: path.into(),
            texture: None,
            sampler: None,
        })
    }

    pub fn build(&mut self, renderer: &Renderer) -> Result<()> {
        let img = image::open(self.path)?.to_rgba();
        let (width, height) = img.dimensions();

        let image_data = img.into_raw().clone();
        let (texture, tex_future) = ImmutableImage::from_iter(
            image_data.into_iter(),
            Dimensions::Dim2d {
                width: 93,
                height: 93,
            },
            Format::R8G8B8A8Srgb,
            renderer.graphics_queue.clone(),
        )?;
        self.texture = Some(texture);

        Ok(())
    }

    pub fn get_texture_and_sampler(&self) -> (Arc<ImmutableImage<Format>>, Arc<Sampler>) {
        (self.texture.unwrap().clone(), self.sampler.unwrap().clone())
    }
}

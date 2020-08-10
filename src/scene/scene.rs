use super::{Material, Model};
use crate::{renderer::pipelines::pipeline, Renderer};
use std::collections::HashMap;

/// The scene keeps a collection of pipelines that are in use,
/// It is a hash map that uses the material enum as the key
pub struct Scene {
    models: Vec<Model>,
    render_pipelines: HashMap<Material, wgpu::RenderPipeline>,
}

impl Scene {
    // TODO: Check for in-use pipeline prior to setting
    pub(crate) fn draw<'a>(&'a self, mut render_pass: &mut wgpu::RenderPass<'a>) {
        for model in self.models.iter() {
            render_pass.set_pipeline(&self.render_pipelines[&model.material]);
            model.draw(&mut render_pass)
        }
    }
}

pub struct SceneBuilder {
    models: Vec<Model>,
}

impl SceneBuilder {
    pub fn new(models: Vec<Model>) -> SceneBuilder {
        SceneBuilder { models }
    }

    pub fn build(self, renderer: &Renderer) -> Scene {
        let mut render_pipelines: HashMap<Material, wgpu::RenderPipeline> = HashMap::new();
        for model in self.models.iter() {
            #[allow(clippy::map_entry)]
            if !render_pipelines.contains_key(&model.material) {
                let rp = pipeline::create_render_pipeline(
                    &model.material,
                    &renderer.device,
                    &renderer.sc_desc,
                );
                render_pipelines.insert(model.material, rp);
                log::info!("Pipeline Built")
            }
        }
        Scene {
            models: self.models,
            render_pipelines,
        }
    }
}

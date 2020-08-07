use super::{Model, Material};
use std::collections::HashMap;

pub struct Scene {
    models: Vec<Model>,
    render_pipelines: HashMap<Material, wgpu::RenderPipeline>,
}

impl Scene {
    pub fn new(models: Vec<Model>) -> Scene {
        Scene {

        }
    }
}
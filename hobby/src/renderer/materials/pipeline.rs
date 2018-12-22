use crate::core::Model;
use crate::renderer::Renderer;
use crate::Result;
use std::sync::Arc;
use vulkano::pipeline::GraphicsPipelineAbstract;

pub trait ModelPipeline {
    fn build_model(&mut self, model: &mut Model, renderer: &Renderer) -> Result<()>;

    fn graphics_pipeline(&self) -> Arc<GraphicsPipelineAbstract + Send + Sync>;
}

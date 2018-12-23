use crate::core::{Model, Transform};
use crate::renderer::Renderer;
use crate::Result;
use std::sync::Arc;
use vulkano::descriptor::DescriptorSet;
use vulkano::pipeline::GraphicsPipelineAbstract;

pub trait ModelPipeline {
    fn build_model(&mut self, model: &mut Model, renderer: &Renderer) -> Result<()>;

    fn graphics_pipeline(&self) -> Arc<GraphicsPipelineAbstract + Send + Sync>;

    fn get_descriptor_set(
        &mut self,
        transform: &Transform,
    ) -> Result<Arc<DescriptorSet + Send + Sync>>;
}

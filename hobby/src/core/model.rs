use crate::core::{MaterialType, Mesh};
use crate::glm;
use crate::renderer::materials::basic;
use crate::renderer::Renderer;
use crate::Result;
use std::sync::Arc;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::pipeline::GraphicsPipelineAbstract;

// TODO: Create Pipeline and uniform buffer information

pub struct Model {
    pub mesh: Mesh,
    pub material_type: MaterialType,
    transform: glm::TMat4<f32>,
    pipeline: Option<Arc<GraphicsPipelineAbstract + Send + Sync>>,
}

impl Model {
    pub fn new(mesh: Mesh, material_type: MaterialType) -> Model {
        let scale_vec = glm::vec3(1.0, 1.0, 1.0);
        let scale = glm::scaling(&scale_vec);

        Model {
            mesh,
            material_type,
            transform: scale,
            pipeline: None,
        }
    }

    pub fn set_pipeline(&mut self, pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>) {
        self.pipeline = Some(pipeline);
    }

    pub fn draw(
        &mut self,
        command_buffer: AutoCommandBufferBuilder,
    ) -> Result<AutoCommandBufferBuilder> {
        let new_cb = command_buffer
            .draw_indexed(
                self.pipeline.clone().unwrap(),
                &DynamicState::none(),
                vec![self.mesh.vertex_buffer()],
                self.mesh.index_buffer(),
                (),
                (),
            )
            .unwrap();

        Ok(new_cb)
    }

    pub fn build(&mut self, renderer: &Renderer) -> Result<()> {
        match self.material_type {
            MaterialType::Basic => basic::build_basic_model(self, renderer)?,
        };

        Ok(())
    }
}

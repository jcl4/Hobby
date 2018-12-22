use crate::core::{MaterialType, Mesh};
use crate::glm;
use crate::renderer::materials::{BasicPipeline, ModelPipeline};
use crate::renderer::Renderer;
use crate::Result;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};

// TODO: Create Pipeline and uniform buffer information

pub struct Model {
    pub mesh: Mesh,
    pub material_type: MaterialType,
    pipeline: Option<Box<ModelPipeline>>,
    transform: glm::TMat4<f32>,
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

    pub fn draw(
        &mut self,
        command_buffer: AutoCommandBufferBuilder,
    ) -> Result<AutoCommandBufferBuilder> {
        let new_cb = command_buffer
            .draw_indexed(
                self.pipeline.as_ref().unwrap().graphics_pipeline(),
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
        let mut pipeline = match self.material_type {
            MaterialType::Basic => BasicPipeline::new(renderer)?,
        };

        pipeline.build_model(self, renderer)?;

        self.pipeline = Some(Box::new(pipeline));

        Ok(())
    }
}

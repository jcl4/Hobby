use crate::core::{MaterialType, Mesh, Transform};
use crate::renderer::materials::{BasicPipeline, ModelPipeline};
use crate::renderer::Renderer;
use crate::Result;
use std::time::Duration;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};

// TODO: Create Pipeline and uniform buffer information

pub struct Model {
    pub mesh: Mesh,
    pub material_type: MaterialType,
    pipeline: Option<Box<dyn ModelPipeline>>,
    transform: Box<Transform>,
    model_update: Box<dyn Fn(Box<Transform>, Duration) -> Box<Transform>>,
}

impl Model {
    pub fn new(mesh: Mesh, material_type: MaterialType) -> Model {
        let transform = Box::new(Transform::default());
        let dt = Duration::from_secs(1);
        let model_update = Box::new(|transform, dt| transform);

        Model {
            mesh,
            material_type,
            transform,
            pipeline: None,
            model_update,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.transform = (self.model_update)(self.transform.clone(), dt);
    }

    pub fn draw(
        &mut self,
        command_buffer: AutoCommandBufferBuilder,
    ) -> Result<AutoCommandBufferBuilder> {
        let set = self
            .pipeline
            .as_mut()
            .unwrap()
            .get_descriptor_set(&mut self.transform)?;

        let new_cb = command_buffer
            .draw_indexed(
                self.pipeline.as_ref().unwrap().graphics_pipeline(),
                &DynamicState::none(),
                vec![self.mesh.vertex_buffer()],
                self.mesh.index_buffer(),
                set,
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

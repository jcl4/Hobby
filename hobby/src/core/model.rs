use crate::Result;
use crate::{
    core::{MaterialType, Mesh, Transform},
    graphics::{
        pipelines::{BasicPipeline, Pipeline},
        Renderer,
    },
};
use ash::{version::DeviceV1_0, vk};

pub struct Model {
    pub mesh: Mesh,
    pub material_type: MaterialType,
    pipeline: Option<Box<dyn Pipeline>>,
    pub transform: Transform,
    // Update function closure called in model update inputs: Transform: Model transform, f32: Update Time in ms, bool: debug update
    model_update: Box<dyn FnMut(Transform, f32, bool) -> Transform>,
}

impl Model {
    pub fn new(mesh: Mesh, material_type: MaterialType) -> Model {
        let transform = Transform::default();
        let model_update = Box::new(|transform, _dt, _debug| transform);

        Model {
            mesh,
            material_type,
            pipeline: None,
            transform,
            model_update,
        }
    }

    pub(crate) fn build(&mut self, renderer: &Renderer) -> Result<()> {
        let mut pipeline = match self.material_type {
            MaterialType::Basic => BasicPipeline::default(),
        };

        pipeline.create_pipeline(
            &renderer.device,
            renderer.swapchain_data.extent,
            renderer.render_pass,
        )?;

        self.mesh.build_mesh(renderer, &self.material_type)?;

        self.pipeline = Some(Box::new(pipeline));

        Ok(())
    }

    pub(crate) fn draw(&self, cb: vk::CommandBuffer, device: &ash::Device) -> Result<()> {
        unsafe {
            device.cmd_bind_pipeline(
                cb,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.as_ref().unwrap().get_pipeline(),
            );
        }

        self.mesh.draw(cb);

        Ok(())
    }

    pub(crate) fn cleanup(&self, device: &ash::Device) -> Result<()> {
        self.pipeline.as_ref().unwrap().cleanup(device)?;
        self.mesh.cleanup();
        Ok(())
    }
}

//     pub fn add_update_fn(&mut self, f: Box<dyn FnMut(Transform, f32, bool) -> Transform>) {
//         self.model_update = f;
//     }

//     pub fn update(&mut self, dt: f32, debug_display: bool) {
//         self.transform = (self.model_update)(self.transform.clone(), dt, debug_display);
//     }

//     pub fn draw(
//         &mut self,
//         command_buffer: AutoCommandBufferBuilder,
//         view: [[f32; 4]; 4],
//         proj: [[f32; 4]; 4],
//     ) -> Result<AutoCommandBufferBuilder> {
//         let set =
//             self.pipeline
//                 .as_mut()
//                 .unwrap()
//                 .get_descriptor_set(&mut self.transform, view, proj)?;

//         let new_cb = command_buffer
//             .draw_indexed(
//                 self.pipeline.as_ref().unwrap().graphics_pipeline(),
//                 &DynamicState::none(),
//                 vec![self.mesh.vertex_buffer()],
//                 self.mesh.index_buffer(),
//                 set,
//                 (),
//             )
//             .unwrap();

//         Ok(new_cb)
//     }

// }

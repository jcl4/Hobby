use crate::{
    core::{MaterialType, Vertex},
    graphics::{pipelines::BasicVertex, Renderer, VkMesh},
    Result,
};
use ash::vk;
use log::debug;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    vk_mesh: Option<VkMesh>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Mesh {
        Mesh {
            vertices,
            indices,
            vk_mesh: None,
        }
    }

    pub(crate) fn build_mesh(
        &mut self,
        renderer: &Renderer,
        material_type: &MaterialType,
    ) -> Result<()> {
        let vk_mesh = match material_type {
            MaterialType::Basic => {
                VkMesh::new::<BasicVertex>(renderer, &self.vertices, &self.indices)?
            }
        };

        self.vk_mesh = Some(vk_mesh);
        Ok(())
    }

    pub(crate) fn draw(&self, cb: vk::CommandBuffer) {
        self.vk_mesh.as_ref().unwrap().draw(cb);
    }

    pub(crate) fn cleanup(&self) {
        debug!("Mesh Cleaned up");
        self.vk_mesh.as_ref().unwrap().cleanup();
    }
}

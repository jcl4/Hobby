/*
Renderer workflow thoughts
- Scene holds a vector of models and a collection of unique materials
- a Material enum is exposed to the user to select various materials
- actual materials are rendering pipelines
- Renderer holds GPU objects
- Object hold all its required buffers and a draw call
-
- Object builder builds the buffers and bind group
- to build the bind group the bind group layout will need to be created
- bind group layouts need the pipeline to be created
- build a piplenines struct with options for each pipeline
- get piple line checks for is some if none creates pipeline
*/

use log::info;
use winit::window::Window;

use crate::{
    core::MaterialType,
    scene::{Mesh, ObjectBufferGroup, Scene},
};

/// The renderer is responsible for displaying a scene on screan
///
/// It holds all required GPU resources
#[allow(dead_code)]
pub(crate) struct Renderer {}

impl Renderer {
    pub(crate) fn new(window: &Window) -> Renderer {
        Renderer {}
    }

    pub(crate) fn get_object_buffer_group(
        &self,
        mesh: &Mesh,
        material: MaterialType,
    ) -> ObjectBufferGroup {
        unimplemented!()
    }

    pub(crate) fn render(&self, scene: &Scene) {
        unimplemented!()
    }
}

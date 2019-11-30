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

use gfx_hal as hal;
use hal::Instance;

use gfx_backend_vulkan as back;

use std::mem::ManuallyDrop;

/// The renderer is responsible for displaying a scene on screan
///
/// It holds all required GPU resources
#[allow(dead_code)]
pub(crate) struct Renderer {
    _instance: ManuallyDrop<back::Instance>,
}

impl Renderer {
    pub(crate) fn new(window: &Window, app_title: &str, app_version: u32) -> Renderer {
        let instance =
            back::Instance::create(app_title, app_version).expect("Instance Creation expect");
        info!("Instance Created");

        Renderer {
            _instance: ManuallyDrop::new(instance),
        }
    }

    pub(crate) fn get_object_buffer_group(
        &self,
        _mesh: &Mesh,
        _material: MaterialType,
    ) -> ObjectBufferGroup {
        unimplemented!()
    }

    pub(crate) fn render(&self, scene: &Scene) {
        unimplemented!()
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {}
}

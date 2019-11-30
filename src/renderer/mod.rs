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
use hal::{adapter, queue, queue::family::QueueFamily, window::Surface, Instance};

use gfx_backend_vulkan as back;

use std::{mem::ManuallyDrop, ptr};

/// The renderer is responsible for displaying a scene on screan
///
/// It holds all required GPU resources
#[allow(dead_code)]
pub(crate) struct Renderer {
    instance: back::Instance,
    surface: ManuallyDrop<<back::Backend as hal::Backend>::Surface>,
}

impl Renderer {
    pub(crate) fn new(window: &Window, app_title: &str, app_version: u32) -> Renderer {
        let instance =
            back::Instance::create(app_title, app_version).expect("Instance Creation expect");
        info!("Instance Created");

        let surface = unsafe {
            instance
                .create_surface(window)
                .expect("Surface Creation expect")
        };
        info!("Surface Created");

        let adapter = {
            let adapter = instance
                .enumerate_adapters()
                .into_iter()
                .find(|a| a.info.device_type == adapter::DeviceType::DiscreteGpu)
                .expect("Unable to find descrete gpu in system");

            let supported = adapter.queue_families.iter().any(|qf| {
                qf.queue_type() == queue::QueueType::General && surface.supports_queue_family(qf)
            });

            if !supported {
                panic!("Unable to find adapter with an appropriate queue family");
            }
            adapter
        };
        info!("Adapder selected: {:#?}", adapter);

        Renderer {
            instance: instance,
            surface: ManuallyDrop::new(surface),
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
    fn drop(&mut self) {
        unsafe {
            let surface = ManuallyDrop::into_inner(ptr::read(&self.surface));
            self.instance.destroy_surface(surface);
        }
    }
}

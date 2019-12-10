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
use ash::{version::DeviceV1_0, vk};
use winit::window::Window;

use crate::{
    core::MaterialType,
    scene::{Mesh, ObjectBufferGroup, Scene},
};

mod context;
mod pipelines;
mod swapchain;

use context::Context;
use pipelines::SolidColor;
use swapchain::{SwapchainData, SwapchainProperties};

/// The renderer is responsible for displaying a scene on screan
///
/// It holds all required GPU resources
#[allow(dead_code)]
pub struct Renderer {
    context: Context,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    swapchain_data: SwapchainData,
    solid_color_pipeline: SolidColor,
    render_pass: vk::RenderPass,
}

impl Renderer {
    pub fn new(window: &Window, app_name: &str, app_version: u32) -> Renderer {
        let context = Context::new(window, app_name, app_version);
        log::info!("Context Created");

        let queue_families_indices = context.queue_families_indices();

        let window_size = window.inner_size();
        let (width, height): (u32, u32) = window_size.to_physical(window.hidpi_factor()).into();
        let swapchain_data =
            swapchain::create_swapchain_data(&context, queue_families_indices, width, height);
        log::info!("Swapchain Created");

        let graphics_queue = unsafe {
            context
                .device()
                .get_device_queue(queue_families_indices.graphics_index(), 0)
        };
        let present_queue = unsafe {
            context
                .device()
                .get_device_queue(queue_families_indices.present_index(), 0)
        };

        let render_pass = create_render_pass(context.device(), &swapchain_data.properties());
        log::info!("Render Pass Created");

        let solid_color_pipeline =
            SolidColor::new(context.device(), &swapchain_data.properties(), render_pass);

        Renderer {
            context,
            graphics_queue,
            present_queue,
            swapchain_data,
            solid_color_pipeline,
            render_pass,
        }
    }

    pub fn get_object_buffer_group(
        &self,
        _mesh: &Mesh,
        _material: MaterialType,
    ) -> ObjectBufferGroup {
        unimplemented!()
    }

    pub fn draw_frame(&self, _scene: &Scene) {
        unimplemented!()
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.solid_color_pipeline.cleanup(self.context.device());
        swapchain::cleanup_swapchain(self.context.device(), &self.swapchain_data);
    }
}

fn create_render_pass(
    device: &ash::Device,
    swapchain_properties: &SwapchainProperties,
) -> vk::RenderPass {
    let color_attachment_desc = vk::AttachmentDescription::builder()
        .format(swapchain_properties.format.format)
        .samples(vk::SampleCountFlags::TYPE_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .build();
    let attachment_descs = [color_attachment_desc];

    let color_attachment_ref = vk::AttachmentReference::builder()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .build();
    let color_attachment_refs = [color_attachment_ref];

    let subpass_desc = vk::SubpassDescription::builder()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(&color_attachment_refs)
        .build();
    let subpass_descs = [subpass_desc];

    let render_pass_info = vk::RenderPassCreateInfo::builder()
        .attachments(&attachment_descs)
        .subpasses(&subpass_descs)
        .build();

    unsafe { device.create_render_pass(&render_pass_info, None).unwrap() }
}

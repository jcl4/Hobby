mod context;
mod debug;
mod swapchain;

pub(crate) mod pipeline;

use context::Context;
use swapchain::SwapchainDetails;

use ash::{
    extensions::ext::DebugUtils,
    version::{DeviceV1_0, InstanceV1_0},
    vk, Device
};
use winit::window::Window;

use crate::config::Config;

pub struct Renderer {
    context: Context,
    debug_utils: (DebugUtils, vk::DebugUtilsMessengerEXT),
    device: Device,
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
    swapchain_details: SwapchainDetails,
    render_pass: vk::RenderPass,
}

impl Renderer {
    pub fn new(config: &Config, window: &Window) -> Renderer {
        let context = Context::new(config, window);

        debug::check_validation_layer_support(&context.entry);

        let debug_utils = debug::setup_debug_messenger(&context.entry, &context.instance);
        log::debug!("Vulkan Debug Utils Created");

        let (device, graphics_queue, present_queue) = create_logical_device_and_queues(&context);
        log::debug!("Logical Device, Graphics and Present Queue Created");

        let swapchain_details = SwapchainDetails::new(&config.window, &context, &device);
        let render_pass = create_render_pass(&device, &swapchain_details);

        Renderer {
            context,
            debug_utils,
            device,
            graphics_queue,
            present_queue,
            swapchain_details,
            render_pass
        }
    }

    pub fn cleanup(&self) {
        log::info!("Renderer Cleanup");
        self.swapchain_details.cleanup(&self.device);
        unsafe {       
            self.device.destroy_render_pass(self.render_pass, None);     
            self.device.destroy_device(None);
            self.debug_utils
                .0
                .destroy_debug_utils_messenger(self.debug_utils.1, None);
        }
        self.context.cleanup();
    }
}

fn create_logical_device_and_queues(context: &Context) -> (Device, vk::Queue, vk::Queue) {
    let (graphics_family_index, present_family_index) = context::find_queue_families(
        &context.instance,
        &context.surface,
        context.surface_khr,
        context.physical_device,
    );

    let graphics_family_index = graphics_family_index.unwrap();
    let present_family_index = present_family_index.unwrap();

    let queue_priorities = [1.0];

    let queue_create_infos = {
        let mut indices = vec![graphics_family_index, present_family_index];
        indices.dedup();

        indices
            .iter()
            .map(|index| {
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(*index)
                    .queue_priorities(&queue_priorities)
                    .build()
            })
            .collect::<Vec<_>>()
    };

    let device_features = vk::PhysicalDeviceFeatures::builder();
    log::debug!("Device Features: {:#?}", *device_features);

    let device_extensions = context::required_device_extension_names();
    let device_extensions_ptrs = device_extensions
            .iter()
            .map(|ext| ext.as_ptr())
            .collect::<Vec<_>>();



    let device_create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(&device_extensions_ptrs)
        .enabled_features(&device_features);

    let device = unsafe {
        context
            .instance
            .create_device(context.physical_device, &device_create_info, None)
            .expect("Failed to create logical device.")
    };

    let graphics_queue = unsafe { device.get_device_queue(graphics_family_index, 0) };
    let present_queue = unsafe { device.get_device_queue(present_family_index, 0) };

    (device, graphics_queue, present_queue)
}

fn create_render_pass(
    device: &Device,
    details: &SwapchainDetails,
) -> vk::RenderPass {
    let attachment_desc = vk::AttachmentDescription::builder()
        .format(details.format.format)
        .samples(vk::SampleCountFlags::TYPE_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .build();
    let attachment_descs = [attachment_desc];

    let attachment_ref = vk::AttachmentReference::builder()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .build();
    let attachment_refs = [attachment_ref];

    let subpass_desc = vk::SubpassDescription::builder()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(&attachment_refs)
        .build();
    let subpass_descs = [subpass_desc];

    let render_pass_info = vk::RenderPassCreateInfo::builder()
        .attachments(&attachment_descs)
        .subpasses(&subpass_descs)
        .build();

    unsafe { device.create_render_pass(&render_pass_info, None).unwrap() }
}
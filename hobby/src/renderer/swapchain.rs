use crate::renderer::base;
use crate::Result;

use log::info;
use std::sync::Arc;
use vulkano::device::{Device, Queue};
use vulkano::format::Format;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract};
use vulkano::image::swapchain::SwapchainImage;
use vulkano::image::ImageUsage;
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::swapchain::{
    Capabilities, ColorSpace, CompositeAlpha, PresentMode, SupportedPresentModes, Surface,
    Swapchain,
};
use vulkano::sync::SharingMode;
use winit::Window;

pub fn create_framebuffers(
    swap_chain_images: &Vec<Arc<SwapchainImage<Window>>>,
    render_pass: &Arc<RenderPassAbstract + Send + Sync>,
) -> Vec<Arc<FramebufferAbstract + Send + Sync>> {
    swap_chain_images
        .iter()
        .map(|image| {
            let fba: Arc<FramebufferAbstract + Send + Sync> = Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );
            fba
        })
        .collect::<Vec<_>>()
}

pub fn create_swapchain(
    instance: &Arc<Instance>,
    surface: &Arc<Surface<Window>>,
    physical_device_index: usize,
    device: &Arc<Device>,
    graphics_queue: &Arc<Queue>,
    present_queue: &Arc<Queue>,
    old_swapchain: Option<Arc<Swapchain<Window>>>,
) -> Result<(Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>)> {
    let physical_device = PhysicalDevice::from_index(&instance, physical_device_index).unwrap();
    let capabilities = surface.capabilities(physical_device)?;

    let surface_format = choose_swap_surface_format(&capabilities.supported_formats);
    let present_mode = choose_swap_present_mode(capabilities.present_modes);
    let extent = choose_swap_extent(&capabilities, surface.window());

    info!("Swapchain Details: ");
    info!("\tSurface Format: {:?}", surface_format);
    info!("\tPrenset Mode: {:?}", present_mode);
    info!("\tExtent: {:?}", extent);

    let mut image_count = capabilities.min_image_count + 1;
    if capabilities.max_image_count.is_some() && image_count > capabilities.max_image_count.unwrap()
    {
        image_count = capabilities.max_image_count.unwrap();
    }

    info!("\tNum Images: {}", image_count);

    let image_usage = ImageUsage {
        color_attachment: true,
        ..ImageUsage::none()
    };

    let indices = base::find_queue_families(&surface, &physical_device);

    let sharing: SharingMode = if indices.graphics_family != indices.present_family {
        vec![graphics_queue, present_queue].as_slice().into()
    } else {
        graphics_queue.into()
    };

    let (swap_chain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        image_count,
        surface_format.0, // TODO: color space?
        extent,
        1, // layers
        image_usage,
        sharing,
        capabilities.current_transform,
        CompositeAlpha::Opaque,
        present_mode,
        true, // clipped
        old_swapchain.as_ref(),
    )?;

    Ok((swap_chain, images))
}

fn choose_swap_surface_format(available_formats: &[(Format, ColorSpace)]) -> (Format, ColorSpace) {
    // NOTE: the 'preferred format' mentioned in the tutorial doesn't seem to be
    // queryable in Vulkano (no VK_FORMAT_UNDEFINED enum)
    *available_formats
        .iter()
        .find(|(format, color_space)| {
            *format == Format::B8G8R8A8Unorm && *color_space == ColorSpace::SrgbNonLinear
        })
        .unwrap_or_else(|| &available_formats[0])
}

fn choose_swap_present_mode(available_present_modes: SupportedPresentModes) -> PresentMode {
    if available_present_modes.mailbox {
        PresentMode::Mailbox
    } else if available_present_modes.immediate {
        PresentMode::Immediate
    } else {
        PresentMode::Fifo
    }
}

fn choose_swap_extent(capabilities: &Capabilities, window: &Window) -> [u32; 2] {
    let logical_size = window.get_inner_size().unwrap();
    let dpi_factor = window.get_hidpi_factor();
    let physical_size = logical_size.to_physical(dpi_factor);

    if let Some(current_extent) = capabilities.current_extent {
        return current_extent;
    } else {
        let mut actual_extent = [physical_size.width as u32, physical_size.height as u32];
        actual_extent[0] = capabilities.min_image_extent[0]
            .max(capabilities.max_image_extent[0].min(actual_extent[0]));
        actual_extent[1] = capabilities.min_image_extent[1]
            .max(capabilities.max_image_extent[1].min(actual_extent[1]));
        actual_extent
    }
}

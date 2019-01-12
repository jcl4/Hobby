use crate::Result;
use ash::{
    extensions::khr::{Surface, Swapchain},
    version::DeviceV1_0,
    vk,
};
use log::{debug, info};

#[derive(Clone)]
pub struct SwapchainData {
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_loader: Swapchain,
    pub surface_format: vk::SurfaceFormatKHR,
    pub extent: vk::Extent2D,
    pub image_views: Vec<vk::ImageView>,
}

pub fn create_swapchain_and_image_views(
    surface_loader: Surface,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    instance: &ash::Instance,
    device: &ash::Device,
) -> Result<(SwapchainData)> {
    let surface_format = get_surface_format(surface_loader.clone(), physical_device, surface)?;
    let caps = unsafe {
        surface_loader.get_physical_device_surface_capabilities(physical_device, surface)?
    };

    let mut image_count = caps.min_image_count + 1;
    if caps.max_image_count > 0 && image_count > caps.max_image_count {
        image_count = caps.max_image_count;
    }

    info!("Swap Chain Image Count: {}", image_count);

    let pre_transform = if caps
        .supported_transforms
        .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
    {
        vk::SurfaceTransformFlagsKHR::IDENTITY
    } else {
        caps.current_transform
    };

    info!("Pre Transform: {}", pre_transform);

    let present_mode = get_present_mode(surface_loader, physical_device, surface)?;

    info!("Present Mode: {}", present_mode);

    let swapchain_loader = Swapchain::new(instance, device);

    let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(caps.current_extent)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(pre_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .image_array_layers(1);

    let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None)? };
    info!("Swapchain Created");
    let image_views = create_image_views(
        swapchain_loader.clone(),
        swapchain,
        surface_format.format,
        device,
    )?;

    Ok(SwapchainData {
        swapchain,
        swapchain_loader,
        surface_format,
        extent: caps.current_extent,
        image_views,
    })
}

fn create_image_views(
    swapchain_loader: Swapchain,
    swapchain: vk::SwapchainKHR,
    format: vk::Format,
    device: &ash::Device,
) -> Result<Vec<vk::ImageView>> {
    let component_mapping = vk::ComponentMapping::builder()
        .r(vk::ComponentSwizzle::R)
        .g(vk::ComponentSwizzle::G)
        .b(vk::ComponentSwizzle::B)
        .a(vk::ComponentSwizzle::A)
        .build();

    let subresource_range = vk::ImageSubresourceRange::builder()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
        .build();

    let images = unsafe { swapchain_loader.get_swapchain_images(swapchain)? };
    let image_views: Vec<vk::ImageView> = images
        .into_iter()
        .map(|image| {
            let create_info = vk::ImageViewCreateInfo::builder()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(format)
                .components(component_mapping)
                .subresource_range(subresource_range)
                .build();

            unsafe { device.create_image_view(&create_info, None).unwrap() }
        })
        .collect();
    Ok(image_views)
}

fn get_present_mode(
    surface_loader: Surface,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<vk::PresentModeKHR> {
    let mut present_mode = vk::PresentModeKHR::FIFO;

    let present_modes = unsafe {
        surface_loader.get_physical_device_surface_present_modes(physical_device, surface)?
    };

    for pm in present_modes {
        if pm == vk::PresentModeKHR::MAILBOX {
            return Ok(pm);
        } else if pm == vk::PresentModeKHR::IMMEDIATE {
            present_mode = pm;
        }
    }

    Ok(present_mode)
}

fn get_surface_format(
    surface_loader: Surface,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<(vk::SurfaceFormatKHR)> {
    let mut surface_formats =
        unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface)? };

    let surface_format = surface_formats.remove(0);

    info!(
        "Swap Chain Format: {}, Color Space: {}",
        surface_format.format, surface_format.color_space
    );

    Ok(surface_format)
}

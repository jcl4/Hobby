use super::context::Context;
use crate::config::WindowConfig;

use ash::{
    extensions::khr::{Surface, Swapchain},
    version::DeviceV1_0,
    vk, Device,
};

pub struct SwapchainDetails {
    pub swapchain: Swapchain,
    pub swapchain_khr: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
}

impl SwapchainDetails {
    pub fn new(
        window_config: &WindowConfig,
        context: &Context,
        device: &Device,
    ) -> SwapchainDetails {
        let capabilities = get_device_capabilies(context);

        let format = choose_swapchain_surface_format(context);
        let present_mode = choose_swapchain_surface_present_mode(context, window_config.vsync);
        let extent = choose_swapchain_extent(context, window_config);
        let image_count = get_image_count(context);
        log::debug!(
            "Creating swapchain.\n\tFormat: {:?}\n\tColorSpace: {:?}\n\tPresentMode: {:?}\n\tExtent: {:?}\n\tImageCount: {}",
            format.format,
            format.color_space,
            present_mode,
            extent,
            image_count,
        );

        let family_indices = [
            context.queue_family_indices.graphics,
            context.queue_family_indices.present,
        ];

        let mut create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(context.surface_khr)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        create_info = if family_indices[0] != family_indices[1] {
            create_info
                .image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(&family_indices)
        } else {
            create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        };

        let swapchain = Swapchain::new(&context.instance, device);
        let swapchain_khr = unsafe { swapchain.create_swapchain(&create_info, None).unwrap() };
        let images = unsafe { swapchain.get_swapchain_images(swapchain_khr).unwrap() };

        let image_views = create_image_views(device, &images, format.format);
        
        SwapchainDetails {
            swapchain,
            swapchain_khr,
            images,
            image_views
        }
    }

    pub fn cleanup(&self, device: &Device) {
        unsafe {
            self.image_views.iter().for_each(|v| device.destroy_image_view(*v, None));
            self.swapchain.destroy_swapchain(self.swapchain_khr, None);
        }
    }
}

/// Choose the swapchain surface format.
///
/// Will choose B8G8R8A8_UNORM/SRGB_NONLINEAR if possible or
/// the first available otherwise.
fn choose_swapchain_surface_format(context: &Context) -> vk::SurfaceFormatKHR {
    let formats = unsafe {
        context
            .surface
            .get_physical_device_surface_formats(context.physical_device, context.surface_khr)
            .expect("Unable to get surface formats")
    };

    if formats.len() == 1 && formats[0].format == vk::Format::UNDEFINED {
        return vk::SurfaceFormatKHR {
            format: vk::Format::B8G8R8A8_UNORM,
            color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
        };
    }

    *formats
        .iter()
        .find(|format| {
            format.format == vk::Format::B8G8R8A8_UNORM
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        })
        .unwrap_or(&formats[0])
}

fn choose_swapchain_surface_present_mode(context: &Context, vsync: bool) -> vk::PresentModeKHR {
    let present_modes = unsafe {
        context
            .surface
            .get_physical_device_surface_present_modes(context.physical_device, context.surface_khr)
            .expect("unable to get present modes")
    };

    if !vsync {
        vk::PresentModeKHR::IMMEDIATE
    } else if present_modes.contains(&vk::PresentModeKHR::MAILBOX) {
        vk::PresentModeKHR::MAILBOX
    } else if present_modes.contains(&vk::PresentModeKHR::FIFO) {
        vk::PresentModeKHR::FIFO
    } else {
        vk::PresentModeKHR::IMMEDIATE
    }
}

fn choose_swapchain_extent(context: &Context, window_config: &WindowConfig) -> vk::Extent2D {
    let capabilities = get_device_capabilies(context);
    if capabilities.current_extent.width != std::u32::MAX {
        return capabilities.current_extent;
    }

    let window_width = window_config.width;
    let widnow_height = window_config.height;

    let min = capabilities.min_image_extent;
    let max = capabilities.max_image_extent;
    let width = window_width.min(max.width).max(min.width);
    let height = widnow_height.min(max.height).max(min.height);
    vk::Extent2D { width, height }
}

fn get_device_capabilies(context: &Context) -> vk::SurfaceCapabilitiesKHR {
    unsafe {
        context
            .surface
            .get_physical_device_surface_capabilities(context.physical_device, context.surface_khr)
            .expect("Unable to get surface capabilities")
    }
}

fn get_image_count(context: &Context) -> u32 {
    let capabilities = get_device_capabilies(context);

    let max = capabilities.max_image_count;
    let mut preferred = capabilities.min_image_count + 1;
    if max > 0 && preferred > max {
        preferred = max;
    }
    preferred
}

fn create_image_views(
    device: &Device,
    swapchain_images: &[vk::Image],
    swapchain_format: vk::Format,
) -> Vec<vk::ImageView> {
    swapchain_images
        .into_iter()
        .map(|image| {
            let create_info = vk::ImageViewCreateInfo::builder()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(swapchain_format)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });

            unsafe { device.create_image_view(&create_info, None).unwrap() }
        })
        .collect::<Vec<_>>()
}

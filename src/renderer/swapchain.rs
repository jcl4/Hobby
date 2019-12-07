use ash::{
    extensions::khr::{Surface, Swapchain},
    version::DeviceV1_0,
    vk,
};

use super::context::{Context, QueueFamiliesIndices};

pub struct SwapchainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupportDetails {
    pub fn new(device: vk::PhysicalDevice, surface: &Surface, surface_khr: vk::SurfaceKHR) -> Self {
        let capabilities = unsafe {
            surface
                .get_physical_device_surface_capabilities(device, surface_khr)
                .unwrap()
        };

        let formats = unsafe {
            surface
                .get_physical_device_surface_formats(device, surface_khr)
                .unwrap()
        };

        let present_modes = unsafe {
            surface
                .get_physical_device_surface_present_modes(device, surface_khr)
                .unwrap()
        };

        Self {
            capabilities,
            formats,
            present_modes,
        }
    }

    pub fn get_ideal_swapchain_properties(&self, width: u32, height: u32) -> SwapchainProperties {
        let format = Self::choose_swapchain_surface_format(&self.formats);
        let present_mode = Self::choose_swapchain_surface_present_mode(&self.present_modes);
        let extent = Self::choose_swapchain_extent(self.capabilities, width, height);
        SwapchainProperties {
            format,
            present_mode,
            extent,
        }
    }

    /// Choose the swapchain surface format.
    ///
    /// Will choose B8G8R8A8_UNORM/SRGB_NONLINEAR if possible or
    /// the first available otherwise.
    fn choose_swapchain_surface_format(
        available_formats: &[vk::SurfaceFormatKHR],
    ) -> vk::SurfaceFormatKHR {
        if available_formats.len() == 1 && available_formats[0].format == vk::Format::UNDEFINED {
            return vk::SurfaceFormatKHR {
                format: vk::Format::B8G8R8A8_UNORM,
                color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            };
        }

        *available_formats
            .iter()
            .find(|format| {
                format.format == vk::Format::B8G8R8A8_UNORM
                    && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .unwrap_or(&available_formats[0])
    }

    /// Choose the swapchain present mode.
    ///
    /// Will favor MAILBOX if present otherwise FIFO.
    /// If none is present it will fallback to IMMEDIATE.
    fn choose_swapchain_surface_present_mode(
        available_present_modes: &[vk::PresentModeKHR],
    ) -> vk::PresentModeKHR {
        if available_present_modes.contains(&vk::PresentModeKHR::MAILBOX) {
            vk::PresentModeKHR::MAILBOX
        } else if available_present_modes.contains(&vk::PresentModeKHR::IMMEDIATE) {
            vk::PresentModeKHR::IMMEDIATE
        } else {
            vk::PresentModeKHR::FIFO
        }
    }

    /// Choose the swapchain extent.
    ///
    /// If a current extent is defined it will be returned.
    /// Otherwise the surface extent clamped between the min
    /// and max image extent will be returned.
    fn choose_swapchain_extent(
        capabilities: vk::SurfaceCapabilitiesKHR,
        width: u32,
        height: u32,
    ) -> vk::Extent2D {
        if capabilities.current_extent.width != std::u32::MAX {
            return capabilities.current_extent;
        }

        let min = capabilities.min_image_extent;
        let max = capabilities.max_image_extent;
        let width = width.min(max.width).max(min.width);
        let height = height.min(max.height).max(min.height);
        vk::Extent2D { width, height }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SwapchainProperties {
    pub format: vk::SurfaceFormatKHR,
    pub present_mode: vk::PresentModeKHR,
    pub extent: vk::Extent2D,
}

pub struct SwapchainData {
    swapchain: Swapchain,
    swapchain_khr: vk::SwapchainKHR,
    properties: SwapchainProperties,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
}

pub fn create_swapchain_data(
    context: &Context,
    queue_families_indices: QueueFamiliesIndices,
    width: u32,
    height: u32,
) -> SwapchainData {
    let details = SwapchainSupportDetails::new(
        context.physical_device(),
        context.surface(),
        context.surface_khr(),
    );
    let properties = details.get_ideal_swapchain_properties(width, height);

    let format = properties.format;
    let present_mode = properties.present_mode;
    let extent = properties.extent;
    let image_count = {
        let max = details.capabilities.max_image_count;
        let mut preferred = details.capabilities.min_image_count + 1;
        if max > 0 && preferred > max {
            preferred = max;
        }
        preferred
    };

    log::info!(
            "Creating swapchain.\n\tFormat: {:?}\n\tColorSpace: {:?}\n\tPresentMode: {:?}\n\tExtent: {:?}\n\tImageCount: {:?}",
            format.format,
            format.color_space,
            present_mode,
            extent,
            image_count,
        );

    println!(
            "Creating swapchain.\n\tFormat: {:?}\n\tColorSpace: {:?}\n\tPresentMode: {:?}\n\tExtent: {:?}\n\tImageCount: {:?}",
            format.format,
            format.color_space,
            present_mode,
            extent,
            image_count,
        );

    let graphics = queue_families_indices.graphics_index();
    let present = queue_families_indices.present_index();
    let families_indices = [graphics, present];

    let create_info = {
        let mut builder = vk::SwapchainCreateInfoKHR::builder()
            .surface(context.surface_khr())
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT);

        builder = if graphics != present {
            builder
                .image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(&families_indices)
        } else {
            builder.image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        };

        builder
            .pre_transform(details.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .build()
        // .old_swapchain() We don't have an old swapchain but can't pass null
    };

    let swapchain = Swapchain::new(context.instance(), context.device());
    let swapchain_khr = unsafe { swapchain.create_swapchain(&create_info, None).unwrap() };
    let images = unsafe { swapchain.get_swapchain_images(swapchain_khr).unwrap() };
    let image_views = create_swapchain_image_views(context.device(), &images, properties);

    SwapchainData {
        swapchain,
        swapchain_khr,
        properties,
        images,
        image_views,
    }
}

fn create_swapchain_image_views(
    device: &ash::Device,
    swapchain_images: &[vk::Image],
    swapchain_properties: SwapchainProperties,
) -> Vec<vk::ImageView> {
    swapchain_images
        .iter()
        .map(|image| {
            create_image_view(
                device,
                *image,
                1,
                swapchain_properties.format.format,
                vk::ImageAspectFlags::COLOR,
            )
        })
        .collect::<Vec<_>>()
}

fn create_image_view(
    device: &ash::Device,
    image: vk::Image,
    mip_levels: u32,
    format: vk::Format,
    aspect_mask: vk::ImageAspectFlags,
) -> vk::ImageView {
    let create_info = vk::ImageViewCreateInfo::builder()
        .image(image)
        .view_type(vk::ImageViewType::TYPE_2D)
        .format(format)
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask,
            base_mip_level: 0,
            level_count: mip_levels,
            base_array_layer: 0,
            layer_count: 1,
        })
        .build();

    unsafe { device.create_image_view(&create_info, None).unwrap() }
}

pub fn cleanup_swapchain(device: &ash::Device, swapchain_data: &SwapchainData) {
    unsafe {
        swapchain_data
            .image_views
            .iter()
            .for_each(|v| device.destroy_image_view(*v, None));
        swapchain_data
            .swapchain
            .destroy_swapchain(swapchain_data.swapchain_khr, None);
    }
    log::info!("Swapchain Cleaned");
}
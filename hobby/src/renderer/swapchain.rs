use std::cmp;

use voodoo as vd;

use Result;

pub fn create_swapchain(
    surface: &vd::SurfaceKhr,
    device: &vd::Device,
    graphics_queue_family: u32,
    present_queue_family: u32,
    v_sync: bool,
    window_size: &vd::Extent2d,
    old_swapchain: Option<&vd::SwapchainKhr>,
) -> Result<vd::SwapchainKhr> {
    let swapchain_details = vd::SwapchainSupportDetails::new(surface, device.physical_device())?;
    let surface_format = choose_swap_surface_format(&swapchain_details.formats);

    let present_mode = choose_swap_present_mode(&swapchain_details.present_modes, v_sync);

    let extent = choose_swap_extent(&swapchain_details.capabilities, window_size.clone());
    let image_count = get_image_count(&swapchain_details.capabilities);

    let indices;

    let mut bldr = vd::SwapchainKhr::builder();
    bldr.surface(surface)
        .min_image_count(image_count)
        .image_format(surface_format.format())
        .image_color_space(surface_format.color_space())
        .image_extent(extent.clone())
        .image_array_layers(1)
        .image_usage(vd::ImageUsageFlags::COLOR_ATTACHMENT)
        .pre_transform(swapchain_details.capabilities.current_transform())
        .composite_alpha(vd::CompositeAlphaFlagsKhr::OPAQUE)
        .present_mode(present_mode)
        .clipped(true);

    if let Some(old_sc) = old_swapchain {
        bldr.old_swapchain(old_sc.handle());
    }

    if graphics_queue_family != present_queue_family {
        indices = [graphics_queue_family, present_queue_family];
        bldr.image_sharing_mode(vd::SharingMode::Concurrent);
        bldr.queue_family_indices(&indices);
        info!{"Swapchain Image Sharing Mode: {:?}", vd::SharingMode::Concurrent};
    } else {
        info!{"Swapchain Image Sharing Mode: {:?}", vd::SharingMode::Exclusive};
        bldr.image_sharing_mode(vd::SharingMode::Exclusive);
    }

    let swapchain = bldr.build(device.clone())?;

    Ok(swapchain)
}

pub fn create_image_views(swapchain: &vd::SwapchainKhr) -> Vec<vd::ImageView> {
    let image_views: Vec<vd::ImageView> = swapchain
        .images()
        .iter()
        .map(|image| {
            vd::ImageView::builder()
                .image(image)
                .view_type(vd::ImageViewType::Type2d)
                .format(swapchain.image_format())
                .components(vd::ComponentMapping::default())
                .subresource_range(
                    vd::ImageSubresourceRange::builder()
                        .aspect_mask(vd::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .build(),
                ).build(swapchain.device().clone(), Some(swapchain.clone()))
                .unwrap()
        }).collect::<Vec<vd::ImageView>>();
    image_views
}

fn get_image_count(capabilities: &vd::SurfaceCapabilitiesKhr) -> u32 {
    let mut image_count = capabilities.min_image_count() + 1;

    if capabilities.max_image_count() > 0 && image_count > capabilities.max_image_count() {
        image_count = capabilities.max_image_count();
    }

    info!("Number of images in swapchain: {}", image_count);

    image_count
}

fn choose_swap_extent(
    capabilities: &vd::SurfaceCapabilitiesKhr,
    window_size: vd::Extent2d,
) -> vd::Extent2d {
    let mut actual_extent = window_size;

    if capabilities.current_extent().width() != u32::max_value() {
        actual_extent = capabilities.current_extent().clone();
    } else {
        let actual_width = actual_extent.width();
        let actual_height = actual_extent.height();

        let width = cmp::max(
            capabilities.min_image_extent().width(),
            cmp::min(capabilities.max_image_extent().width(), actual_width),
        );
        let height = cmp::max(
            capabilities.min_image_extent().height(),
            cmp::min(capabilities.max_image_extent().height(), actual_height),
        );

        actual_extent.set_width(width);
        actual_extent.set_height(height);
    }

    info!(
        "Swapchain Extent Size: W: {}, H: {}",
        actual_extent.width(),
        actual_extent.height()
    );
    actual_extent
}

fn choose_swap_present_mode(
    available_present_modes: &[vd::PresentModeKhr],
    v_sync: bool,
) -> vd::PresentModeKhr {
    let mut best_mode = vd::PresentModeKhr::FifoKhr;

    for &mode in available_present_modes {
        if v_sync {
            if mode == vd::PresentModeKhr::MailboxKhr {
                best_mode = vd::PresentModeKhr::MailboxKhr;
            }
        } else {
            if mode == vd::PresentModeKhr::ImmediateKhr {
                best_mode = vd::PresentModeKhr::ImmediateKhr;
            }
        }
    }

    info!{"Present mode selected: {:?}", best_mode};

    best_mode
}

fn choose_swap_surface_format(available_formats: &[vd::SurfaceFormatKhr]) -> vd::SurfaceFormatKhr {
    if available_formats.len() == 1 && available_formats[0].format() == vd::Format::Undefined {
        info!("Prefered Surface Format Undefined");
        info!("Surface Format: {:?}", vd::Format::B8G8R8A8Unorm);
        info!("Color Space: {:?}", vd::ColorSpaceKhr::SrgbNonlinearKhr);

        return vd::SurfaceFormatKhr::builder()
            .format(vd::Format::B8G8R8A8Unorm)
            .color_space(vd::ColorSpaceKhr::SrgbNonlinearKhr)
            .build();
    }

    for available_format in available_formats {
        if available_format.format() == vd::Format::B8G8R8A8Unorm
            && available_format.color_space() == vd::ColorSpaceKhr::SrgbNonlinearKhr
        {
            info!("Prefered surface format found");
            info!("Surface Format: {:?}", vd::Format::B8G8R8A8Unorm);
            info!("Color Space: {:?}", vd::ColorSpaceKhr::SrgbNonlinearKhr);
            return vd::SurfaceFormatKhr::builder()
                .format(vd::Format::B8G8R8A8Unorm)
                .color_space(vd::ColorSpaceKhr::SrgbNonlinearKhr)
                .build();
        }
    }

    info!("Prefered format not found, using first available");
    info!("Surface Format: {:?}", available_formats[0].format());
    info!("Color Space: {:?}", available_formats[0].color_space());

    vd::SurfaceFormatKhr::builder()
        .format(available_formats[0].format())
        .color_space(available_formats[0].color_space())
        .build()
}

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use voodoo as vd;
use winit::{dpi::PhysicalSize, EventsLoop, Window, WindowBuilder};

use AppInfo;
use Result;
use WindowSettings;

static REQUIRED_DEVICE_EXTENSIONS: &[&str] = &["VK_KHR_swapchain"];

pub fn init_window(events_loop: &EventsLoop, window_settings: &WindowSettings) -> Window {
    let monitor = events_loop.get_primary_monitor();
    let dpi = monitor.get_hidpi_factor();

    let physical_size = PhysicalSize::new(window_settings.width, window_settings.height);
    let logical_size = physical_size.to_logical(dpi);

    let window = WindowBuilder::new()
        .with_dimensions(logical_size)
        .with_title(window_settings.title.clone())
        .build(events_loop)
        .expect("Unable to create window");

    let size = window.get_inner_size().expect("Unable to get window size");

    info!("Built Window");
    info!("\tWindow Size: {:?}", size.to_physical(dpi));

    window
}

pub fn create_instance(app_info: &AppInfo) -> Result<vd::Instance> {
    let engine_name = CString::new(env!("CARGO_PKG_NAME"))?;

    let engine_version = vd::Version::new(
        env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
    );

    let app_name = CString::new(app_info.app_name.clone())?;
    let app_version = vd::Version::new(
        app_info.app_version.0,
        app_info.app_version.1,
        app_info.app_version.2,
    );

    let app_info = vd::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(app_version)
        .engine_name(&engine_name)
        .engine_version(engine_version)
        .build();

    info!("Vulkan Application Info:");
    info!("\tEngine Name: {:?}", engine_name);
    info!("\tEngine Version: {}", engine_version);
    info!("\tApplication Name: {:?}", app_name);
    info!("\tApplication Version: {}", app_version);

    let loader = vd::Loader::new()?;
    let instance_extensions = loader.enumerate_instance_extension_properties()?;

    info!("Available instance extensions:");

    for ext in instance_extensions.iter() {
        let name = (&ext.extensionName) as *const c_char;
        unsafe {
            info!(
                "\t'{}' (version: {})",
                CStr::from_ptr(name).to_str().unwrap(),
                ext.specVersion
            );
        }
    }

    let instance = vd::Instance::builder()
        .application_info(&app_info)
        .enabled_extensions(&instance_extensions)
        .build(loader)?;

    info!("Instance Created");

    Ok(instance)
}

pub fn create_surface(instance: vd::Instance, window: &winit::Window) -> Result<vd::SurfaceKhr> {
    let surface = voodoo_winit::create_surface(instance, window)?;
    info!("Surface Created");
    Ok(surface)
}

pub fn pick_physical_device(instance: &vd::Instance) -> Result<vd::PhysicalDevice> {
    let physical_devices = instance.physical_devices()?;

    let preferred_device = rank_devices(physical_devices.to_vec());

    if let Some(preferred_device) = preferred_device {
        let properties = preferred_device.properties();
        info!(
            "Physical Device: {:#?}, Type: {:#?}",
            properties.device_name(),
            properties.device_type()
        );
        info!("Supported API Version: {:?}", properties.api_version());

        Ok(preferred_device)
    } else {
        bail!("Unable to find suitable device");
    }
}

fn rank_devices(devices: Vec<vd::PhysicalDevice>) -> Option<vd::PhysicalDevice> {
    let mut ranking = BTreeMap::new();

    for device in devices {
        let mut device_score = 0;

        let device_type = device.properties().device_type();

        match device_type {
            vd::PhysicalDeviceType::IntegratedGpu => device_score += 10,
            vd::PhysicalDeviceType::DiscreteGpu => device_score += 50,
            vd::PhysicalDeviceType::VirtualGpu => device_score += 20,
            vd::PhysicalDeviceType::Cpu => device_score += 5,
            vd::PhysicalDeviceType::Other => device_score += 0,
        }

        ranking.insert(device_score, device);
    }

    let mut temp_vec = Vec::new();
    for (_, device) in ranking.into_iter().rev() {
        temp_vec.push(device);
    }

    if temp_vec.len() > 0 {
        Some(temp_vec[0].clone())
    } else {
        None
    }
}

pub fn find_queue_families(
    physical_device: &vd::PhysicalDevice,
    surface: &vd::SurfaceKhr,
) -> Result<(u32, u32)> {
    let queue_families = physical_device.queue_family_properties()?;
    let mut graphics_family_idx = None;
    let mut present_family_idx = None;

    let mut i = 0;
    for queue_family in queue_families {
        if queue_family.queue_count() > 0 && queue_family
            .queue_flags()
            .contains(vd::QueueFlags::GRAPHICS)
        {
            graphics_family_idx = Some(i);
        }

        let presentation_support = physical_device.surface_support_khr(i, surface)?;
        if queue_family.queue_count() > 0 && presentation_support {
            present_family_idx = Some(i);
        }

        if let (Some(gf_idx), Some(pf_idx)) = (graphics_family_idx, present_family_idx) {
            return Ok((gf_idx, pf_idx));
        }
        i += 1;
    }
    bail!("Unable to find graphics and/or present queue family support");
}

pub fn create_device(
    physical_device: vd::PhysicalDevice,
    graphics_queue_family: u32,
    present_queue_family: u32,
) -> Result<(vd::Device, vd::QueueHandle, vd::QueueHandle)> {
    let unique_qfi: BTreeSet<u32> = [graphics_queue_family, present_queue_family]
        .iter()
        .map(|&i| i)
        .collect();

    let queue_priorities = [1.0];
    let queue_create_infos: Vec<_> = unique_qfi
        .iter()
        .map(|&idx| {
            vd::DeviceQueueCreateInfo::builder()
                .queue_family_index(idx)
                .queue_priorities(&queue_priorities)
                .build()
        }).collect();

    let features = vd::PhysicalDeviceFeatures::builder()
        .sampler_anisotropy(true)
        .build();

    let device = vd::Device::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(REQUIRED_DEVICE_EXTENSIONS)
        .enabled_features(&features)
        .build(physical_device)?;
    info!("Logical Device Created");

    let (graphics_queue, present_queue) =
        get_queues(&device, graphics_queue_family, present_queue_family);

    Ok((device, graphics_queue, present_queue))
}

fn get_queues(
    device: &vd::Device,
    graphics_queue_family: u32,
    present_queue_family: u32,
) -> (vd::QueueHandle, vd::QueueHandle) {
    info!("Graphics Queue Family ID: {}", graphics_queue_family);
    info!("Present Queue Family ID: {}", present_queue_family);

    let graphics_queue = device
        .get_device_queue(graphics_queue_family, 0)
        .expect("Unable to create graphics queue handle");
    let present_queue = device
        .get_device_queue(present_queue_family, 0)
        .expect("Unable to create present queue hadnle");

    (graphics_queue, present_queue)
}

pub fn create_render_pass(swapchain: &vd::SwapchainKhr) -> Result<vd::RenderPass> {
    let format = swapchain.image_format();
    let device = swapchain.device();
    let color_attachment = vd::AttachmentDescription::builder()
        .format(format.clone())
        .samples(vd::SampleCountFlags::COUNT_1)
        .load_op(vd::AttachmentLoadOp::Clear)
        .store_op(vd::AttachmentStoreOp::Store)
        .initial_layout(vd::ImageLayout::Undefined)
        .final_layout(vd::ImageLayout::PresentSrcKhr)
        .build();

    let color_attachment_ref = vd::AttachmentReference::builder()
        .attachment(0)
        .layout(vd::ImageLayout::ColorAttachmentOptimal)
        .build();

    let color_attachments = [color_attachment_ref];

    let subpass = vd::SubpassDescription::builder()
        .pipeline_bind_point(vd::PipelineBindPoint::Graphics)
        .color_attachments(&color_attachments)
        .build();

    let dependency = vd::SubpassDependency::builder()
        .src_subpass(vd::SUBPASS_EXTERNAL)
        .dst_subpass(0)
        .src_stage_mask(vd::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_stage_mask(vd::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(
            vd::AccessFlags::COLOR_ATTACHMENT_READ | vd::AccessFlags::COLOR_ATTACHMENT_WRITE,
        ).build();

    let render_pass = vd::RenderPass::builder()
        .attachments(&[color_attachment])
        .subpasses(&[subpass])
        .dependencies(&[dependency])
        .build(device.clone())?;

    Ok(render_pass)
}

pub fn create_frame_buffers(
    image_views: &Vec<vd::ImageView>,
    extent: &vd::Extent2d,
    render_pass: &vd::RenderPass,
) -> Result<Vec<vd::Framebuffer>> {
    let device = render_pass.device();

    let framebuffers = image_views
        .iter()
        .map(|image_view| {
            vd::Framebuffer::builder()
                .attachments(&[image_view])
                .render_pass(render_pass)
                .width(extent.width())
                .height(extent.height())
                .layers(1)
                .build(device.clone())
                .unwrap()
        }).collect::<Vec<_>>();

    Ok(framebuffers)
}

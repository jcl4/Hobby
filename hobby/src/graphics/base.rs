use super::swapchain::SwapchainData;
use crate::{AppInfo, Result, Version, WindowSettings};
use ash;
use ash::{
    extensions::{
        ext::DebugReport,
        khr::{Surface, XlibSurface},
    },
    version::{DeviceV1_0, EntryV1_0, InstanceV1_0},
    vk,
};
use failure::bail;
use log::{debug, error, info, warn};
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
};
use winit::{dpi::PhysicalSize, EventsLoop, Window, WindowBuilder};

type DeviceExtensions = [&'static str; 1];

const DEVICE_EXTENSIONS: DeviceExtensions = ["VK_KHR_swapchain\0"];

pub(crate) fn required_extensions() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugReport::name().as_ptr(),
    ]
}

pub(crate) struct QueueData {
    pub(crate) graphics_queue_family: u32,
    pub(crate) graphics_queue: vk::Queue,
    pub(crate) _present_queue_family: u32,
    pub(crate) present_queue: vk::Queue,
}

pub(crate) unsafe extern "system" fn vulkan_debug_callback(
    flags: vk::DebugReportFlagsEXT,
    _: vk::DebugReportObjectTypeEXT,
    _: u64,
    _: usize,
    _: i32,
    _: *const c_char,
    p_message: *const c_char,
    _: *mut c_void,
) -> u32 {
    if flags.contains(vk::DebugReportFlagsEXT::INFORMATION)
        || flags.contains(vk::DebugReportFlagsEXT::PERFORMANCE_WARNING)
    {
        info!("{:?}", CStr::from_ptr(p_message));
    } else if flags.contains(vk::DebugReportFlagsEXT::WARNING) {
        warn!("{:?}", CStr::from_ptr(p_message));
    } else if flags.contains(vk::DebugReportFlagsEXT::DEBUG) {
        debug!("{:?}", CStr::from_ptr(p_message));
    } else if flags.contains(vk::DebugReportFlagsEXT::ERROR) {
        error!("{:?}", CStr::from_ptr(p_message));
    }

    vk::FALSE
}

pub(crate) fn create_window(
    events_loop: &EventsLoop,
    window_settings: &WindowSettings,
) -> Result<Window> {
    let monitor = events_loop.get_primary_monitor();
    let dpi = monitor.get_hidpi_factor();

    let physical_size = PhysicalSize::new(window_settings.width, window_settings.height);
    let logical_size = physical_size.to_logical(dpi);

    let window = WindowBuilder::new()
        .with_dimensions(logical_size)
        .with_title(window_settings.title.clone())
        .build(events_loop)?;

    let size = window.get_inner_size().unwrap().to_physical(dpi);
    let size: (u32, u32) = size.into();

    info!("Built Window");
    info!("\tWindow Size: {:?}", size);

    Ok(window)
}

pub(crate) fn create_instance(app_info: &AppInfo, entry: &ash::Entry) -> Result<ash::Instance> {
    let app_name = CString::new(app_info.app_name.clone())?;
    let engine_name = CString::new(env!("CARGO_PKG_NAME"))?;

    let engine_version = Version {
        major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
    };

    let api_version = Version::new(1, 1, 92);

    let application_info = vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(app_info.app_version.vulkan_version())
        .engine_name(&engine_name)
        .engine_version(engine_version.vulkan_version())
        .api_version(api_version.vulkan_version());

    let extension_names = required_extensions();

    let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
    let layer_names_raw: Vec<*const i8> = layer_names
        .iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extension_names)
        .enabled_layer_names(&layer_names_raw);

    let instance: ash::Instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create instance!")
    };
    info!("Instance Created");
    Ok(instance)
}

pub(crate) fn setup_debug_callback(
    entry: &ash::Entry,
    instance: &ash::Instance,
) -> Result<(vk::DebugReportCallbackEXT, DebugReport)> {
    let debug_info = vk::DebugReportCallbackCreateInfoEXT::builder()
        .flags(
            vk::DebugReportFlagsEXT::ERROR
                | vk::DebugReportFlagsEXT::WARNING
                | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING
                | vk::DebugReportFlagsEXT::DEBUG,
            // | vk::DebugReportFlagsEXT::INFORMATION,
        )
        .pfn_callback(Some(vulkan_debug_callback));

    let debug_loader = DebugReport::new(entry, instance);

    let debug_callback = unsafe { debug_loader.create_debug_report_callback(&debug_info, None)? };

    Ok((debug_callback, debug_loader))
}

pub(crate) fn create_framebuffers(
    swapchain_data: &SwapchainData,
    render_pass: vk::RenderPass,
    device: &ash::Device,
) -> Result<Vec<vk::Framebuffer>> {
    let frame_buffers: Vec<vk::Framebuffer> = swapchain_data
        .image_views
        .iter()
        .map(|&image_view| {
            let attachements = [image_view];
            let framebuffer_create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass)
                .attachments(&attachements)
                .width(swapchain_data.extent.width)
                .height(swapchain_data.extent.height)
                .layers(1);

            unsafe {
                device
                    .create_framebuffer(&framebuffer_create_info, None)
                    .unwrap()
            }
        })
        .collect();

    Ok(frame_buffers)
}

pub(crate) unsafe fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window: &winit::Window,
) -> Result<vk::SurfaceKHR> {
    use std::ptr;
    use winit::os::unix::WindowExt;

    let x11_display = window.get_xlib_display().unwrap();
    let x11_window = window.get_xlib_window().unwrap();
    let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
        s_type: vk::StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
        p_next: ptr::null(),
        flags: Default::default(),
        window: x11_window as vk::Window,
        dpy: x11_display as *mut vk::Display,
    };
    let xlib_surface_loader = XlibSurface::new(entry, instance);
    let surface = xlib_surface_loader.create_xlib_surface(&x11_create_info, None)?;
    Ok(surface)
}

pub(crate) fn create_device_and_queues(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    surface_loader: &Surface,
    surface: vk::SurfaceKHR,
    // ) -> Result<()> {
) -> Result<(ash::Device, QueueData)> {
    let (graphics_queue_family, present_queue_family) =
        get_queue_families(&instance, physical_device, &surface_loader, surface)?;

    let mut queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = vec![];

    if graphics_queue_family == present_queue_family {
        let priorities = [1.0, 1.0];

        let queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(graphics_queue_family)
            .queue_priorities(&priorities)
            .build();
        queue_create_infos.push(queue_create_info);
    } else {
        let queue_families = [graphics_queue_family, present_queue_family];
        let priorities = [[1.0], [1.0]];

        queue_create_infos = queue_families
            .iter()
            .zip(priorities.iter())
            .map(|(queue_family, priority)| {
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(*queue_family)
                    .queue_priorities(priority)
                    .build()
            })
            .collect();
    }

    let physical_device_features = vk::PhysicalDeviceFeatures::default();

    let extensions: Vec<*const i8> = DEVICE_EXTENSIONS
        .iter()
        .map(|exension| {
            CStr::from_bytes_with_nul(exension.as_bytes())
                .unwrap()
                .as_ptr()
        })
        .collect();

    let device_create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(&extensions)
        .enabled_features(&physical_device_features);

    let device = unsafe { instance.create_device(physical_device, &device_create_info, None)? };
    let (graphics_queue, present_queue) =
        get_queues(&device, graphics_queue_family, present_queue_family)?;

    let queue_data = QueueData {
        graphics_queue_family,
        graphics_queue,
        _present_queue_family: present_queue_family,
        present_queue,
    };

    Ok((device, queue_data))
}

fn get_queues(
    device: &ash::Device,
    graphics_queue_family: u32,
    present_queue_family: u32,
) -> Result<(vk::Queue, vk::Queue)> {
    //TODO: Make More flexible
    let graphics_queue = unsafe { device.get_device_queue(graphics_queue_family, 0) };
    let present_queue = unsafe { device.get_device_queue(present_queue_family, 1) };

    Ok((graphics_queue, present_queue))
}

fn get_queue_families(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    surface_loader: &Surface,
    surface: vk::SurfaceKHR,
) -> Result<(u32, u32)> {
    let qfps = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    let graphics_queue_family = get_graphics_queue_family(&qfps)?;
    let present_queue_family =
        get_present_queue_family(&qfps, physical_device, &surface_loader, surface)?;

    info!(
        "Graphics Queue Family ID: {}, Present Queue Family ID: {}",
        graphics_queue_family, present_queue_family
    );

    Ok((graphics_queue_family, present_queue_family))
}

fn get_graphics_queue_family(qfps: &[vk::QueueFamilyProperties]) -> Result<u32> {
    let graphics_queue_family = qfps.iter().enumerate().find_map(|(index, info)| {
        let supports_graphics = info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
        if supports_graphics {
            Some(index)
        } else {
            None
        }
    });

    if graphics_queue_family.is_none() {
        bail!("Unable to find queue family with Graphics Support");
    }

    Ok(graphics_queue_family.unwrap() as u32)
}

fn get_present_queue_family(
    qfps: &[vk::QueueFamilyProperties],
    physical_device: vk::PhysicalDevice,
    surface_loader: &Surface,
    surface: vk::SurfaceKHR,
) -> Result<u32> {
    let present_queue_family = qfps.iter().enumerate().find_map(|(index, _)| {
        let supports_surface = unsafe {
            surface_loader.get_physical_device_surface_support(
                physical_device,
                index as u32,
                surface,
            )
        };
        if supports_surface {
            Some(index)
        } else {
            None
        }
    });

    if present_queue_family.is_none() {
        bail!("Unable to find queue family with Present Support");
    }

    Ok(present_queue_family.unwrap() as u32)
}

pub(crate) fn create_descriptor_pool(
    num_sets: u32,
    device: &ash::Device,
) -> Result<vk::DescriptorPool> {
    let pool_size = vk::DescriptorPoolSize::builder()
        .descriptor_count(num_sets)
        .ty(vk::DescriptorType::UNIFORM_BUFFER)
        .build();

    let pool_sizes = [pool_size];

    let pool_info = vk::DescriptorPoolCreateInfo::builder()
        .pool_sizes(&pool_sizes)
        .max_sets(num_sets);

    let descriptor_pool;
    unsafe {
        descriptor_pool = device.create_descriptor_pool(&pool_info, None)?;
    }
    Ok(descriptor_pool)
}

#[allow(dead_code)]
pub(crate) fn log_physical_device_and_queue_info(instance: &ash::Instance) -> Result<()> {
    unsafe {
        let physical_devices = instance.enumerate_physical_devices()?;
        for (index, physical_device) in physical_devices.into_iter().enumerate() {
            let props = instance.get_physical_device_properties(physical_device);
            info!("Physical Device index: {}", index);
            info!("Physical Device Properties: {:#?}", props);

            let qfps = instance.get_physical_device_queue_family_properties(physical_device);
            for (i, qfp) in qfps.iter().enumerate() {
                info!("Queue Family Index {}", i);
                info!("Queue Flags: {}", qfp.queue_flags);
                info!("Queue Count: {}", qfp.queue_count);
            }
        }
    }

    Ok(())
}

// pub(crate) fn pick_physical_device(
//     instance: &ash::Instance,
//     surface_stuff: &SurfaceStuff,
//     required_device_extensions: &DeviceExtensions,
// ) -> Result<vk::PhysicalDevice> {
//     let physical_devices = unsafe { instance.enumerate_physical_devices()? };

//     let result = physical_devices.iter().find(|&&physical_device| {
//         let swapchain_support = query_swapchain_support(physical_device, surface_stuff);
//         let is_suitable = is_physical_device_suitable(
//             instance,
//             physical_device,
//             surface_stuff,
//             &swapchain_support,
//             required_device_extensions,
//         );

//         is_suitable
//     });

//     let physical_device = match result {
//         Some(p_physical_device) => *p_physical_device,
//         None => bail!("Failed to find a suitable GPU!"),
//     };

//     Ok(physical_device)
// }

// pub(crate) fn is_physical_device_suitable(
//     instance: &ash::Instance,
//     physical_device: vk::PhysicalDevice,
//     surface_stuff: &SurfaceStuff,
//     swapchain_support: &SwapChainSupportDetail,
//     required_device_extensions: &DeviceExtensions,
// ) -> bool {
//     let device_features = unsafe { instance.get_physical_device_features(physical_device) };

//     let indices = find_queue_family(instance, physical_device, surface_stuff);

//     let is_queue_family_supported = indices.is_complete();
//     let is_device_extension_supported =
//         check_device_extension_support(instance, physical_device, required_device_extensions);
//     let is_swapchain_supported =
//         !swapchain_support.formats.is_empty() && !swapchain_support.present_modes.is_empty();
//     let is_support_sampler_anisotropy = device_features.sampler_anisotropy == 1;

//     return is_queue_family_supported
//         && is_device_extension_supported
//         && is_swapchain_supported
//         && is_support_sampler_anisotropy;
// }

// pub(crate) fn query_swapchain_support(
//     physical_device: vk::PhysicalDevice,
//     surface_stuff: &SurfaceStuff,
// ) -> SwapChainSupportDetail {
//     unsafe {
//         let capabilities = surface_stuff
//             .surface_loader
//             .get_physical_device_surface_capabilities_khr(physical_device, surface_stuff.surface)
//             .expect("Failed to query for surface capabilities.");
//         let formats = surface_stuff
//             .surface_loader
//             .get_physical_device_surface_formats_khr(physical_device, surface_stuff.surface)
//             .expect("Failed to query for surface formats.");
//         let present_modes = surface_stuff
//             .surface_loader
//             .get_physical_device_surface_present_modes_khr(physical_device, surface_stuff.surface)
//             .expect("Failed to query for surface present mode.");

//         SwapChainSupportDetail {
//             capabilities,
//             formats,
//             present_modes,
//         }
//     }
// }

// pub(crate) fn find_queue_family(
//     instance: &ash::Instance,
//     physical_device: vk::PhysicalDevice,
//     surface_stuff: &SurfaceStuff,
// ) -> QueueFamilyIndices {
//     let queue_families =
//         unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

//     let mut queue_family_indices = QueueFamilyIndices::new();

//     let mut index = 0;
//     for queue_family in queue_families.iter() {
//         if queue_family.queue_count > 0
//             && queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
//         {
//             queue_family_indices.graphics_family = index;
//         }

//         let is_present_support = unsafe {
//             surface_stuff
//                 .surface_loader
//                 .get_physical_device_surface_support_khr(
//                     physical_device,
//                     index as u32,
//                     surface_stuff.surface,
//                 )
//         };
//         if queue_family.queue_count > 0 && is_present_support {
//             queue_family_indices.present_family = index;
//         }

//         if queue_family_indices.is_complete() {
//             break;
//         }

//         index += 1;
//     }

//     queue_family_indices
// }

// pub(crate) fn check_device_extension_support(
//     instance: &ash::Instance,
//     physical_device: vk::PhysicalDevice,
//     device_extensions: &DeviceExtensions,
// ) -> Result<bool> {
//     let available_extensions =
//         unsafe { instance.enumerate_device_extension_properties(physical_device)? };

//     let mut available_extension_names = vec![];

//     for extension in available_extensions.iter() {
//         let extension_name = vk_to_string(&extension.extension_name)?;

//         available_extension_names.push(extension_name);
//     }

//     use std::collections::HashSet;
//     let mut required_extensions = HashSet::new();
//     for extension in device_extensions.iter() {
//         required_extensions.insert(extension.to_string());
//     }

//     for extension_name in available_extension_names.iter() {
//         required_extensions.remove(extension_name);
//     }

//     Ok(required_extensions.is_empty())
// }

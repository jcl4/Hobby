use crate::AppInfo;
use crate::Result;
use crate::WindowSettings;

use std::collections::HashSet;
use std::iter::FromIterator;
use std::sync::Arc;

use failure::bail;
use log::{info, warn};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::instance::{
    layers_list, ApplicationInfo, Instance, InstanceExtensions, PhysicalDevice, Version,
};
use vulkano::swapchain::Surface;
use vulkano_win;
use vulkano_win::VkSurfaceBuild;
use winit::dpi::PhysicalSize;
use winit::{EventsLoop, Window, WindowBuilder};

const VALIDATION_LAYERS: &[&str] = &["VK_LAYER_LUNARG_standard_validation"];

//TODO: check if release build and set to false
const ENABLE_VALIDATION_LAYERS: bool = true;

pub struct QueueFamilyIndices {
    pub graphics_family: i32,
    pub present_family: i32,
}
impl QueueFamilyIndices {
    pub fn new() -> Self {
        Self {
            graphics_family: -1,
            present_family: -1,
        }
    }

    fn is_complete(&self) -> bool {
        self.graphics_family >= 0 && self.present_family >= 0
    }
}

pub fn create_logical_device(
    instance: &Arc<Instance>,
    surface: &Arc<Surface<Window>>,
    physical_device_index: usize,
) -> Result<(Arc<Device>, Arc<Queue>, Arc<Queue>)> {
    let physical_device = PhysicalDevice::from_index(&instance, physical_device_index).unwrap();
    let indices = find_queue_families(&surface, &physical_device);

    let families = [indices.graphics_family, indices.present_family];
    let unique_queue_families: HashSet<&i32> = HashSet::from_iter(families.iter());

    let queue_priority = 1.0;
    let queue_families = unique_queue_families.iter().map(|i| {
        (
            physical_device.queue_families().nth(**i as usize).unwrap(),
            queue_priority,
        )
    });

    // NOTE: the tutorial recommends passing the validation layers as well
    // for legacy reasons (if ENABLE_VALIDATION_LAYERS is true). Vulkano handles that
    // for us internally.

    let (device, mut queues) = Device::new(
        physical_device,
        &Features::none(),
        &device_extensions(),
        queue_families,
    )?;

    let graphics_queue = queues.next().unwrap();
    let present_queue = queues.next().unwrap_or_else(|| graphics_queue.clone());

    info!("Device Created");
    info!(
        "\tGraphcis Queue Family ID: {}, Index: {}",
        graphics_queue.family().id(),
        graphics_queue.id_within_family()
    );

    info!(
        "\tPresent Queue Family ID: {}, Index: {}",
        present_queue.family().id(),
        present_queue.id_within_family()
    );

    Ok((device, graphics_queue, present_queue))
}

pub fn create_instance(app_info: &AppInfo) -> Result<Arc<Instance>> {
    if ENABLE_VALIDATION_LAYERS && check_validation_layer_support() {
        bail!("Validation layers requested, but not available!")
    }

    let supported_extensions = InstanceExtensions::supported_by_core()?;
    info!("Supported extensions: {:#?}", supported_extensions);

    let engine_name = env!("CARGO_PKG_NAME");

    let engine_version = Version {
        major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
    };

    let app_info = ApplicationInfo {
        application_name: Some(app_info.app_name.clone().into()),
        application_version: Some(Version {
            major: app_info.app_version.0,
            minor: app_info.app_version.1,
            patch: app_info.app_version.2,
        }),
        engine_name: Some(engine_name.into()),
        engine_version: Some(engine_version),
    };

    let required_extensions = get_required_extensions();

    let instance;

    if ENABLE_VALIDATION_LAYERS && check_validation_layer_support() {
        instance = Instance::new(
            Some(&app_info),
            &required_extensions,
            VALIDATION_LAYERS.iter().map(|s| *s),
        )?;
    } else {
        instance = Instance::new(Some(&app_info), &required_extensions, None)?;
    }

    info!("Instance Created");
    Ok(instance)
}

fn check_validation_layer_support() -> bool {
    let layers: Vec<_> = layers_list()
        .unwrap()
        .map(|l| l.name().to_owned())
        .collect();
    VALIDATION_LAYERS
        .iter()
        .all(|layer_name| layers.contains(&layer_name.to_string()))
}

fn get_required_extensions() -> InstanceExtensions {
    let mut extensions = vulkano_win::required_extensions();
    if ENABLE_VALIDATION_LAYERS {
        // TODO!: this should be ext_debug_utils (_report is deprecated), but that doesn't exist yet in vulkano
        extensions.ext_debug_report = true;
    }

    extensions
}

pub fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
    if !ENABLE_VALIDATION_LAYERS {
        return None;
    }
    info!("Debug Callback Enabled");

    let msg_types = MessageTypes {
        error: true,
        warning: true,
        performance_warning: true,
        information: true,
        debug: true,
    };
    DebugCallback::new(&instance, msg_types, |msg| {
        if msg.ty.information {
            info!("Validation Layer: {:?}", msg.layer_prefix);
            info!("\tdescription: {:?}", msg.description);
        } else {
            warn!("Validation Layer: {:?}", msg.layer_prefix);
            warn!("\tdescription: {:?}", msg.description);
        }
    })
    .ok()
}

pub fn pick_physical_device(
    instance: &Arc<Instance>,
    surface: &Arc<Surface<Window>>,
) -> Result<usize> {
    let index = PhysicalDevice::enumerate(&instance)
        .position(|device| is_device_suitable(surface, &device));

    match index {
        Some(index) => {
            let mut devices = PhysicalDevice::enumerate(&instance);
            let device = devices.nth(index).unwrap();
            info!("Physical Device Chosen: {}", device.name());
            Ok(index)
        }
        None => bail!("Unable to find suitable physical device"),
    }
}

fn is_device_suitable(surface: &Arc<Surface<Window>>, device: &PhysicalDevice) -> bool {
    let indices = find_queue_families(surface, device);
    let extensions_supported = check_device_extension_support(device);

    let swap_chain_adequate = if extensions_supported {
        let capabilities = surface
            .capabilities(*device)
            .expect("failed to get surface capabilities");
        !capabilities.supported_formats.is_empty()
            && capabilities.present_modes.iter().next().is_some()
    } else {
        false
    };

    indices.is_complete() && extensions_supported && swap_chain_adequate
}

fn check_device_extension_support(device: &PhysicalDevice) -> bool {
    let available_extensions = DeviceExtensions::supported_by_device(*device);
    let device_extensions = device_extensions();
    available_extensions.intersection(&device_extensions) == device_extensions
}

pub fn find_queue_families(
    surface: &Arc<Surface<Window>>,
    device: &PhysicalDevice,
) -> QueueFamilyIndices {
    let mut indices = QueueFamilyIndices::new();
    // TODO: replace index with id to simplify?
    for (i, queue_family) in device.queue_families().enumerate() {
        if queue_family.supports_graphics() {
            indices.graphics_family = i as i32;
        }

        if surface.is_supported(queue_family).unwrap() {
            indices.present_family = i as i32;
        }

        if indices.is_complete() {
            break;
        }
    }

    indices
}

fn device_extensions() -> DeviceExtensions {
    DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    }
}

pub fn create_surface(
    events_loop: &EventsLoop,
    window_settings: &WindowSettings,
    instance: &Arc<Instance>,
) -> Result<Arc<Surface<Window>>> {
    let monitor = events_loop.get_primary_monitor();
    let dpi = monitor.get_hidpi_factor();

    let physical_size = PhysicalSize::new(window_settings.width, window_settings.height);
    let logical_size = physical_size.to_logical(dpi);

    let surface = WindowBuilder::new()
        .with_dimensions(logical_size)
        .with_title(window_settings.title.clone())
        .build_vk_surface(events_loop, instance.clone())?;

    let size = surface.window().get_inner_size().unwrap();

    info!("Built Window");
    info!("\tWindow Size: {:?}", size.to_physical(dpi));

    Ok(surface)
}

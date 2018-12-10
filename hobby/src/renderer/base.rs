use crate::AppInfo;
use crate::Result;

use std::sync::Arc;

use failure::bail;
use log::{info, warn};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::instance::{
    layers_list, ApplicationInfo, Instance, InstanceExtensions, PhysicalDevice, Version,
};
use vulkano::swapchain::Surface;
use vulkano_win;
use winit::Window;

const VALIDATION_LAYERS: &[&str] = &["VK_LAYER_LUNARG_standard_validation"];

//TODO: check if release build and set to false
const ENABLE_VALIDATION_LAYERS: bool = true;

struct QueueFamilyIndices {
    graphics_family: i32,
    present_family: i32,
}
impl QueueFamilyIndices {
    fn new() -> Self {
        Self {
            graphics_family: -1,
            present_family: -1,
        }
    }

    fn is_complete(&self) -> bool {
        self.graphics_family >= 0 && self.present_family >= 0
    }
}



pub(crate) fn create_instance(app_info: &AppInfo) -> Result<Arc<Instance>> {
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

pub(crate) fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
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

pub(crate) fn pick_physical_device(
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

fn find_queue_families(
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


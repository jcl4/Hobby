use ash::{
    extensions::{
        ext::DebugUtils,
        khr::{Surface, XlibSurface},
    },
    version::{EntryV1_0, InstanceV1_0, DeviceV1_0},
    vk,
    Entry, Instance, Device,
};

use log::{debug, error, info, warn};
use std::{ffi::CStr, os::raw::c_void};

pub const REQUIRED_LAYERS: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation"];

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> u32 {
    let callback_data = *callback_data;
    let p_message_id_name = callback_data.p_message_id_name;
    let message_id_num = callback_data.message_id_number;
    let p_message = callback_data.p_message;

    let validation_message = format!(
        "Validation Message: Sevarity: {:?}; Type: {:?}; VUID: {:?}; ID#: {:?}; Message: {:?}",
        message_severity,
        message_type,
        CStr::from_ptr(p_message_id_name),
        message_id_num,
        CStr::from_ptr(p_message)
    );

    if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE {
        debug!("{:?}", validation_message);
    } else if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        info!("{:?}", validation_message);
    } else if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("{:?}", validation_message);
    } else {
        error!("{:?}", validation_message);
    }
    vk::FALSE
}

pub fn check_validation_layer_support(entry: &Entry) {
    for required in REQUIRED_LAYERS.iter() {
        let found = entry
            .enumerate_instance_layer_properties()
            .unwrap()
            .iter()
            .any(|layer| {
                let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()) };
                let name = name.to_str().expect("Failed to get layer name pointer");
                required == &name
            });

        if !found {
            panic!("Validation layer not supported: {}", required);
        }
    }
}

pub fn setup_debug_messenger(
    entry: &Entry,
    instance: &Instance,
) -> (DebugUtils, vk::DebugUtilsMessengerEXT) {
    let create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .pfn_user_callback(Some(vulkan_debug_callback));

    let debug_utils = DebugUtils::new(entry, instance);
    let debug_utils_messenger = unsafe {
        debug_utils
            .create_debug_utils_messenger(&create_info, None)
            .expect("Unable to create debug utils messenger")
    };

    (debug_utils, debug_utils_messenger)
}

pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

pub fn pick_physical_device(instance: &Instance) -> vk::PhysicalDevice {
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    let device = devices
        .into_iter()
        .find(|device| is_device_suitable(instance, *device))
        .expect("No suitable physical device.");

    let props = unsafe { instance.get_physical_device_properties(device) };
    info!("Selected physical device: {:?}", unsafe {
        CStr::from_ptr(props.device_name.as_ptr())
    });
    device
}

fn is_device_suitable(instance: &Instance, device: vk::PhysicalDevice) -> bool {
    find_queue_families(instance, device).is_some()
}

fn find_queue_families(instance: &Instance, device: vk::PhysicalDevice) -> Option<u32> {
    let props = unsafe { instance.get_physical_device_queue_family_properties(device) };
    props
        .iter()
        .enumerate()
        .find(|(_, family)| {
            family.queue_count > 0 && family.queue_flags.contains(vk::QueueFlags::GRAPHICS)
        })
        .map(|(index, _)| index as _)
}

pub fn create_logical_device_w_graphics_queue(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> (Device, vk::Queue) {
    let queue_family_index =
        find_queue_families(instance, physical_device).expect("Unable to find appropriate Queue Families");
    let queue_priorities = [1.0];

    let queue_create_infos = [vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_index)
        .queue_priorities(&queue_priorities)
        .build()];

    debug!("Queue Create Infos: {:?}", queue_create_infos);

    let device_features = vk::PhysicalDeviceFeatures::builder();

    debug!("Device Features: {:#?}", *device_features);

    let mut device_create_info_builder = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_features(&device_features);

    let device_create_info = device_create_info_builder.build();

    let device = unsafe {
        instance
            .create_device(physical_device, &device_create_info, None)
            .expect("Failed to create logical device.")
    };
    let graphics_queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    (device, graphics_queue)
}

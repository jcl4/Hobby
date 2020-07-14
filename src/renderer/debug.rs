use ash::{
    extensions::{
        ext::DebugUtils,
    },
    version::{EntryV1_0},
    vk,
    Entry, Instance,
};

use std::{ffi::CStr, os::raw::c_void};

use log::{debug, info, warn, error};


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

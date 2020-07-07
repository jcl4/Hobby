mod util;

use log::{debug, error, info, warn};
use std::{
    ffi::{CStr, CString},
    os::raw::c_void,
};

use ash::{
    version::{EntryV1_0, InstanceV1_0},
    vk, Entry, Instance,
    extensions::ext::DebugUtils,
};

use crate::config::Config;
use crate::Result;

const REQUIRED_LAYERS: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation"];

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> u32 {
    let callback_data = unsafe { *callback_data };
    let p_message_id_name = callback_data.p_message_id_name;
    let message_id_num = callback_data.message_id_number;
    let p_message = callback_data.p_message;

    let validation_message = format!(
        "Validation Message: Sevarity: {:?}; Type: {:?}; VUID: {:?}; ID#: {:?}; Message: {:?}",
        message_severity, message_type, CStr::from_ptr(p_message_id_name), message_id_num, CStr::from_ptr(p_message)
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

pub(crate) struct Renderer {
    _entry: Entry,
    instance: Instance,
    debug_utils: (DebugUtils, vk::DebugUtilsMessengerEXT),
}

impl Renderer {
    pub(crate) fn new(config: Config) -> Result<Renderer> {
        let entry = Entry::new()?;

        check_validation_layer_support(&entry);

        let app_name = CString::new(config.application.name)?;
        let app_version = vk::make_version(
            config.application.version[0],
            config.application.version[1],
            config.application.version[2],
        );

        let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>()?;
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>()?;
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>()?;
        let engine_version = vk::make_version(major, minor, patch);
        let engine_name = CString::new("Hobby")?;

        let api_version = vk::make_version(1, 2, 0);

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(app_version)
            .engine_name(&engine_name)
            .engine_version(engine_version)
            .api_version(api_version);

        let required_extensions = util::required_extension_names();

        let layer_names = REQUIRED_LAYERS
            .iter()
            .map(|name| CString::new(*name).expect("Failed to build CString"))
            .collect::<Vec<_>>();
        let layer_names_ptrs = layer_names
            .iter()
            .map(|name| name.as_ptr())
            .collect::<Vec<_>>();

        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&required_extensions)
            .enabled_layer_names(&layer_names_ptrs);

        let instance = unsafe { entry.create_instance(&instance_create_info, None)? };
        let debug_utils = setup_debug_messenger(&entry, &instance)?;

        info!("Vulkan Instance Created");

        Ok(Renderer {
            _entry: entry,
            instance,
            debug_utils,
        })
    }

    pub(crate) fn cleanup(&self) {
        info!("Renderer Cleanup");
        unsafe {
            self.debug_utils.0.destroy_debug_utils_messenger(self.debug_utils.1, None);
            self.instance.destroy_instance(None);
        }
    }
}

fn check_validation_layer_support(entry: &Entry) {
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

fn setup_debug_messenger(
    entry: &Entry,
    instance: &Instance,
) -> Result<(DebugUtils, vk::DebugUtilsMessengerEXT)> {
    let create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .pfn_user_callback(Some(vulkan_debug_callback));

    let debug_utils = DebugUtils::new(entry, instance);
    let debug_utils_messenger = unsafe {debug_utils.create_debug_utils_messenger(&create_info, None)?};

    Ok((debug_utils, debug_utils_messenger))
}

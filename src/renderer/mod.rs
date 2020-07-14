mod util;

use log::{debug, info};
use std::ffi::CString;

use ash::{
    extensions::ext::DebugUtils,
    version::{EntryV1_0, InstanceV1_0},
    vk,
    Entry, Instance, Device,
};

use crate::config::Config;

pub(crate) struct Renderer {
    _entry: Entry,
    instance: Instance,
    debug_utils: (DebugUtils, vk::DebugUtilsMessengerEXT),
    _physical_device: vk::PhysicalDevice,
    device: Device,
    graphics_queue: vk::Queue,
}

impl Renderer {
    pub(crate) fn new(config: Config) -> Renderer {
        let entry = Entry::new().expect("Unable to create Ash Entry");

        util::check_validation_layer_support(&entry);

        let app_name = CString::new(config.application.name)
            .expect("Unable to convert application name to CString");
        let app_version = vk::make_version(
            config.application.version[0],
            config.application.version[1],
            config.application.version[2],
        );

        let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();
        let engine_version = vk::make_version(major, minor, patch);
        let engine_name = CString::new("Hobby").unwrap();

        let api_version = vk::make_version(1, 2, 0);

        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(app_version)
            .engine_name(&engine_name)
            .engine_version(engine_version)
            .api_version(api_version);

        let required_extensions = util::required_extension_names();

        let layer_names = util::REQUIRED_LAYERS
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

        let instance = unsafe { entry.create_instance(&instance_create_info, None).expect("Unable to create instance") };
        debug!("Vulkan Instance Created");

        let debug_utils = util::setup_debug_messenger(&entry, &instance);
        debug!("Vulkan Debug Utils Created");

        let physical_device = util::pick_physical_device(&instance);
        debug!("Vulkan Physical Device Created");

        let (device, graphics_queue) =
            util::create_logical_device_w_graphics_queue(&instance, physical_device);

        Renderer {
            _entry: entry,
            instance,
            debug_utils,
            _physical_device: physical_device,
            device,
            graphics_queue,
        }
    }

    pub(crate) fn cleanup(&self) {
        info!("Renderer Cleanup");
        unsafe {
            self.debug_utils
                .0
                .destroy_debug_utils_messenger(self.debug_utils.1, None);
            self.instance.destroy_instance(None);
        }
    }
}

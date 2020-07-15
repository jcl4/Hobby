use ash::{
    extensions::{
        ext::DebugUtils,
        khr::{Surface, XlibSurface, Swapchain},
    },
    version::{EntryV1_0, InstanceV1_0},
    vk, Entry, Instance,
};
use winit::window::Window;

use std::ffi::{CStr, CString};

use super::debug;
use crate::config::Config;


fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

pub fn required_device_extension_names() -> [&'static CStr; 1]{
    [Swapchain::name()]
}



pub struct QueueFamilyIndices{
    pub graphics: u32,
    pub present: u32,
}


pub struct Context {
    pub entry: Entry,
    pub instance: Instance,
    pub physical_device: vk::PhysicalDevice,
    pub surface: Surface,
    pub surface_khr: vk::SurfaceKHR,
    pub queue_family_indices: QueueFamilyIndices,
}

impl Context {
    pub fn new(config: &Config, window: &Window) -> Context {
        let entry = Entry::new().expect("Unable to create Ash Entry");

        let app_name = CString::new(config.application.name.clone())
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

        let required_extensions = required_extension_names();

        let layer_names = debug::REQUIRED_LAYERS
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

        let instance = unsafe {
            entry
                .create_instance(&instance_create_info, None)
                .expect("Unable to create instance")
        };
        log::debug!("Vulkan Instance Created");

        let surface = Surface::new(&entry, &instance);
        let surface_khr = create_surface(&entry, &instance, window);
        log::debug!("Surface and SurfaceKHR Created");

        let physical_device = pick_physical_device(&instance, &surface, surface_khr);
        log::debug!("Vulkan Physical Device Created");

        let(graphics, present) = find_queue_families(&instance, &surface, surface_khr, physical_device);

        let queue_family_indices = QueueFamilyIndices{
            graphics: graphics.unwrap(),
            present: present.unwrap(),
        };

        Context {
            entry,
            instance,
            physical_device,
            surface,
            surface_khr,
            queue_family_indices,
        }
    }

    pub fn cleanup(&self) {
        unsafe {
            self.surface.destroy_surface(self.surface_khr, None);
            self.instance.destroy_instance(None);
        }
    }
}

fn pick_physical_device(instance: &Instance, surface: &Surface,
    surface_khr: vk::SurfaceKHR,) -> vk::PhysicalDevice {
    let devices = unsafe { instance.enumerate_physical_devices().unwrap() };
    let device = devices
        .into_iter()
        .find(|device| is_device_suitable(instance, surface, surface_khr, *device))
        .expect("No suitable physical device.");

    let props = unsafe { instance.get_physical_device_properties(device) };
    log::info!("Selected physical device: {:?}", unsafe {
        CStr::from_ptr(props.device_name.as_ptr())
    });
    device
}

fn is_device_suitable(
    instance: &Instance,
    surface: &Surface,
    surface_khr: vk::SurfaceKHR,
    device: vk::PhysicalDevice,
) -> bool {
    let (graphics, present) = find_queue_families(instance, surface, surface_khr, device);

    let extention_support = check_device_extension_support(instance, device);

    let is_swapchain_adequate = {
        let formats = unsafe {
            surface
                .get_physical_device_surface_formats(device, surface_khr)
                .expect("Unable to get surface formats")
        };

        let present_modes = unsafe {
            surface
                .get_physical_device_surface_present_modes(device, surface_khr)
                .expect("Unable to get present modes")
        };
        !formats.is_empty() && !present_modes.is_empty()
    };

    graphics.is_some() && present.is_some() && extention_support && is_swapchain_adequate
}

fn check_device_extension_support(instance: &Instance, device: vk::PhysicalDevice) -> bool {
    let required_extentions = required_device_extension_names();

    let extension_props = unsafe {
        instance
            .enumerate_device_extension_properties(device)
            .unwrap()
    };

    for required in required_extentions.iter() {
        let found = extension_props.iter().any(|ext| {
            let name = unsafe { CStr::from_ptr(ext.extension_name.as_ptr()) };
            required == &name
        });

        if !found {
            return false;
        }
    }

    true
}


pub fn find_queue_families(
    instance: &Instance,
    surface: &Surface,
    surface_khr: vk::SurfaceKHR,
    device: vk::PhysicalDevice,
) -> (Option<u32>, Option<u32>) {
    let mut graphics = None;
    let mut present = None;

    let props = unsafe { instance.get_physical_device_queue_family_properties(device) };
    for (index, family) in props.iter().filter(|f| f.queue_count > 0).enumerate() {
        let index = index as u32;

        if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) && graphics.is_none() {
            graphics = Some(index);
        }

        let present_support = unsafe {
            surface
                .get_physical_device_surface_support(device, index, surface_khr)
                .expect("Physical device does not support surface")
        };
        if present_support && present.is_none() {
            present = Some(index);
        }

        if graphics.is_some() && present.is_some() {
            break;
        }
    }

    (graphics, present)
}

pub fn create_surface(entry: &Entry, instance: &Instance, window: &Window) -> vk::SurfaceKHR {
    use winit::platform::unix::WindowExtUnix;
    let x11_display = window.xlib_display().expect("Display does not use X11");
    let x11_window = window.xlib_window().expect("Window does not use X11");
    let x11_create_info = vk::XlibSurfaceCreateInfoKHR::builder()
        .window(x11_window)
        .dpy(x11_display as *mut vk::Display);

    let xlib_surface_loader = XlibSurface::new(entry, instance);
    unsafe {
        xlib_surface_loader
            .create_xlib_surface(&x11_create_info, None)
            .expect("Unable to create X11 Surface")
    }
    
}

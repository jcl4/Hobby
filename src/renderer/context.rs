use super::swapchain::SwapchainSupportDetails;
use ash::{
    extensions::khr,
    version::{DeviceV1_0, EntryV1_0, InstanceV1_0},
    vk,
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};
use winit::window::Window;

pub struct Context {
    _entry: ash::Entry,
    instance: ash::Instance,
    surface: khr::Surface,
    surface_khr: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    queue_families_indices: QueueFamiliesIndices,
}

impl Context {
    pub fn new(window: &Window, app_name: &str, app_version: u32) -> Self {
        let entry = ash::Entry::new().expect("Failed to create entry");
        let instance = create_instance(&entry, app_name, app_version);

        let surface = khr::Surface::new(&entry, &instance);
        let surface_khr = unsafe { create_surface(&entry, &instance, &window) };

        let (physical_device, queue_families_indices) =
            pick_physical_device(&instance, &surface, surface_khr);

        let device = create_logical_device_with_graphics_queue(
            &instance,
            physical_device,
            queue_families_indices,
        );

        Context {
            _entry: entry,
            instance,
            surface,
            surface_khr,
            physical_device,
            device,
            queue_families_indices,
        }
    }

    pub fn instance(&self) -> &ash::Instance {
        &self.instance
    }

    pub fn surface(&self) -> &khr::Surface {
        &self.surface
    }

    pub fn surface_khr(&self) -> vk::SurfaceKHR {
        self.surface_khr
    }

    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }

    pub fn device(&self) -> &ash::Device {
        &self.device
    }

    pub fn queue_families_indices(&self) -> QueueFamiliesIndices {
        self.queue_families_indices
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.surface.destroy_surface(self.surface_khr, None);
            self.instance.destroy_instance(None);
        }
    }
}

#[derive(Clone, Copy)]
pub struct QueueFamiliesIndices {
    graphics_index: u32,
    present_index: u32,
}

impl QueueFamiliesIndices {
    pub fn graphics_index(&self) -> u32 {
        self.graphics_index
    }

    pub fn present_index(&self) -> u32 {
        self.present_index
    }
}

fn create_instance(entry: &ash::Entry, app_name: &str, app_version: u32) -> ash::Instance {
    // let app_name = CString::new("Vulkan Application").unwrap();
    let engine_name = CString::new("Hobby Engine").unwrap();
    let app_name = CString::new(app_name).unwrap();
    let app_info = vk::ApplicationInfo::builder()
        .application_name(app_name.as_c_str())
        .application_version(app_version)
        .engine_name(engine_name.as_c_str())
        .engine_version(ash::vk_make_version!(0, 1, 0))
        .api_version(ash::vk_make_version!(1, 0, 0))
        .build();

    let extension_names = required_extension_names();

    let instance_create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names);

    unsafe { entry.create_instance(&instance_create_info, None).unwrap() }
}

fn create_logical_device_with_graphics_queue(
    instance: &ash::Instance,
    device: vk::PhysicalDevice,
    queue_families_indices: QueueFamiliesIndices,
) -> ash::Device {
    let graphics_family_index = queue_families_indices.graphics_index;
    let present_family_index = queue_families_indices.present_index;
    let queue_priorities = [1.0f32];

    let queue_create_infos = {
        // Vulkan specs does not allow passing an array containing duplicated family indices.
        // And since the family for graphics and presentation could be the same we need to
        // deduplicate it.
        let mut indices = vec![graphics_family_index, present_family_index];
        indices.dedup();

        // Now we build an array of `DeviceQueueCreateInfo`.
        // One for each different family index.
        indices
            .iter()
            .map(|index| {
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(*index)
                    .queue_priorities(&queue_priorities)
                    .build()
            })
            .collect::<Vec<_>>()
    };

    let device_extensions = get_required_device_extensions();
    let device_extensions_ptrs = device_extensions
        .iter()
        .map(|ext| ext.as_ptr())
        .collect::<Vec<_>>();

    let device_features = vk::PhysicalDeviceFeatures::builder()
        .sampler_anisotropy(true)
        .build();

    let device_create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(&device_extensions_ptrs)
        .enabled_features(&device_features)
        .build();

    // Build device and queues
    unsafe {
        instance
            .create_device(device, &device_create_info, None)
            .expect("Failed to create logical device.")
    }
}

fn required_extension_names() -> Vec<*const c_char> {
    use ash::extensions::khr::XlibSurface;
    vec![khr::Surface::name().as_ptr(), XlibSurface::name().as_ptr()]
}

unsafe fn create_surface<E: EntryV1_0, I: InstanceV1_0>(
    entry: &E,
    instance: &I,
    window: &Window,
) -> vk::SurfaceKHR {
    use ash::extensions::khr::XlibSurface;
    use std::ffi::c_void;

    let result = match window.raw_window_handle() {
        RawWindowHandle::Xlib(handle) => {
            let create_info = vk::XlibSurfaceCreateInfoKHR::builder()
                .window(handle.window)
                .dpy(handle.display as *mut *const c_void);
            let surface_loader = XlibSurface::new(entry, instance);
            let surface = surface_loader
                .create_xlib_surface(&create_info, None)
                .expect("Surface creation error");
            Ok(surface)
        }
        _ => Err("Window handle not available"),
    };
    result.expect("Unable to create surface")
}

fn pick_physical_device(
    instance: &ash::Instance,
    surface: &khr::Surface,
    surface_khr: vk::SurfaceKHR,
) -> (vk::PhysicalDevice, QueueFamiliesIndices) {
    let devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Unable to enumerate physical devices")
    };

    let device = devices
        .into_iter()
        .find(|device| is_device_suitable(instance, surface, surface_khr, *device))
        .expect("No suitable physical device.");

    let props = unsafe { instance.get_physical_device_properties(device) };
    log::debug!("Selected physical device: {:#?}", props);
    unsafe {
        println!(
            "Device Selcted: {:?}",
            CStr::from_ptr(props.device_name.as_ptr())
        );
    }
    println!(
        "Vulkan API Version: Major: {:?}, Minor: {:?}, Patch: {:?}",
        ash::vk_version_major!(props.api_version),
        ash::vk_version_minor!(props.api_version),
        ash::vk_version_patch!(props.api_version)
    );

    let (graphics, present) = find_queue_families(instance, surface, surface_khr, device);
    let queue_families_indices = QueueFamiliesIndices {
        graphics_index: graphics.unwrap(),
        present_index: present.unwrap(),
    };

    (device, queue_families_indices)
}

fn is_device_suitable(
    instance: &ash::Instance,
    surface: &khr::Surface,
    surface_khr: vk::SurfaceKHR,
    device: vk::PhysicalDevice,
) -> bool {
    let props = unsafe { instance.get_physical_device_properties(device) };
    let device_type = props.device_type;
    let (graphics, present) = find_queue_families(instance, surface, surface_khr, device);
    let extention_support = check_device_extension_support(instance, device);
    let is_swapchain_adequate = {
        let details = SwapchainSupportDetails::new(device, surface, surface_khr);
        !details.formats.is_empty() && !details.present_modes.is_empty()
    };
    let features = unsafe { instance.get_physical_device_features(device) };

    graphics.is_some()
        && present.is_some()
        && extention_support
        && is_swapchain_adequate
        && features.sampler_anisotropy == vk::TRUE
        && device_type == vk::PhysicalDeviceType::DISCRETE_GPU
}

fn check_device_extension_support(instance: &ash::Instance, device: vk::PhysicalDevice) -> bool {
    let required_extentions = get_required_device_extensions();

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

fn get_required_device_extensions() -> [&'static CStr; 1] {
    [khr::Swapchain::name()]
}

fn find_queue_families(
    instance: &ash::Instance,
    surface: &khr::Surface,
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

        let present_support =
            unsafe { surface.get_physical_device_surface_support(device, index, surface_khr) };
        if present_support && present.is_none() {
            present = Some(index);
        }

        if graphics.is_some() && present.is_some() {
            break;
        }
    }

    (graphics, present)
}

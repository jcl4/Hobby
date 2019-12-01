/*
Renderer workflow thoughts
- Scene holds a vector of models and a collection of unique materials
- a Material enum is exposed to the user to select various materials
- actual materials are rendering pipelines
- Renderer holds GPU objects
- Object hold all its required buffers and a draw call
-
- Object builder builds the buffers and bind group
- to build the bind group the bind group layout will need to be created
- bind group layouts need the pipeline to be created
- build a piplenines struct with options for each pipeline
- get piple line checks for is some if none creates pipeline
*/
use ash::{
    extensions::{
        ext::DebugReport,
        khr::{Surface, Swapchain},
    },
    version::{EntryV1_0, InstanceV1_0},
    vk,
};
use log::info;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};
use winit::window::Window;

use crate::{
    core::MaterialType,
    scene::{Mesh, ObjectBufferGroup, Scene},
};

mod context;
mod debug;

#[cfg(debug_assertions)]
pub const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = false;

#[derive(Clone, Copy)]
struct QueueFamiliesIndices {
    graphics_index: u32,
    present_index: u32,
}

/// The renderer is responsible for displaying a scene on screan
///
/// It holds all required GPU resources
#[allow(dead_code)]
pub struct Renderer {}

impl Renderer {
    pub fn new(window: &Window, app_name: &str, app_version: u32) -> Renderer {
        let entry = ash::Entry::new().expect("Failed to create entry");
        info!("Entry Created");
        let instance = create_instance(&entry, app_name, app_version);
        info!("Instance Created");

        let surface = Surface::new(&entry, &instance);
        let surface_khr = unsafe { create_surface(&entry, &instance, &window) };
        log::info!("Surface Created");

        let debug_report_callback = debug::setup_debug_messenger(&entry, &instance);

        let (physical_device, queue_families_indices) =
            pick_physical_device(&instance, &surface, surface_khr);

        let (device, graphics_queue, present_queue) = create_logical_device_with_graphics_queue(
            &instance,
            physical_device,
            queue_families_indices,
        );

        let vk_context = context::Context::new(
            entry,
            instance,
            debug_report_callback,
            surface,
            surface_khr,
            physical_device,
            device,
        );

        Renderer {}
    }

    pub fn get_object_buffer_group(
        &self,
        _mesh: &Mesh,
        _material: MaterialType,
    ) -> ObjectBufferGroup {
        unimplemented!()
    }

    pub fn draw_frame(&self, _scene: &Scene) {
        unimplemented!()
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {}
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

    let mut extension_names = required_extension_names();
    if ENABLE_VALIDATION_LAYERS {
        extension_names.push(DebugReport::name().as_ptr());
    }

    let (_layer_names, layer_names_ptrs) = debug::get_layer_names_and_pointers();

    let mut instance_create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&extension_names);
    if ENABLE_VALIDATION_LAYERS {
        debug::check_validation_layer_support(&entry);
        instance_create_info = instance_create_info.enabled_layer_names(&layer_names_ptrs);
    }

    unsafe { entry.create_instance(&instance_create_info, None).unwrap() }
}

fn required_extension_names() -> Vec<*const c_char> {
    use ash::extensions::khr::XlibSurface;
    vec![Surface::name().as_ptr(), XlibSurface::name().as_ptr()]
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
    surface: &Surface,
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
    log::debug!("Selected physical device: {:?}", unsafe {
        CStr::from_ptr(props.device_name.as_ptr())
    });

    let (graphics, present) = Self::find_queue_families(instance, surface, surface_khr, device);
    let queue_families_indices = QueueFamiliesIndices {
        graphics_index: graphics.unwrap(),
        present_index: present.unwrap(),
    };

    (device, queue_families_indices)
}

fn is_device_suitable(
    instance: &ash::Instance,
    surface: &Surface,
    surface_khr: vk::SurfaceKHR,
    device: vk::PhysicalDevice,
) -> bool {
    let (graphics, present) = find_queue_families(instance, surface, surface_khr, device);
    let extention_support = Self::check_device_extension_support(instance, device);
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
}

fn check_device_extension_support(instance: &ash::Instance, device: vk::PhysicalDevice) -> bool {
    let required_extentions = Self::get_required_device_extensions();

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
    [Swapchain::name()]
}

fn find_queue_families(
    instance: &ash::Instance,
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

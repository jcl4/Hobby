use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use voodoo as vd;
use winit::{dpi::PhysicalSize, EventsLoop, Window, WindowBuilder};

use AppInfo;
use Result;
use WindowSettings;

pub fn init_window(events_loop: &EventsLoop, window_settings: &WindowSettings) -> Window {
    let monitor = events_loop.get_primary_monitor();
    let dpi = monitor.get_hidpi_factor();

    let physical_size = PhysicalSize::new(window_settings.width, window_settings.height);
    let logical_size = physical_size.to_logical(dpi);

    let window = WindowBuilder::new()
        .with_dimensions(logical_size)
        .with_title(window_settings.title.clone())
        .build(events_loop)
        .expect("Unable to create window");

    let size = window.get_inner_size().expect("Unable to get window size");

    info!("Built Window");
    info!("\tWindow Size: {:?}", size.to_physical(dpi));

    window
}

pub fn create_instance(app_info: &AppInfo) -> Result<vd::Instance> {
    let engine_name = CString::new(env!("CARGO_PKG_NAME"))?;

    let engine_version = vd::Version::new(
        env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
    );

    let app_name = CString::new(app_info.app_name.clone())?;
    let app_version = vd::Version::new(
        app_info.app_version.0,
        app_info.app_version.1,
        app_info.app_version.2,
    );

    let app_info = vd::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(app_version)
        .engine_name(&engine_name)
        .engine_version(engine_version)
        .build();

    info!("Vulkan Application Info:");
    info!("\tEngine Name: {:?}", engine_name);
    info!("\tEngine Version: {}", engine_version);
    info!("\tApplication Name: {:?}", app_name);
    info!("\tApplication Version: {}", app_version);

    let loader = vd::Loader::new()?;
    let instance_extensions = loader.enumerate_instance_extension_properties()?;

    info!("Available instance extensions:");

    for ext in instance_extensions.iter() {
        let name = (&ext.extensionName) as *const c_char;
        unsafe {
            info!(
                "\t'{}' (version: {})",
                CStr::from_ptr(name).to_str().unwrap(),
                ext.specVersion
            );
        }
    }

    let instance = vd::Instance::builder()
        .application_info(&app_info)
        .enabled_extensions(&instance_extensions)
        .build(loader)?;

    info!("Instance Created");

    Ok(instance)
}

pub fn create_surface(instance: vd::Instance, window: &winit::Window) -> Result<vd::SurfaceKhr> {
    let surface = voodoo_winit::create_surface(instance, window)?;
    info!("Surface Created");
    Ok(surface)
}

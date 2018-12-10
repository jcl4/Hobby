use log::{info, warn};
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::{EventsLoop, Window, WindowBuilder};

use vulkano::instance::debug::DebugCallback;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;
use vulkano_win::VkSurfaceBuild;

use crate::renderer::base;
use crate::{HobbySettings, Result, WindowSettings};

pub struct Renderer {
    instance: Arc<Instance>,
    debug_callback: Option<DebugCallback>,

    surface: Arc<Surface<Window>>,
    physical_device_index: usize,
}

impl Renderer {
    pub fn new(hobby_settings: HobbySettings, events_loop: &EventsLoop) -> Result<Renderer> {
        let instance = base::create_instance(&hobby_settings.app_info)?;
        info!("Instance Created");
        let debug_callback = base::setup_debug_callback(&instance);
        let surface = create_surface(events_loop, &hobby_settings.window_settings, &instance)?;
        let physical_device_index = base::pick_physical_device(&instance, &surface)?;
       

        Ok(Renderer {
            instance,
            debug_callback,
            surface,
            physical_device_index,
        })
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

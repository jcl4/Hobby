use voodoo as vd;
use winit::{EventsLoop, Window};

use super::base;
use AppInfo;
use Result;
use WindowSettings;

pub struct Renderer {
    window: Window,
    instance: vd::Instance,
    surface: vd::SurfaceKhr,
}

impl Renderer {
    pub fn new(
        events_loop: &EventsLoop,
        window_settings: &WindowSettings,
        app_info: &AppInfo,
    ) -> Result<Renderer> {
        info!("Initializing Renderer");

        let window = base::init_window(events_loop, window_settings);
        let instance = base::create_instance(app_info)?;
        let surface = base::create_surface(instance.clone(), &window)?;

        Ok(Renderer {
            window,
            instance,
            surface,
        })
    }
}

pub struct QueueFamilyIndices {
    pub graphics_family: i32,
    pub present_family: i32,
}

impl QueueFamilyIndices {
    pub fn new() -> QueueFamilyIndices {
        QueueFamilyIndices {
            graphics_family: -1,
            present_family: -1,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_family >= 0 && self.present_family >= 0
    }
}

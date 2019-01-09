use std::path::PathBuf;
use std::time::Duration;

pub struct WindowSettings {
    pub width: f64,
    pub height: f64,
    pub title: String,
    pub v_sync: bool,
}

impl Default for WindowSettings {
    fn default() -> WindowSettings {
        WindowSettings {
            width: 1600.0,
            height: 900.0,
            title: "Hobby Window".into(),
            v_sync: false,
        }
    }
}

pub struct AppInfo {
    pub app_name: String,
    pub app_version: Version,
}

impl Default for AppInfo {
    fn default() -> AppInfo {
        AppInfo {
            app_name: "Default".into(),
            app_version: Version::new(0, 1, 0),
        }
    }
}

pub struct HobbySettings {
    pub window_settings: WindowSettings,
    pub app_info: AppInfo,
    pub display_update_duration: Duration,
    pub resource_path: PathBuf,
}

impl Default for HobbySettings {
    fn default() -> HobbySettings {
        let display_update_duration = Duration::from_millis(2000);
        let resource_path = PathBuf::new();
        HobbySettings {
            window_settings: WindowSettings::default(),
            app_info: AppInfo::default(),
            display_update_duration,
            resource_path,
        }
    }
}

pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    pub fn new(major: u16, minor: u16, patch: u16) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn into_vulkan_version(&self) -> u32 {
        assert!(self.major <= 0x3ff);
        assert!(self.minor <= 0x3ff);
        assert!(self.patch <= 0xfff);

        (self.major as u32) << 22 | (self.minor as u32) << 12 | (self.patch as u32)
    }
}

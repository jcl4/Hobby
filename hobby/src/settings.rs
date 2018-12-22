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
    pub app_version: (u16, u16, u16),
}

impl Default for AppInfo {
    fn default() -> AppInfo {
        AppInfo {
            app_name: "Default".into(),
            app_version: (0, 1, 0),
        }
    }
}

pub struct HobbySettings {
    pub window_settings: WindowSettings,
    pub app_info: AppInfo,
    pub display_update_duration: Duration,
}

impl Default for HobbySettings {
    fn default() -> HobbySettings {
        let display_update_duration = Duration::from_millis(2000);
        HobbySettings {
            window_settings: WindowSettings::default(),
            app_info: AppInfo::default(),
            display_update_duration,
        }
    }
}

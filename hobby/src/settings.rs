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
}

impl Default for HobbySettings {
    fn default() -> HobbySettings {
        HobbySettings {
            window_settings: WindowSettings::default(),
            app_info: AppInfo::default(),
        }
    }
}
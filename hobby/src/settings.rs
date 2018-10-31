pub struct WindowSettings {
    pub width: f64,
    pub height: f64,
    pub title: String,
}

impl Default for WindowSettings {
    fn default() -> WindowSettings {
        WindowSettings {
            width: 1600.0,
            height: 900.0,
            title: "Hobby Window".into(),
        }
    }
}

pub struct AppInfo {
    pub app_name: String,
    pub app_version: (u16, u16, u16),
}

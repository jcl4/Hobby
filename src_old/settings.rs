#[derive(Debug)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

impl WindowSettings {
    pub fn default() -> WindowSettings {
        WindowSettings {
            width: 1600,
            height: 900,
            title: String::from("Hobby Window"),
        }
    }
}

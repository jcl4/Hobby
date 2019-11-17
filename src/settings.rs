#[derive(Debug)]
pub struct GameSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
}

impl GameSettings {
    pub fn default() -> GameSettings {
        GameSettings {
            window_width: 1600,
            window_height: 900,
            window_title: String::from("Hobby Window"),
        }
    }
}

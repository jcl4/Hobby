use crate::{Scene};

#[derive(Debug)]
pub struct ApplicationSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
}

impl ApplicationSettings {
    pub fn default() -> ApplicationSettings {
        ApplicationSettings {
            window_width: 1600,
            window_height: 900,
            window_title: String::from("Hobby Window"),
        }
    }
}


pub struct Application{}

impl Application {
	pub fn new(app_settings: ApplicationSettings) -> Application {
		Application{}
	}

	pub fn start(&self, scene: Scene) {

	}

	
}
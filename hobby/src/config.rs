#[derive(Debug, Default)]
pub struct Config {
    pub window: WindowConfig,
    pub application: AppConfig,
}


#[derive(Debug)]
pub struct WindowConfig {
    pub fullscreen: bool,
    pub vsync: bool,
    pub width: u32,
    pub height: u32,
    pub bg_color: [f32; 4],
}

impl WindowConfig {
    pub fn builder() -> WindowConfigBuilder {
        WindowConfigBuilder::new()
    }
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfigBuilder::new().build()
    }
}

pub struct WindowConfigBuilder {
    pub fullscreen: bool,
    pub vsync: bool,
    pub width: u32,
    pub height: u32,
    pub bg_color: [f32; 4],
}

impl WindowConfigBuilder {
    pub fn new() -> WindowConfigBuilder {
        WindowConfigBuilder {
            fullscreen: false,
            vsync: false,
            width: 1600,
            height: 900,
            bg_color: [1.0, 1.0, 1.0, 1.0],
        }
    }

    pub fn fullscreen(mut self, fullscreen: bool) -> WindowConfigBuilder {
        self.fullscreen = fullscreen;
        self
    }

    pub fn vsync(mut self, vsync: bool) -> WindowConfigBuilder {
        self.vsync = vsync;
        self
    }

    pub fn dimensions(mut self, width: u32, height: u32) -> WindowConfigBuilder {
        self.width = width;
        self.height = height;
        self
    }

    pub fn bg_color(mut self, bg_color: [f32; 4]) -> WindowConfigBuilder {
        self.bg_color = bg_color;
        self
    }

    pub fn build(self) -> WindowConfig {
        WindowConfig {
            fullscreen: self.fullscreen,
            vsync: self.vsync,
            width: self.width,
            height: self.height,
            bg_color: self.bg_color,
        }
    }
}

#[derive(Debug)]
pub struct AppConfig {
    pub name: String,
    // [Major, Minor, Patch]
    pub version: [u32; 3],
}

impl AppConfig {
    pub fn builder() -> AppConfigBuilder {
        AppConfigBuilder::new()
    }
}

impl Default for AppConfig {
    fn default() -> AppConfig {
        AppConfigBuilder::new().build()
    }
}

pub struct AppConfigBuilder {
    name: String,
    // [Major, Minor, Patch]
    version: [u32; 3],
}

impl AppConfigBuilder {
    pub fn new() -> AppConfigBuilder {
        AppConfigBuilder {
            name: "Hobby App".into(),
            version: [1, 0, 0],
        }
    }

    pub fn name(mut self, name: &str) -> AppConfigBuilder {
        self.name = name.into();
        self
    }

    pub fn version(mut self, version: [u32; 3]) -> AppConfigBuilder {
        self.version = version;
        self
    }
    pub fn build(self) -> AppConfig {
        AppConfig {
            name: self.name,
            version: self.version,
        }
    }
}
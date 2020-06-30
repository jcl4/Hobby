use std::{path::Path, fs::File, io::Read};
use crate::Result;
use toml;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub window: WindowConfig,
    pub application: AppConfig,
}

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    pub fullscreen: bool,
    pub vsync: bool,
    pub width: i32,
    pub height: i32,
    pub bg_color: [f32; 4],
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub name: String,
}

pub fn get_config(config_file: &Path) -> Result<Config> {
    let mut file = File::open(config_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
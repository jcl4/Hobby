use failure;
use nalgebra_glm as glm;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;

mod game;
mod settings;
mod tools;

pub mod core;
pub mod renderer;

pub use crate::game::Game;
pub use crate::settings::AppInfo;
pub use crate::settings::HobbySettings;
pub use crate::settings::WindowSettings;

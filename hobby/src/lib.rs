use failure;
// use nalgebra as na;
// use nalgebra_glm as glm;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;

mod game;
mod settings;

// pub mod core;
pub mod renderer;
pub mod tools;

pub use crate::game::Game;
pub use crate::settings::AppInfo;
pub use crate::settings::HobbySettings;
pub use crate::settings::Version;
pub use crate::settings::WindowSettings;

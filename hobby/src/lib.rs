use failure;
use std::result;

pub type Result<T> = result::Result<T, failure::Error>;

mod game;
mod renderer;
mod settings;

pub use crate::game::Game;
pub use crate::settings::AppInfo;
pub use crate::settings::HobbySettings;
pub use crate::settings::WindowSettings;

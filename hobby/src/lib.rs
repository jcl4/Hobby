#[macro_use]
extern crate log;
extern crate voodoo;
extern crate voodoo_winit;
extern crate winit;
#[macro_use]
extern crate failure;

mod game;
mod renderer;
mod settings;

pub use game::Game;
pub use settings::AppInfo;
pub use settings::WindowSettings;

use std::result;
type Result<T> = result::Result<T, failure::Error>;

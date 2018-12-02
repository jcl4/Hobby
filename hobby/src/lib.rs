#[macro_use]
extern crate log;
extern crate gfx_backend_vulkan as vulkan;
extern crate gfx_hal as hal;
extern crate winit;

mod game;
mod settings;

pub use game::Game;
pub use settings::AppInfo;
pub use settings::HobbySettings;
pub use settings::WindowSettings;

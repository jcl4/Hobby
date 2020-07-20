#![warn(clippy::all)]

use log::info;
// use std::time::{Duration, Instant},
// };

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub mod config;
mod input;
mod model;
mod renderer;

pub(crate) use renderer::pipeline::Pipeline;

pub use input::InputState;
pub use renderer::Renderer;
pub use model::{Model, material::Material};

use config::Config;

pub fn get_window_and_event_loop(config: &Config) -> (Window, EventLoop<()>) {
    let (window, event_loop) = {
        let width = config.window.width;
        let height = config.window.height;

        let title = config.application.name.clone();

        let event_loop = EventLoop::new();
        let physical_size = PhysicalSize::new(width, height);

        let window = WindowBuilder::new()
            .with_inner_size(physical_size)
            .with_title(title)
            .build(&event_loop)
            .unwrap();
        (window, event_loop)
    };
    info!("Window and Event Loop Created");
    (window, event_loop)
}

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub mod scene;

pub mod config;
use config::Config;

mod input;
pub use input::InputState;

mod renderer;
pub use renderer::Renderer;



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
    log::info!("Window and Event Loop Created");
    (window, event_loop)
}
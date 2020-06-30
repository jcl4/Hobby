#![warn(clippy::all)]

use log::info;
use std::{error::Error, path::Path};
use std::time::{Duration, Instant};
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub mod config;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;


pub struct Hobby {
//     window: Window,
//     event_loop: EventLoop<()>,
//     input_state: input::InputState,
//     renderer: renderer::Renderer,
}

impl Hobby {
    pub fn new() -> Result<Hobby> {
        let config_file = Path::new("./default_config.toml");
        Hobby::from_file(config_file)   
    }

    pub fn from_config(config: config::Config) -> Result<Hobby> {
        let start = Instant::now();
        info!("Initialization of Hobby Engine Started");
        info!("{:#?}", config);


        let init_time = start.elapsed();
        info!("Initialization complete in {} s", init_time.as_secs_f32());
        Ok(Hobby{})
    }

    pub fn from_file(config_file: &Path) -> Result<Hobby> {
        let config = config::get_config(config_file)?;
        Hobby::from_config(config)
    }



    /// Game loop lives here
    pub fn run(self) {
        // info!("Game Loop Starting");
        // let mut input_state = self.input_state;
        // let window = self.window;
        // let mut renderer = self.renderer;

        // self.event_loop.run(move |event, _, control_flow| {
        //     match event {
        //         Event::MainEventsCleared => {
        //             if input_state.is_key_pressed(VirtualKeyCode::Escape) {
        //                 info!("Escape Key Pressed.");
        //                 *control_flow = ControlFlow::Exit;
        //             }
        //             // scene.update();
        //             // window.request_redraw();
        //         }
        //         Event::RedrawRequested(_) => {
        //             renderer.render();
        //             // frame_timer.tic();
        //         }
        //         Event::WindowEvent {
        //             event: WindowEvent::CloseRequested,
        //             ..
        //         } => *control_flow = ControlFlow::Exit,
        //         Event::WindowEvent {
        //             event: WindowEvent::Resized(physical_size),
        //             ..
        //         } => renderer.resize(physical_size),

        //         Event::WindowEvent {
        //             event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
        //             ..
        //         } => renderer.resize(*new_inner_size),

        //         Event::LoopDestroyed => {
        //             info!("Loop Destroyed");
        //         }
        //         Event::DeviceEvent { event, .. } => {
        //             input_state.update(&event);
        //         }
        //         // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        //         // dispatched any events. This is ideal for games and similar applications.
        //         _ => *control_flow = ControlFlow::Poll,
        //     }
        // });
    }
}

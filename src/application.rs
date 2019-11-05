use crate::{renderer::Renderer, InputState, WindowSettings};
use log::info;

use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct Application {
    window_settings: WindowSettings,
}

impl Application {
    pub fn new(window_settings: WindowSettings) -> Application {
        info!("Window Settings: {:#?}", window_settings);

        Application { window_settings }
    }

    pub fn start(self) {
        info!("Starting Application Loop");

        let (window, event_loop) = {
            let width = self.window_settings.width;
            let height = self.window_settings.height;

            let title = self.window_settings.title.clone();

            let event_loop = EventLoop::new();
            let monitor = event_loop.primary_monitor();
            let dpi = monitor.hidpi_factor();

            let physical_size = PhysicalSize::from((width, height));
            let logical_size = physical_size.to_logical(dpi);

            let window = WindowBuilder::new()
                .with_inner_size(logical_size)
                .with_title(title)
                .build(&event_loop)
                .unwrap();
            (window, event_loop)
        };
        info!("Window and Event Loop Created");

        let renderer = Renderer::new(&window, &self.window_settings);
        let input_state = InputState::new();

        run(window, event_loop, renderer, input_state);
    }
}

fn run(window: Window, event_loop: EventLoop<()>, renderer: Renderer, mut input_state: InputState) {
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::EventsCleared => {
                if input_state.is_key_pressed(VirtualKeyCode::Escape) {
                    info!("Escape Key Pressed.");
                    *control_flow = ControlFlow::Exit;
                }
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {}
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::LoopDestroyed => {
                info!("Loop Destroyed");
            }
            Event::DeviceEvent { event, .. } => {
                input_state.update(&event);
            }
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            _ => *control_flow = ControlFlow::Poll,
        }
    });
}

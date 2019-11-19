use crate::{Scene, renderer::Renderer, InputState};

use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use log::info;

/// Application Configuration
#[derive(Debug)]
pub struct ApplicationSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
}

impl ApplicationSettings {
    /// Default Settings
    /// ```rust, no_run
    /// window_width: 1600,
    /// window_height: 900,
    /// window_title: String::from("Hobby Window"),
    ///```
    pub fn default() -> ApplicationSettings {
        ApplicationSettings {
            window_width: 1600,
            window_height: 900,
            window_title: String::from("Hobby Window"),
        }
    }
}

/// The main entry point to the library and configured via [ApplicationSettings](struct.ApplicationSettings.html)
///
/// Contains  Renderer, and Input State
///
/// Passed to build functions that need access to GPU resources through renderer - see [ObjectBuilder](scene/object/struct.ObjectBuilder.html)
pub struct Application {
    window: Window,
    event_loop: EventLoop<()>,
    renderer: Renderer,
    input_state: InputState,

}

impl Application {
	pub fn new(app_settings: ApplicationSettings) -> Application {

        let (window, event_loop) = {
            let width = app_settings.window_width;
            let height = app_settings.window_height;

            let title = app_settings.window_title.clone();

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

        let renderer = Renderer::new(&window);
        let input_state = InputState::new();
		Application{
            window,
            event_loop,
            renderer,
            input_state,
        }
	}

    /// Game loop lives here
	pub fn run(self, scene: Scene) {
        info!("Game Loop Starting");
        let mut input_state = self.input_state;
        let window = self.window;
        let renderer = self.renderer;

        self.event_loop.run(move |event, _, control_flow| {
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
            } => {
                renderer.render();
            }
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
}


use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowEvent};

use renderer::Renderer;
use AppInfo;
use WindowSettings;

use Result;

pub struct Game {
    events_loop: EventsLoop,
    renderer: Renderer,
}

impl Game {
    pub fn new(window_settings: WindowSettings, app_info: AppInfo) -> Result<Game> {
        info!("Initializing Hobby Engine");
        let events_loop = EventsLoop::new();
        let renderer = Renderer::new(&events_loop, &window_settings, &app_info)?;
        Ok(Game {
            events_loop,
            renderer,
        })
    }

    pub fn run(&mut self) {
        let mut running = true;

        while running {
            running = manage_input(&mut self.events_loop);
        }
    }
}

fn manage_input(events_loop: &mut EventsLoop) -> bool {
    let mut running = true;

    events_loop.poll_events(|event| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            running = false;
        }
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                },
            ..
        } => {
            running = false;
        }
        Event::WindowEvent {
            event: WindowEvent::HiDpiFactorChanged(dpi),
            ..
        } => {
            //TODO: Figure out what to do here...
            info!("DPI Changed: {}", dpi);
        }

        _ => (),
    });
    running
}

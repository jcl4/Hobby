use crate::renderer::Renderer;
use crate::{HobbySettings, Result};
use log::info;
use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Game {
    renderer: Renderer,
    events_loop: EventsLoop,
}

impl Game {
    pub fn new(hobby_settings: HobbySettings) -> Result<Game> {
        let events_loop = EventsLoop::new();
        let renderer = Renderer::new(hobby_settings, &events_loop)?;
        Ok(Game {
            renderer,
            events_loop,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut running = true;
        while running {
            running = manage_input(&mut self.events_loop);
        }
        Ok(())
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

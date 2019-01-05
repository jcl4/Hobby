use crate::renderer::Renderer;
use crate::tools::FrameTimer;
use crate::{HobbySettings, Result};
use ash::version::DeviceV1_0;

use log::info;
use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Game {
    renderer: Renderer,
    events_loop: EventsLoop,
    frame_timer: FrameTimer,
}

impl Game {
    pub fn new(hobby_settings: HobbySettings) -> Result<Game> {
        let events_loop = EventsLoop::new();
        let renderer = Renderer::new(&hobby_settings, &events_loop)?;
        let frame_timer = FrameTimer::new(
            hobby_settings.display_update_duration,
            &hobby_settings.app_info.app_name,
        );

        Ok(Game {
            renderer,
            events_loop,
            frame_timer,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut running = true;

        self.frame_timer.start();

        let mut update_debug: bool;

        while running {
            update_debug = self.frame_timer.kick();
            // self.renderer.draw_frame()?;
            running = manage_input(&mut self.events_loop);
        }
        self.frame_timer.stop()?;
        unsafe { self.renderer.device.device_wait_idle()? };

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

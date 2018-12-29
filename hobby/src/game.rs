use crate::core::Model;
use crate::na;
use crate::renderer::Renderer;
use crate::tools::FrameTimer;
use crate::{HobbySettings, Result};

use log::info;
use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Game {
    renderer: Renderer,
    events_loop: EventsLoop,
    frame_timer: FrameTimer,
    models: Option<Vec<Model>>,
    view: na::Isometry3<f32>,
}

impl Game {
    pub fn new(hobby_settings: HobbySettings) -> Result<Game> {
        let events_loop = EventsLoop::new();
        let renderer = Renderer::new(&hobby_settings, &events_loop)?;
        let frame_timer = FrameTimer::new(
            hobby_settings.display_update_duration,
            &hobby_settings.app_info.app_name,
        );

        let eye = na::Point3::new(0.0, 0.0, -5.0);
        let target = na::Point3::new(0.0, 0.0, 0.0);
        let up = na::Vector3::new(0.0, -1.0, 0.0);
        let view = na::Isometry3::new_observer_frame(&eye, &target, &up);

        Ok(Game {
            renderer,
            events_loop,
            frame_timer,
            models: None,
            view,
        })
    }

    pub fn add_model(&mut self, mut model: Model) -> Result<()> {
        model.build(&self.renderer)?;
        match self.models.as_mut() {
            Some(models) => models.push(model),
            None => self.models = Some(vec![model]),
        }

        Ok(())
    }

    pub fn add_models(&mut self, models: Vec<Model>) -> Result<()> {
        for model in models {
            self.add_model(model)?;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let mut running = true;

        self.frame_timer.start();

        let mut update_debug: bool;

        while running {
            running = manage_input(&mut self.events_loop);
            update_debug = self.frame_timer.kick();

            match self.models.as_mut() {
                Some(models) => {
                    for model in models.iter_mut() {
                        model.update(self.frame_timer.frame_time(), update_debug);
                    }

                    self.renderer.draw_frame(models, &self.view)?
                }
                None => {}
            }
        }

        self.frame_timer.stop()?;
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

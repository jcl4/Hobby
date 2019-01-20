use crate::{core::Model, graphics::Renderer, na, tools::FrameTimer};
use crate::{HobbySettings, Result};
use ash::version::DeviceV1_0;
use log::info;
use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Game {
    renderer: Renderer,
    events_loop: EventsLoop,
    frame_timer: FrameTimer,
    models: Vec<Model>,
}

impl Game {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(hobby_settings: &HobbySettings) -> Result<Game> {
        let events_loop = EventsLoop::new();
        let renderer = Renderer::new(&hobby_settings, &events_loop)?;
        let frame_timer = FrameTimer::new(
            hobby_settings.display_update_duration,
            &hobby_settings.app_info.app_name,
        );

        let models = vec![];

        Ok(Game {
            renderer,
            events_loop,
            frame_timer,
            models,
        })
    }

    pub fn add_model(&mut self, mut model: Model) -> Result<()> {
        model.build(&self.renderer)?;
        self.models.push(model);
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let mut running = true;

        self.frame_timer.start();

        // TODO: This really should be a part of the renderer
        self.renderer.command_buffer_data.build_cb(
            &self.renderer.device,
            &self.renderer.swapchain_data,
            &self.renderer.framebuffers,
            self.renderer.render_pass,
            &self.models,
        )?;

        let view = na::Matrix4::identity();
        let proj = na::Matrix4::identity();

        while running {
            let update_debug = self.frame_timer.kick();
            let dt = self.frame_timer.frame_time();

            for model in self.models.iter_mut() {
                model.update(dt, update_debug);
            }

            self.renderer.draw_frame(&mut self.models, view, proj)?;
            running = manage_input(&mut self.events_loop, &mut self.renderer);
        }

        self.frame_timer.stop()?;
        unsafe { self.renderer.device.device_wait_idle()? };
        self.renderer.cleanup(&self.models)?;

        Ok(())
    }
}

fn manage_input(events_loop: &mut EventsLoop, renderer: &mut Renderer) -> bool {
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

        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            info!("Resized");
            renderer.is_resized = true;
        }

        _ => (),
    });
    running
}


#![warn(clippy::all)]
use std::time::{Duration, Instant};
use winit::{event::{Event, WindowEvent}, event_loop::{EventLoop, ControlFlow}, window::{WindowBuilder, Window}, dpi::PhysicalSize};
use log::info;

pub(crate) mod renderer;


#[derive(Debug)]
pub struct HobbySettings {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
    pub app_name: String,
    pub app_version: u32,
    pub frame_timer_display_interval: Duration,
}

impl HobbySettings {
    pub fn default() -> HobbySettings {
        HobbySettings {
            window_width: 1600,
            window_height: 900,
            window_title: String::from("Hobby Window"),
            app_name: String::from("Hobby Application"),
            app_version: 0,
            frame_timer_display_interval: Duration::from_secs_f32(60.0),
        }
    }
}

pub struct Hobby {
    window: Window,
    event_loop: EventLoop<()>,
    hobby_settings: HobbySettings,
}

impl Hobby {
    pub fn new(hobby_settings: HobbySettings) -> Hobby {
        let init_start = Instant::now();

        let (window, event_loop) = {
            let width = hobby_settings.window_width;
            let height = hobby_settings.window_height;

            let title = hobby_settings.window_title.clone();

            let event_loop = EventLoop::new();
            let size: PhysicalSize<u32> = PhysicalSize::from((width, height));

            let window = WindowBuilder::new()
                .with_inner_size(size)
                .with_title(title)
                .build(&event_loop)
                .unwrap();
            (window, event_loop)
        };
        info!("Window and Event Loop Created");

        // let renderer = Renderer::new(&window, &hobby_settings.app_name, hobby_settings.app_version);
        // let input_state = InputState::new();

        info!(
            "Hobby initialization time: {:#?} sec",
            Instant::now().duration_since(init_start).as_secs_f32()
        );

        Hobby {
            window,
            event_loop,
            hobby_settings,
        }
    }

    /// Game loop lives here
    pub fn run(self) {
        info!("Game Loop Starting");
        let window = self.window;

        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::MainEventsCleared => {
                    // if input_state.is_key_pressed(VirtualKeyCode::Escape) {
                    //     info!("Escape Key Pressed.");
                    //     *control_flow = ControlFlow::Exit;
                    // }
                    // scene.update();
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    // renderer.draw_frame(&scene);
                    // frame_timer.tic();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit
                }
                Event::LoopDestroyed => {
                    info!("Loop Destroyed");
                }
                Event::DeviceEvent { event, .. } => {
                    // input_state.update(&event);
                }
                // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
                // dispatched any events. This is ideal for games and similar applications.
                _ => *control_flow = ControlFlow::Poll,
            }
        });
    }
}
use std::{env, path::Path, time::Instant};

use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use hobby::{
    config::{AppConfig, Config, WindowConfig},
    gpu,
};

use futures::executor::block_on;

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let full_name = format!("{}.log", name);

    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let example_dir = root_dir.join("examples").join(name);
    let log_file = example_dir.join(full_name);

    hobby::setup_logging(&log_file);

    let start = Instant::now();
    log::info!("Starting!");
    let config = create_config();
    log::info!("{:#?}", config);

    let (window, event_loop) = hobby::get_window_and_event_loop(&config);
    let mut input_state = hobby::InputState::new();
    let mut context = block_on(gpu::Context::new(&config, &window));

    // let triangle = create_triangle_model(&mut renderer);
    // let models = vec![triangle];
    // let scene = SceneBuilder::new(models).build(&renderer);

    let init_time = start.elapsed();
    log::info!("Initialization complete in {} s", init_time.as_secs_f32());
    log::info!("Starting Game Loop");

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                if input_state.is_key_pressed(VirtualKeyCode::Escape) {
                    log::info!("Escape Key Pressed");
                    *control_flow = ControlFlow::Exit;
                }
                // scene.update();
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // renderer.render(&scene);
                // frame_timer.tic();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => context.resize(physical_size),

            // Event::WindowEvent {
            //     event: WindowEvent::ScaleFactorChanged { new_inner_size, .. },
            //     ..
            // } => renderer.resize(*new_inner_size),
            Event::LoopDestroyed => {
                log::info!("Game Loop Stopped");
                // triangle.cleanup(&renderer);
                // renderer.cleanup();
                std::process::exit(0);
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

pub fn create_config() -> Config {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();

    let app_config = AppConfig::builder()
        .name("Triangle Example")
        .version([major, minor, patch])
        .build();
    let bg_color = [0.757, 0.258, 0.121, 1.0];
    let window_config = WindowConfig::builder()
        .bg_color(bg_color)
        .vsync(false)
        .build();

    Config {
        window: window_config,
        application: app_config,
    }
}

// pub fn create_triangle_model(renderer: &mut Renderer) -> Model {
//     let vertices = vec![
//         Vertex {
//             position: [0.0, 0.5, 0.0],
//             color: [1.0, 0.0, 0.0],
//         },
//         Vertex {
//             position: [-0.5, -0.5, 0.0],
//             color: [0.0, 1.0, 0.0],
//         },
//         Vertex {
//             position: [0.5, -0.5, 0.0],
//             color: [0.0, 0.0, 1.0],
//         },
//     ];

//     let indices = vec![0, 1, 2];

//     let mesh = Mesh::new(vertices, indices);
//     let material = Material::Basic;

//     Model::new(mesh, material, renderer)
// }

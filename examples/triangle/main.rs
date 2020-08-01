use std::{env, fs::File, path::Path, time::Instant};

use simplelog as sl;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use hobby::{
    config::{AppConfig, Config, WindowConfig},
    model::{Material, Mesh, Model, Vertex},
    Renderer,
};

use futures::executor::block_on;

fn main() {
    setup_logging();
    let start = Instant::now();
    log::info!("Starting!");
    let config = create_config();
    log::info!("{:#?}", config);

    let (window, event_loop) = hobby::get_window_and_event_loop(&config);
    let mut input_state = hobby::InputState::new();
    let mut renderer = block_on(hobby::Renderer::new(&config, &window));

    let triangle = create_triangle_model(&mut renderer);
    let models = vec![triangle];
    renderer.build_pipelines(&models);

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
                renderer.render(&models);
                // frame_timer.tic();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => renderer.resize(physical_size),

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

pub fn setup_logging() {
    let time_format = "%F %H:%M:%S.%3f";
    let log_config = sl::ConfigBuilder::new()
        .set_time_format_str(time_format)
        .build();

    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let example_dir = root_dir.join("examples").join("triangle");
    let log_file = example_dir.join("triangle.log");

    sl::CombinedLogger::init(vec![
        sl::TermLogger::new(
            sl::LevelFilter::Warn,
            log_config.clone(),
            sl::TerminalMode::Mixed,
        ),
        sl::WriteLogger::new(
            sl::LevelFilter::Info,
            log_config,
            File::create(log_file).unwrap(),
        ),
    ])
    .expect("Unable to create logger");
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

pub fn create_triangle_model(renderer: &mut Renderer) -> Model {
    let vertices = vec![
        Vertex {
            position: [0.0, 0.5, 0.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ];

    let mesh = Mesh::new(vertices, vec![]);
    let material = Material::Basic;

    Model::new(mesh, material, renderer)
}

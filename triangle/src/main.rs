use std::{env, mem, path::Path, time::Instant};

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

    let log_file = root_dir.join(full_name);

    hobby::setup_logging(&log_file);

    let start = Instant::now();
    log::info!("Starting!");
    let config = create_config();
    log::info!("{:#?}", config);

    let (window, event_loop) = hobby::get_window_and_event_loop(&config);
    let mut input_state = hobby::InputState::new();
    let mut context = block_on(gpu::Context::new(&config, &window));

    let triangle = Triangle::new(&context);

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
                draw(&config.window, &triangle, &mut context);
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

fn draw(window_config: &WindowConfig, triangle: &Triangle, context: &mut gpu::Context) {
    let (frame, mut encoder) = context.get_frame_data();
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: window_config.bg_color[0],
                    g: window_config.bg_color[1],
                    b: window_config.bg_color[2],
                    a: window_config.bg_color[3],
                },
            }],
            depth_stencil_attachment: None,
        });
        triangle.draw(&mut render_pass);
    }
    context.submit_command(encoder);
}

fn create_config() -> Config {
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

#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
        }
    }
}

struct Triangle {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    pipeline: wgpu::RenderPipeline,
}

impl Triangle {
    fn new(context: &gpu::Context) -> Triangle {
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

        let vertex_buffer = context
            .device
            .create_buffer_with_data(bytemuck::cast_slice(&vertices), wgpu::BufferUsage::VERTEX);

        let indices: Vec<u16> = vec![0, 1, 2];
        let num_indices = indices.len() as _;

        let index_buffer = context
            .device
            .create_buffer_with_data(bytemuck::cast_slice(&indices), wgpu::BufferUsage::INDEX);

        let pipeline_layout_desc = wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[],
        };

        let pipeline_data = gpu::PipelineData {
            vert_str: include_str!("../shaders/shader.vert"),
            vert_name: "basic.vert",
            frag_str: include_str!("../shaders/shader.frag"),
            frag_name: "basic.frag",
            vert_desc: Vertex::desc(),
            pipeline_layout_desc,
        };

        let pipeline = gpu::create_pipeline(pipeline_data, context);

        Triangle {
            vertex_buffer,
            index_buffer,
            num_indices,
            pipeline,
        }
    }

    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        render_pass.set_index_buffer(&self.index_buffer, 0, 0);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}

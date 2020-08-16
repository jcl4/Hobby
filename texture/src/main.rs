use futures::executor::block_on;
use hobby::{
    config::{AppConfig, Config, WindowConfig},
    gpu,
};

use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use std::{env, mem, path::Path, time::Instant};

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

    let num_segments = 3;
    let circle_mesh = CircleMesh::new(num_segments);
    let mut circle = Circle::new(&circle_mesh, &context);

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

                circle.update(&input_state, &context);
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                draw(&config.window, &circle, &mut context);
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

fn create_config() -> Config {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();

    let app_config = AppConfig::builder()
        .name("Texture Example")
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
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
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
                    // We only need to change this to reflect that tex_coords
                    // is only 2 floats and not 3. It's in the same position
                    // as color was, so nothing else needs to change
                    format: wgpu::VertexFormat::Float2,
                },
            ],
        }
    }
}

fn draw(window_config: &WindowConfig, circle: &Circle, context: &mut gpu::Context) {
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
        circle.draw(&mut render_pass);
    }
    context.submit_command(encoder);
}

struct CircleMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl CircleMesh {
    fn new(num_segments: u8) -> CircleMesh {
        let full_circle = 360.0_f32.to_radians();
        let seg_size = full_circle / num_segments as f32;

        let mut vertices = vec![Vertex {
            position: [0.0, 0.0, 0.0],
            tex_coords: [0.5, 0.5],
        }];

        let mut angle = 0.0_f32;
        for _ in 0..num_segments {
            let x_val = angle.cos();
            let y_val = angle.sin();
            vertices.push(Vertex {
                position: [x_val, y_val, 0.0],
                tex_coords: [(x_val + 1.0) / 2.0, (y_val + 1.0) / 2.0],
            });
            angle += seg_size;
        }

        let mut indices: Vec<u16> = vec![];
        for segment in 0..(num_segments - 1) as _ {
            let mut temp: Vec<u16> = vec![0, segment + 1, segment + 2];
            indices.append(&mut temp);
        }

        let mut temp: Vec<u16> = vec![0, num_segments as _, 1];
        indices.append(&mut temp);

        CircleMesh { vertices, indices }
    }
}

struct Circle {
    diff_texture: gpu::Texture,
    diff_bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    pipeline: wgpu::RenderPipeline,
}

impl Circle {
    fn new(circle_mesh: &CircleMesh, context: &gpu::Context) -> Circle {
        let bytes = include_bytes!("../oil_painting_texture.jpg");
        let (diff_texture, cmd_buffer) = gpu::Texture::from_bytes(bytes, context, "oil_diff");
        context.queue.submit(&[cmd_buffer]);

        let bgl_desc = wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2,
                        component_type: wgpu::TextureComponentType::Uint,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
            label: Some("texture_bind_group_layout"),
        };

        let texture_bind_group_layout = context.device.create_bind_group_layout(&bgl_desc);

        let bg_desc = wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diff_texture.view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diff_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        };

        let diff_bind_group = context.device.create_bind_group(&bg_desc);

        let (vertex_buffer, index_buffer) =
            gpu::create_vertex_index_buffers(&circle_mesh.vertices, &circle_mesh.indices, &context);

        let num_indices = circle_mesh.indices.len() as _;

        let pipeline_layout_desc = wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&texture_bind_group_layout],
        };

        let pipeline_data = gpu::PipelineData {
            vert_str: include_str!("../shaders/shader.vert"),
            vert_name: "shader.vert",
            frag_str: include_str!("../shaders/shader.frag"),
            frag_name: "shader.frag",
            vert_desc: Vertex::desc(),
            pipeline_layout_desc,
        };

        let pipeline = gpu::create_pipeline(pipeline_data, context);

        Circle {
            diff_texture,
            diff_bind_group,
            vertex_buffer,
            index_buffer,
            num_indices,
            pipeline,
        }
    }

    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.diff_bind_group, &[]);
        render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        render_pass.set_index_buffer(&self.index_buffer, 0, 0);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    fn update(&mut self, input_state: &hobby::InputState, context: &gpu::Context) {
        let num_segments: i16 = if input_state.is_key_pressed(VirtualKeyCode::Key3) {
            3
        } else if input_state.is_key_pressed(VirtualKeyCode::Key4) {
            4
        } else if input_state.is_key_pressed(VirtualKeyCode::Key5) {
            5
        } else if input_state.is_key_pressed(VirtualKeyCode::Key6) {
            6
        } else if input_state.is_key_pressed(VirtualKeyCode::Key7) {
            7
        } else if input_state.is_key_pressed(VirtualKeyCode::Key8) {
            8
        } else if input_state.is_key_pressed(VirtualKeyCode::Key9) {
            9
        } else if input_state.is_key_pressed(VirtualKeyCode::Key0) {
            255
        } else {
            -1
        };

        if num_segments > 2 {
            self.create_new_buffers(&CircleMesh::new(num_segments as u8), context);
        }
    }

    fn create_new_buffers(&mut self, circle_mesh: &CircleMesh, context: &gpu::Context) {
        // self.vertex_buffer.unmap();
        // &self.index_buffer.unmap();
        // if let Some(v_buff) = & {
        //     v_buff.unmap();
        //     self.vertex_buffer = None;
        //     build_new_buffers = true;
        // }

        // if let Some(i_buff) =  {
        //     // log::error!("Index buffer unmapped");
        //     i_buff.unmap();
        //     self.index_buffer = None;
        //     build_new_buffers = true;
        // }

        let (v_buff, i_buff) = hobby::gpu::create_vertex_index_buffers(
            &circle_mesh.vertices,
            &circle_mesh.indices,
            context,
        );
        self.vertex_buffer = v_buff;
        self.index_buffer = i_buff;
        self.num_indices = circle_mesh.indices.len() as _;
    }
}

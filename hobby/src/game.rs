use winit::dpi::PhysicalSize;
use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowBuilder, WindowEvent};

use HobbySettings;
use WindowSettings;

use hal;
use hal::adapter::DeviceType;
use hal::format::{ChannelType, Format};
use hal::image::{Access, Layout};
use hal::pass::{
    Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDependency, SubpassDesc,
    SubpassRef,
};
use hal::pool::{CommandPool, CommandPoolCreateFlags};
use hal::pso::PipelineStage;
use hal::{Device, Graphics, Instance, Surface};
use vulkan;

pub struct Game {
    events_loop: EventsLoop,
    _window: Window,
    _instance: vulkan::Instance,
    device: vulkan::Device,
    command_pool: CommandPool<vulkan::Backend, Graphics>,
    render_pass: <vulkan::Backend as hal::Backend>::RenderPass,
}

impl Game {
    pub fn new(hobby_settings: HobbySettings) -> Game {
        info!("Initializing Hobby Engine");
        let events_loop = EventsLoop::new();
        let window = create_window(&events_loop, &hobby_settings.window_settings);

        let instance = vulkan::Instance::create("Hobby", 1);
        let surface = instance.create_surface(&window);
        let mut adapters = instance.enumerate_adapters();

        let mut adapter_found = false;
        let mut adapter_index = 0;

        for (i, adapter) in adapters.iter().enumerate() {
            info!("Adapter info: {:#?}", adapter.info);

            match adapter.info.device_type {
                DeviceType::DiscreteGpu => {
                    adapter_found = true;
                    adapter_index = i;
                    break;
                }
                _ => {}
            }
        }

        if !adapter_found {
            error!("Unable to find suitable adapter");
            panic!("Uanble to find suitable adapter");
        } else {
        };

        let adapter = adapters.remove(adapter_index);

        let num_queues = 1;
        let (device, queue_group) = adapter
            .open_with::<_, Graphics>(num_queues, |family| surface.supports_queue_family(family))
            .unwrap();

        let max_buffers = 16;
        let command_pool = device
            .create_command_pool_typed(&queue_group, CommandPoolCreateFlags::empty(), max_buffers)
            .expect("Unable to create command pool");

        let physical_device = &adapter.physical_device;

        let (caps, formats, present_modes) = surface.compatibility(physical_device);

        info!("Capabilities: {:#?}", caps);

        info!("Availabe Present Modes: ");
        for present_mode in present_modes {
            info!("\t{:#?}", present_mode);
        }

        let surface_color_format = {
            match formats {
                Some(choices) => choices
                    .into_iter()
                    .find(|format| format.base_format().1 == ChannelType::Srgb)
                    .unwrap(),
                None => Format::Rgba8Srgb,
            }
        };

        let render_pass = {
            let color_attachement = Attachment {
                format: Some(surface_color_format),
                samples: 1,
                ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::Present,
            };

            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            let dependency = SubpassDependency {
                passes: SubpassRef::External..SubpassRef::Pass(0),
                stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT
                    ..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
                accesses: Access::empty()
                    ..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
            };

            device
                .create_render_pass(&[color_attachement], &[subpass], &[dependency])
                .expect("Unable to create render pass")
        };

        // let pipeline_layout

        Game {
            events_loop,
            _window: window,
            _instance: instance,
            device,
            command_pool,
            render_pass,
        }
    }

    pub fn run(&mut self) {
        let mut running = true;

        while running {
            running = manage_input(&mut self.events_loop);
        }
    }

    pub fn cleanup(self) {
        self.device.destroy_render_pass(self.render_pass);
        self.device
            .destroy_command_pool(self.command_pool.into_raw());
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

pub fn create_window(events_loop: &EventsLoop, window_settings: &WindowSettings) -> Window {
    let monitor = events_loop.get_primary_monitor();
    let dpi = monitor.get_hidpi_factor();

    let physical_size = PhysicalSize::new(window_settings.width, window_settings.height);
    let logical_size = physical_size.to_logical(dpi);

    let window = WindowBuilder::new()
        .with_dimensions(logical_size)
        .with_title(window_settings.title.clone())
        .build(events_loop)
        .expect("Unable to create window");

    let size = window.get_inner_size().expect("Unable to get window size");

    info!("Built Window");
    info!("\tWindow Size: {:?}", size.to_physical(dpi));

    window
}

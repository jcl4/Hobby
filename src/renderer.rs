pub mod pipelines;

use log::info;
use raw_window_handle;
use wgpu::{Adapter, BackendBit, Device, PowerPreference, Queue, RequestAdapterOptions, Surface};

use crate::{
    renderer::pipelines::{ColoredMeshModel, ColoredMeshPipeline},
    WindowSettings,
};

pub(crate) struct Renderer {
    _surface: Surface,
    _adapter: Adapter,
    device: Device,
    queue: Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pipeline: ColoredMeshPipeline,
}

impl Renderer {
    pub fn new<W: raw_window_handle::HasRawWindowHandle>(
        window: &W,
        window_settings: &WindowSettings,
        model: ColoredMeshModel,
    ) -> Renderer {
        let surface = Surface::create(window);
        info!("Surface Created");
        let adapter = Adapter::request(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            backends: BackendBit::VULKAN,
        })
        .expect("Unable to create Adapter");

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window_settings.width,
            height: window_settings.height,
            present_mode: wgpu::PresentMode::NoVsync,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let pipeline = ColoredMeshPipeline::new(&device, &sc_desc, model);

        Renderer {
            _surface: surface,
            _adapter: adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            pipeline,
        }
    }

    pub fn render(&mut self) {
        let frame = self.swap_chain.get_next_texture();
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

        let color_att = wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame.view,
            resolve_target: None,
            load_op: wgpu::LoadOp::Clear,
            store_op: wgpu::StoreOp::Store,
            clear_color: wgpu::Color {
                r: 0.07,
                g: 0.12,
                b: 0.08,
                a: 1.0,
            },
        };
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[color_att],
                depth_stencil_attachment: None,
            });

            self.pipeline.draw(&mut render_pass);
        }
        let command_buffer = encoder.finish();
        self.queue.submit(&[command_buffer]);
    }
}

pub mod pipelines;

use log::info;
use raw_window_handle;
use wgpu::{Adapter, BackendBit, Device, PowerPreference, Queue, RequestAdapterOptions, Surface};

use crate::{renderer::pipelines::ColoredMeshPipeline, WindowSettings};

pub(crate) struct Renderer {
    surface: Surface,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pipeline: Option<ColoredMeshPipeline>,
}

impl Renderer {
    pub fn new<W: raw_window_handle::HasRawWindowHandle>(
        window: &W,
        window_settings: &WindowSettings,
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

        Renderer {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            pipeline: None,
        }
    }

    pub fn add_pipeline(&mut self, mut pipeline: ColoredMeshPipeline) {
        pipeline.build(&self.device, &self.sc_desc);
        self.pipeline = Some(pipeline);
    }
}

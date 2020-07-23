use crate::Config;
use winit::window::Window;

pub struct Renderer {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    // sc_desc: wgpu::SwapChainDescriptor,
    // swap_chain: wgpu::SwapChain,

    // size: winit::dpi::PhysicalSize<u32>,
}

impl Renderer {
    pub async fn new(config: &Config, window: &Window) -> Renderer {
        let size = window.inner_size();
        let surface = wgpu::Surface::create(window);

        let options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        };

        let adapter = wgpu::Adapter::request(&options, wgpu::BackendBit::PRIMARY).await.expect("Unable to create adapter");
        let desc = wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {anisotropic_filtering: false},
            limits: Default::default()

        };

        let (device, queue) = adapter.request_device(&desc).await;

        Renderer {
            surface,
            adapter,
            device,
            queue
        }
    }
}

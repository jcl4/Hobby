use crate::Config;
use winit::window::Window;
pub struct Context {
    surface: wgpu::Surface,
    _adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
}

impl Context {
    pub async fn new(config: &Config, window: &Window) -> Context {
        let size = window.inner_size();
        let surface = wgpu::Surface::create(window);

        let options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
        };

        let adapter = wgpu::Adapter::request(&options, wgpu::BackendBit::PRIMARY)
            .await
            .expect("Unable to create adapter");
        let desc = wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        };

        let (device, queue) = adapter.request_device(&desc).await;

        let present_mode = if config.window.vsync {
            wgpu::PresentMode::Mailbox
        } else {
            wgpu::PresentMode::Immediate
        };

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Context {
            surface,
            _adapter: adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn get_frame_data(&mut self) -> (wgpu::SwapChainOutput, wgpu::CommandEncoder) {
        let frame = self
            .swap_chain
            .get_next_texture()
            .expect("error getting next frame");

        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };

        let encoder = self.device.create_command_encoder(&ce_desc);

        (frame, encoder)
    }

    pub fn submit_command(&self, encoder: wgpu::CommandEncoder) {
        self.queue.submit(&[encoder.finish()]);
    }
}

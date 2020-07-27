use winit::window::Window;

use super::pipeline;
use crate::model::Material;
use crate::Config;

pub struct Renderer {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    render_pipelines: Vec<wgpu::RenderPipeline>,
    size: winit::dpi::PhysicalSize<u32>,
}

impl Renderer {
    pub async fn new(config: &Config, window: &Window) -> Renderer {
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

        let mut present_mode = wgpu::PresentMode::Immediate;
        if config.window.vsync {
            present_mode = wgpu::PresentMode::Mailbox;
        }

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Renderer {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            render_pipelines: vec![],
            size,
        }
    }

    pub fn add_pipeline(&mut self, material: &Material) {
        self.render_pipelines
            .push(pipeline::create_render_pipeline(material, self));
        log::info! {"Material: {:?}, Render Pipeline Built", material};
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn render(&mut self) {
        let frame = self
            .swap_chain
            .get_next_texture()
            .expect("Timeout getting texture");

        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        };

        let mut encoder = self.device.create_command_encoder(&ce_desc);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipelines[0]);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(&[encoder.finish()]);
    }
}

use std::ffi::CStr;

use voodoo as vd;
use winit::{EventsLoop, Window};

use super::base;
use super::swapchain;
use AppInfo;
use Result;
use WindowSettings;

static VERT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/renderer/shaders/vert.spv");
static FRAG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/renderer/shaders/frag.spv");

pub struct Vertex {
    pub pos: [f32; 3],
}

pub struct Renderer {
    window: Window,
    window_size: vd::Extent2d,
    instance: vd::Instance,
    surface: vd::SurfaceKhr,

    graphics_queue_family: u32,
    present_queue_family: u32,

    graphics_queue: vd::QueueHandle,
    present_queue: vd::QueueHandle,

    device: vd::Device,

    swapchain: vd::SwapchainKhr,
    image_views: Vec<vd::ImageView>,

    render_pass: vd::RenderPass,
    frame_buffers: Vec<vd::Framebuffer>,
}

impl Renderer {
    pub fn new(
        events_loop: &EventsLoop,
        window_settings: &WindowSettings,
        app_info: &AppInfo,
    ) -> Result<Renderer> {
        info!("Initializing Renderer");

        let window = base::init_window(events_loop, window_settings);
        let instance = base::create_instance(app_info)?;
        let surface = base::create_surface(instance.clone(), &window)?;
        let physical_device = base::pick_physical_device(&instance)?;
        let (graphics_queue_family, present_queue_family) =
            base::find_queue_families(&physical_device, &surface)?;
        let (device, graphics_queue, present_queue) =
            base::create_device(physical_device, graphics_queue_family, present_queue_family)?;

        let window_size = vd::Extent2d::builder()
            .width(window_settings.width as u32)
            .height(window_settings.height as u32)
            .build();

        let swapchain = swapchain::create_swapchain(
            &surface,
            &device,
            graphics_queue_family,
            present_queue_family,
            window_settings.v_sync,
            &window_size,
            None,
        )?;

        let image_views = swapchain::create_image_views(&swapchain);
        let render_pass = base::create_render_pass(&swapchain)?;
        let frame_buffers = base::create_frame_buffers(&image_views, &window_size, &render_pass)?;
        create_pipeline(device.clone())?;

        Ok(Renderer {
            window,
            window_size,
            instance,
            surface,
            graphics_queue_family,
            present_queue_family,
            graphics_queue,
            present_queue,
            device,
            swapchain,
            image_views,
            render_pass,
            frame_buffers,
        })
    }
}

fn create_pipeline(device: vd::Device) -> Result<()> {
    let vert_shader_code = vd::util::read_spir_v_file(VERT_PATH)?;
    let frag_shader_code = vd::util::read_spir_v_file(FRAG_PATH)?;

    let vert_shader_module = vd::ShaderModule::new(device.clone(), &vert_shader_code)?;
    let frag_shader_moudle = vd::ShaderModule::new(device, &frag_shader_code)?;

    let fn_name = CStr::from_bytes_with_nul(b"main\0").unwrap();

    let vert_shader_stage_info = vd::PipelineShaderStageCreateInfo::builder()
        .stage(vd::ShaderStageFlags::VERTEX)
        .module(&vert_shader_module)
        .name(fn_name)
        .build();

    let frag_shader_stage_info = vd::PipelineShaderStageCreateInfo::builder()
        .stage(vd::ShaderStageFlags::FRAGMENT)
        .module(&frag_shader_moudle)
        .name(fn_name)
        .build();

    Ok(())
}

use std::ffi::CStr;
use std::mem;

use smallvec::SmallVec;
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
    pub pos: [f32; 2],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn binding_description() -> vd::VertexInputBindingDescription {
        vd::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(mem::size_of::<Vertex>() as u32)
            .input_rate(vd::VertexInputRate::Vertex)
            .build()
    }

    pub fn attribute_descriptions() -> [vd::VertexInputAttributeDescription; 2] {
        [
            vd::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(vd::Format::R32G32Sfloat)
                .offset(offset_of!(Vertex, pos))
                .build(),
            vd::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(vd::Format::R32G32B32Sfloat)
                .offset(offset_of!(Vertex, color))
                .build(),
        ]
    }
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

    pipeline_layout: vd::PipelineLayout,
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
        let pipeline_layout = create_pipeline_layout(device.clone())?;
        let pipeline = create_pipeline(device.clone(), &window_size, &render_pass, &pipeline_layout)?;

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
            pipeline_layout,
        })
    }
}

fn create_pipeline_layout(device: vd::Device) -> Result<vd::PipelineLayout> {
    let layouts = SmallVec::<[_; 8]>::new();

    let pipeline_layout = vd::PipelineLayout::builder()
        .set_layouts(&layouts)
        .build(device)?;

    Ok(pipeline_layout)
}

fn create_pipeline(
    device: vd::Device,
    extent: &vd::Extent2d,
    render_pass: &vd::RenderPass,
    pipeline_layout: &vd::PipelineLayout,
) -> Result<vd::GraphicsPipeline> {
    let vert_shader_code = vd::util::read_spir_v_file(VERT_PATH)?;
    let frag_shader_code = vd::util::read_spir_v_file(FRAG_PATH)?;

    let vert_shader_module = vd::ShaderModule::new(device.clone(), &vert_shader_code)?;
    let frag_shader_moudle = vd::ShaderModule::new(device.clone(), &frag_shader_code)?;

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

    let binding_description = [Vertex::binding_description()];
    let attribute_descriptions = Vertex::attribute_descriptions();

    let vertex_input_info = vd::PipelineVertexInputStateCreateInfo::builder()
        .vertex_binding_descriptions(&binding_description)
        .vertex_attribute_descriptions(&attribute_descriptions)
        .build();

    let input_assembly = vd::PipelineInputAssemblyStateCreateInfo::builder()
        .topology(vd::PrimitiveTopology::TriangleList)
        .primitive_restart_enable(false)
        .build();

    let viewport = vd::Viewport::builder()
        .x(0.0f32)
        .y(0.0f32)
        .width(extent.width() as f32)
        .height(extent.height() as f32)
        .min_depth(0.0f32)
        .max_depth(1.0f32)
        .build();

    let scissor = vd::Rect2d::builder()
        .offset(vd::Offset2d::builder().x(0).y(0).build())
        .extent(extent.clone())
        .build();

    let viewports = [viewport];
    let scissors = [scissor];

    let viewport_state = vd::PipelineViewportStateCreateInfo::builder()
        .viewports(&viewports)
        .scissors(&scissors)
        .build();

    let rasterizer = vd::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(vd::PolygonMode::Fill)
        .cull_mode(vd::CullModeFlags::NONE)
        .front_face(vd::FrontFace::CounterClockwise)
        .depth_bias_enable(false)
        .depth_bias_constant_factor(0.0f32)
        .depth_bias_clamp(0.0f32)
        .depth_bias_slope_factor(0.0f32)
        .line_width(1.0f32)
        .build();

    let multisampling = vd::PipelineMultisampleStateCreateInfo::builder()
        .rasterization_samples(vd::SampleCountFlags::COUNT_1)
        .sample_shading_enable(false)
        .min_sample_shading(1.0f32)
        .alpha_to_coverage_enable(false)
        .alpha_to_one_enable(false)
        .build();

    let stencil_op_state = vd::StencilOpState::builder()
        .fail_op(vd::StencilOp::Keep)
        .pass_op(vd::StencilOp::Keep)
        .depth_fail_op(vd::StencilOp::Keep)
        .compare_op(vd::CompareOp::Never)
        .compare_mask(0)
        .write_mask(0)
        .reference(0)
        .build();

    let depth_stencil = vd::PipelineDepthStencilStateCreateInfo::builder()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(vd::CompareOp::Less)
        .depth_bounds_test_enable(false)
        .stencil_test_enable(false)
        .front(stencil_op_state.clone())
        .back(stencil_op_state)
        .min_depth_bounds(0.0)
        .max_depth_bounds(1.0)
        .build();

    let color_blend_attachment = vd::PipelineColorBlendAttachmentState::builder()
        .blend_enable(false)
        .src_color_blend_factor(vd::BlendFactor::One)
        .dst_color_blend_factor(vd::BlendFactor::Zero)
        .color_blend_op(vd::BlendOp::Add)
        .src_alpha_blend_factor(vd::BlendFactor::One)
        .dst_alpha_blend_factor(vd::BlendFactor::Zero)
        .alpha_blend_op(vd::BlendOp::Add)
        .color_write_mask(
            vd::ColorComponentFlags::R
                | vd::ColorComponentFlags::G
                | vd::ColorComponentFlags::B
                | vd::ColorComponentFlags::A,
        ).build();

    let attachments = [color_blend_attachment];

    let color_blending = vd::PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .logic_op(vd::LogicOp::Copy)
        .attachments(&attachments)
        .blend_constants([0.0f32; 4])
        .build();

    let shader_stages = &[vert_shader_stage_info, frag_shader_stage_info];

    let pipeline = vd::GraphicsPipeline::builder()
        .stages(shader_stages)
        .vertex_input_state(&vertex_input_info)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterizer)
        .multisample_state(&multisampling)
        .depth_stencil_state(&depth_stencil)
        .color_blend_state(&color_blending)
        .layout(pipeline_layout)
        .render_pass(render_pass)
        .subpass(0)
        .base_pipeline_index(-1)
        .build(device)?;

    Ok(pipeline)
}

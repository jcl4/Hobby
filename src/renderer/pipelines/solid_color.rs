use crate::renderer::swapchain::SwapchainProperties;
use ash::{version::DeviceV1_0, vk};
use std::{ffi::CString, path::Path};

/// Solid Color pipeline
pub struct SolidColor {
    layout: vk::PipelineLayout,
}

impl SolidColor {
    pub fn new(device: &ash::Device, swapchain_properties: &SwapchainProperties) -> Self {
        let vertex_file = Path::new("resources/shaders/solid_color.frag.spv");
        let fragment_file = Path::new("resources/shaders/solid_color.frag.spv");

        let entry_point_name = CString::new("main").unwrap();

        let vert_module = super::create_shader_module(vertex_file, device);
        let vert_shader_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_module)
            .name(&entry_point_name)
            .build();

        let frag_module = super::create_shader_module(fragment_file, device);
        let frag_shader_state = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_module)
            .name(&entry_point_name)
            .build();

        let vetex_input_info = vk::PipelineVertexInputStateCreateInfo::builder().build();

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false)
            .build();

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: swapchain_properties.extent.width as _,
            height: swapchain_properties.extent.height as _,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let viewports = [viewport];
        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain_properties.extent,
        };
        let scissors = [scissor];
        let viewport_info = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors)
            .build();

        let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0)
            .build();

        let multisampling_info = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .build();

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::all())
            .blend_enable(false)
            .build();

        let color_blend_attachments = [color_blend_attachment];

        let color_blending_info = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .attachments(&color_blend_attachments)
            .build();

        let layout_info = vk::PipelineLayoutCreateInfo::builder().build();
        let layout = unsafe {
            device
                .create_pipeline_layout(&layout_info, None)
                .expect("Unable to create pipeline layout")
        };

        log::debug!("Solid Color Graphics Pipleline Built");

        SolidColor { layout }
    }

    pub fn cleanup(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline_layout(self.layout, None);
        }
    }
}

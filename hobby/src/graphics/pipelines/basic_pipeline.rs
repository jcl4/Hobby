use super::{pipeline, shader, Pipeline};
use crate::{
    core::{MaterialType, Vertex},
    graphics::VkVertex,
    Result,
};
use ash::{version::DeviceV1_0, vk};
use failure::bail;
use std::{ffi::CString, mem};

#[derive(Default)]
pub(crate) struct BasicPipeline {
    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct BasicVertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Pipeline for BasicPipeline {
    fn create_pipeline(
        &mut self,
        device: &ash::Device,
        swap_extent: vk::Extent2D,
        render_pass: vk::RenderPass,
    ) -> Result<()> {
        let material_type = MaterialType::Basic;
        let modules = shader::get_shader_modules(material_type, &device.clone())?;
        let vert_module = modules[0];
        let frag_module = modules[1];
        let shader_entry_name = CString::new("main").unwrap();

        let shader_stage_create_infos = [
            vk::PipelineShaderStageCreateInfo::builder()
                .name(&shader_entry_name)
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vert_module)
                .build(),
            vk::PipelineShaderStageCreateInfo::builder()
                .name(&shader_entry_name)
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(frag_module)
                .build(),
        ];

        let binding_descriptions = BasicVertex::get_binding_description();
        let attribute_descriptions = BasicVertex::get_attribute_descriptions();

        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions)
            .build();

        let (pipeline, pipeline_layout) = pipeline::create_graphics_pipeline(
            device,
            swap_extent,
            render_pass,
            &shader_stage_create_infos,
            vertex_input_info,
        )?;

        self.pipeline = pipeline;
        self.pipeline_layout = pipeline_layout;

        unsafe {
            device.destroy_shader_module(vert_module, None);
            device.destroy_shader_module(frag_module, None);
        }

        Ok(())
    }

    fn get_pipeline(&self) -> vk::Pipeline {
        self.pipeline
    }

    fn cleanup(&self, device: &ash::Device) -> Result<()> {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
        }
        Ok(())
    }
}

impl VkVertex for BasicVertex {
    fn get_binding_description() -> Vec<vk::VertexInputBindingDescription> {
        let input_rate = vk::VertexInputRate::VERTEX;
        let binding = vk::VertexInputBindingDescription::builder()
            .stride(mem::size_of::<Self>() as u32)
            .binding(0)
            .input_rate(input_rate)
            .build();

        vec![binding]
    }

    fn get_attribute_descriptions() -> Vec<vk::VertexInputAttributeDescription> {
        let pos_description = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(offset_of!(BasicVertex, position) as u32)
            .build();

        let color_description = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(offset_of!(BasicVertex, color) as u32)
            .build();

        vec![pos_description, color_description]
    }

    fn check_vertex(vertex: &Vertex) -> Result<()> {
        if vertex.color.is_none() {
            bail!("Basic Vertex requires a color component");
        }
        Ok(())
    }

    fn from_vertex(vertex: &Vertex) -> BasicVertex {
        BasicVertex {
            position: vertex.position,
            color: vertex.color.unwrap(),
        }
    }
}

// fn check_vertex()

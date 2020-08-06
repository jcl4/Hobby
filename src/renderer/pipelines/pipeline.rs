use std::io::Cursor;

use crate::scene::Material;
use crate::{renderer::pipelines::BasicVertex, Renderer};

struct PipelineData<'a> {
    vert_str: &'a str,
    vert_name: &'a str,
    frag_str: &'a str,
    frag_name: &'a str,
    vert_desc: wgpu::VertexBufferDescriptor<'a>,

}

pub fn create_render_pipeline(
    material: &Material,
    device: &wgpu::Device,
    sc_desc: &wgpu::SwapChainDescriptor,
) -> wgpu::RenderPipeline {
    let pipeline_data = match material {
        Material::Basic => {
            log::info!("Building Basic Material Pipeline");

            PipelineData{
                vert_str: include_str!("shaders/basic.vert"),
                vert_name: "basic.vert",
                frag_str: include_str!("shaders/basic.frag"),
                frag_name: "basic.frag",
                vert_desc: BasicVertex::desc(),
            }
        }
    };

    let mut compiler = shaderc::Compiler::new().expect("Unable to get shaderc compiler");
    let vert_spv = compiler.compile_into_spirv(pipeline_data.vert_str, shaderc::ShaderKind::Vertex, pipeline_data.vert_name, "main", None).expect("unable to compile vertex shader");
    let frag_spv = compiler.compile_into_spirv(pipeline_data.frag_str, shaderc::ShaderKind::Fragment, pipeline_data.frag_name, "main", None).expect("unale to compile fragment shader");

    let vert_data =
        wgpu::read_spirv(Cursor::new(vert_spv.as_binary_u8())).expect("Unable to read Vertex SPIRV data");
    let frag_data =
        wgpu::read_spirv(Cursor::new(frag_spv.as_binary_u8())).expect("Unable to read Fragtment SPRIV data");

    let vert_module = device.create_shader_module(&vert_data);
    let frag_module = device.create_shader_module(&frag_data);

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[],
    });

    let pipeline_desc = wgpu::RenderPipelineDescriptor {
        layout: &render_pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vert_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &frag_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        color_states: &[wgpu::ColorStateDescriptor {
            format: sc_desc.format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[pipeline_data.vert_desc],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    };

    device.create_render_pipeline(&pipeline_desc)
}

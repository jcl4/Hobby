use std::io::Cursor;

use crate::model::Material;
use crate::Renderer;

pub fn create_render_pipeline(material: &Material, renderer: &Renderer) -> wgpu::RenderPipeline {
    let (vert_spv, frag_spv) = match material {
        Material::Basic => {
            let vert_spv = include_bytes!("shaders/basic.vert.spv");
            let frag_spv = include_bytes!("shaders/basic.frag.spv");
            (vert_spv, frag_spv)
        }
    };

    let vert_data =
        wgpu::read_spirv(Cursor::new(&vert_spv[..])).expect("Unable to read Vertex SPIRV data");
    let frag_data =
        wgpu::read_spirv(Cursor::new(&frag_spv[..])).expect("Unable to read Fragtment SPRIV data");

    let vert_module = renderer.device.create_shader_module(&vert_data);
    let frag_module = renderer.device.create_shader_module(&frag_data);

    let render_pipeline_layout =
        renderer
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
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
            format: renderer.sc_desc.format,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    };

    renderer.device.create_render_pipeline(&pipeline_desc)
}

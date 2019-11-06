use log::info;
use std::fs::File;

pub struct ColoredMeshVertex {
    pos: [f32; 4],
    color: [f32; 4],
}

impl ColoredMeshVertex {
    pub fn new(pos: [f32; 3], color: [f32; 4]) -> ColoredMeshVertex {
        let pos = [pos[0], pos[1], pos[2], 1.0];
        ColoredMeshVertex { pos, color }
    }
}

pub struct ColoredMeshModel {
    vertices: Vec<ColoredMeshVertex>,
    indices: Vec<u16>,
}

impl ColoredMeshModel {
    pub fn new(vertices: Vec<ColoredMeshVertex>, indices: Vec<u16>) -> ColoredMeshModel {
        ColoredMeshModel { vertices, indices }
    }
}

pub struct ColoredMeshPipeline {
    pipeline: Option<wgpu::RenderPipeline>,
    bind_group: Option<wgpu::BindGroup>,
    models: Vec<ColoredMeshModel>,
}

impl ColoredMeshPipeline {
    pub fn new() -> ColoredMeshPipeline {
        let models: Vec<ColoredMeshModel> = vec![];

        ColoredMeshPipeline {
            pipeline: None,
            bind_group: None,
            models,
        }
    }

    pub(crate) fn build(&mut self, device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) {
        let vert_file = File::open("src/renderer/shaders/colored_mesh.vert.spv")
            .expect("Unable to find vertex shader spirv file for colored_mesh pipeline");
        let frag_file = File::open("src/renderer/shaders/colored_mesh.frag.spv")
            .expect("Unable to find fragment shader spirv file for colored_mesh pipeline");

        let vert_spv = wgpu::read_spirv(vert_file)
            .expect("Unable to create spirv data from vertex shader for colored_mesh pipeline");
        let vert_mod = device.create_shader_module(&vert_spv);
        let frag_spv = wgpu::read_spirv(frag_file)
            .expect("Unable to create spirv data from fragment shader for colored_mesh pipeline");
        let frag_mod = device.create_shader_module(&frag_spv);

        // let pipeline_desc = wgpu::RenderPipelineDescriptor

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { bindings: &[] });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let vertex_size = std::mem::size_of::<ColoredMeshVertex>();

        let pipeline_desc = wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vert_mod,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &frag_mod,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: vertex_size as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Float4,
                        offset: 0,
                        shader_location: 0,
                    },
                    wgpu::VertexAttributeDescriptor {
                        format: wgpu::VertexFormat::Float4,
                        offset: 4 * 4,
                        shader_location: 1,
                    },
                ],
            }],
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let pipeline = device.create_render_pipeline(&pipeline_desc);

        self.pipeline = Some(pipeline);
        self.bind_group = Some(bind_group);
        info!("Colored Mesh Pipeline Built");
    }

    pub fn add_model(&mut self, model: ColoredMeshModel) {
        self.models.push(model);
    }

    pub fn add_models(&mut self, mut models: Vec<ColoredMeshModel>) {
        self.models.append(&mut models);
    }
}

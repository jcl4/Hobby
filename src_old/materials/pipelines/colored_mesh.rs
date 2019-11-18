use crate::{
    math::matrix4::Matrix4,
    renderer::pipelines::{Pipelines, RenderObject},
};
// use log::info;
use std::{fs::File, mem, time::Instant};

#[derive(Clone, Copy)]
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
    transform: Matrix4,
    vertices: Vec<ColoredMeshVertex>,
    indices: Vec<u16>,
    vertex_buf: Option<wgpu::Buffer>,
    index_buf: Option<wgpu::Buffer>,
    bind_group: Option<wgpu::BindGroup>,
    uniform_buf: Option<wgpu::Buffer>,
}

impl ColoredMeshModel {
    pub fn new(
        vertices: Vec<ColoredMeshVertex>,
        indices: Vec<u16>,
        transform: Matrix4,
    ) -> ColoredMeshModel {
        ColoredMeshModel {
            transform,
            vertices,
            indices,
            vertex_buf: None,
            index_buf: None,
            bind_group: None,
            uniform_buf: None,
        }
    }
}

impl RenderObject for ColoredMeshModel {
    fn get_pipeline(&self) -> Pipelines {
        Pipelines::ColoredMesh
    }

    fn update_uniform_buffer(&mut self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {

        let temp_buf = device
            .create_buffer_mapped(16, wgpu::BufferUsage::COPY_SRC)
            .fill_from_slice(&self.transform.data);

        encoder.copy_buffer_to_buffer(&temp_buf, 0, &self.uniform_buf.unwrap(), 0, 64);
    }

    fn draw(&mut self, render_pass: &mut wgpu::RenderPass) {

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group.unwrap(), &[]);
        render_pass.set_vertex_buffers(0, &[(&self.vertex_buf, 0)]);
        render_pass.set_index_buffer(&self.index_buf, 0);
        render_pass.draw_indexed(0..self.index_count as u32, 0, 0..1)
        
    }

    fn build_buffers(&mut self, device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) {
        self.vertex_buf = Some(
            device
                .create_buffer_mapped(self.vertices.len(), wgpu::BufferUsage::VERTEX)
                .fill_from_slice(&self.vertices),
        );

        self.index_buf = Some(
            device
                .create_buffer_mapped(self.indices.len(), wgpu::BufferUsage::INDEX)
                .fill_from_slice(&self.indices),
        );

        let uniform_size = mem::size_of::<Matrix4>() as wgpu::BufferAddress;

        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            size: uniform_size,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        self.bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &uniform_buf,
                    range: 0..uniform_size,
                },
            }],
        }));

        self.uniform_buf = Some(uniform_buf);
    }
}

pub struct ColoredMeshPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl ColoredMeshPipeline {
    pub fn new(
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
        model: ColoredMeshModel,
    ) -> ColoredMeshPipeline {
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

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            }],
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

        ColoredMeshPipeline {
            pipeline,
            bind_group_layout,
        }
    }


}
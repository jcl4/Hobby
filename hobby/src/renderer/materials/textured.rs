use crate::core::{Mesh, Texture, Vertex};
use crate::renderer::Renderer;
use crate::Result;
use failure::bail;
use log::info;
use std::sync::Arc;
use vulkano::buffer::immutable::ImmutableBuffer;
use vulkano::buffer::{BufferAccess, BufferUsage, CpuBufferPool, TypedBufferAccess};
use vulkano::descriptor::descriptor_set::{FixedSizeDescriptorSetsPool, PersistentDescriptorSet};
use vulkano::descriptor::DescriptorSet;
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::image::{ImageViewAccess, ImmutableImage};
use vulkano::impl_vertex;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::sampler::Sampler;
use vulkano::sync::GpuFuture;

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/renderer/materials/shaders/textured.vert"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/renderer/materials/shaders/textured.frag"
    }
}

pub struct TexturedPipeline {
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    sets_pool: FixedSizeDescriptorSetsPool<Arc<GraphicsPipelineAbstract + Send + Sync>>,
    mvp_buffer_pool: CpuBufferPool<vs::ty::MVP>,
    texture_set: Arc<DescriptorSet + Send + Sync>,
}

impl TexturedPipeline {
    pub fn new(renderer: &Renderer, texture: &Texture) -> Result<TexturedPipeline> {
        let pipeline = build_pipeline(
            &renderer.device,
            renderer.swapchain.dimensions(),
            &renderer.render_pass,
        )?;

        let sets_pool = FixedSizeDescriptorSetsPool::new(pipeline.clone(), 0);
        let mvp_buffer_pool = CpuBufferPool::uniform_buffer(renderer.device.clone());

        let (texture, sampler) = texture.get_texture_and_sampler();

        let texture_set = Arc::new(
            PersistentDescriptorSet::start(pipeline.clone(), 1)
                .add_sampled_image(texture, sampler)?
                .build()?,
        );

        Ok(TexturedPipeline {
            pipeline,
            sets_pool,
            mvp_buffer_pool,
            texture_set,
        })
    }
}

#[derive(Debug, Clone)]
struct TexturedVertex {
    position: [f32; 3],
    tex_coord: [f32; 2],
}
impl_vertex!(TexturedVertex, position, tex_coord);

impl TexturedVertex {
    fn from_vertex(vertex: &Vertex) -> TexturedVertex {
        TexturedVertex {
            position: vertex.position.clone(),
            tex_coord: vertex.tex_coord.clone().unwrap(),
        }
    }
}

fn check_mesh(mesh: &Mesh) -> Result<()> {
    let vertex = &mesh.vertices[0];

    if vertex.tex_coord.is_none() {
        bail!("Textured Materials need texture coordinates defined in the vertex data");
    }

    Ok(())
}

fn build_buffers(
    mesh: &Mesh,
    graphics_queue: &Arc<Queue>,
) -> Result<(
    Arc<BufferAccess + Send + Sync>,
    Arc<TypedBufferAccess<Content = [u32]> + Send + Sync>,
)> {
    let vertices: Vec<TexturedVertex> = mesh
        .vertices
        .iter()
        .map(|vertex| TexturedVertex::from_vertex(vertex))
        .collect();

    let (vertex_buffer, vertex_future) = ImmutableBuffer::from_iter(
        vertices.iter().cloned(),
        BufferUsage::vertex_buffer(),
        graphics_queue.clone(),
    )?;

    let (index_buffer, index_future) = ImmutableBuffer::from_iter(
        mesh.indices.iter().cloned(),
        BufferUsage::index_buffer(),
        graphics_queue.clone(),
    )?;

    index_future.join(vertex_future).flush()?;

    Ok((vertex_buffer, index_buffer))
}

fn build_pipeline(
    device: &Arc<Device>,
    swapchain_extent: [u32; 2],
    render_pass: &Arc<RenderPassAbstract + Send + Sync>,
) -> Result<Arc<GraphicsPipelineAbstract + Send + Sync>> {
    let vert_shader_module = vs::Shader::load(device.clone())?;
    let frag_shader_module = fs::Shader::load(device.clone())?;

    let dimensions = [swapchain_extent[0] as f32, swapchain_extent[1] as f32];
    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions,
        depth_range: 0.0..1.0,
    };

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<TexturedVertex>()
            .vertex_shader(vert_shader_module.main_entry_point(), ())
            .triangle_list()
            .primitive_restart(false)
            .viewports(vec![viewport])
            .fragment_shader(frag_shader_module.main_entry_point(), ())
            .depth_clamp(false)
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    Ok(pipeline)
}

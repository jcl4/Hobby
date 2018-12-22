use crate::core::{Mesh, Model, Vertex};
use crate::renderer::materials::ModelPipeline;
use crate::renderer::Renderer;
use crate::Result;
use failure::bail;
use log::info;
use std::sync::Arc;
use vulkano::buffer::immutable::ImmutableBuffer;
use vulkano::buffer::{BufferAccess, BufferUsage, TypedBufferAccess};
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::impl_vertex;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::sync::GpuFuture;

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/renderer/materials/shaders/basic.vert"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/renderer/materials/shaders/basic.frag"
    }
}

pub struct BasicPipeline {
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
}

impl BasicPipeline {
    pub fn new(renderer: &Renderer) -> Result<BasicPipeline> {
        let pipeline = build_pipline(
            &renderer.device,
            renderer.swapchain.dimensions(),
            &renderer.render_pass,
        )?;

        Ok(BasicPipeline { pipeline })
    }
}

impl ModelPipeline for BasicPipeline {
    fn build_model(&mut self, model: &mut Model, renderer: &Renderer) -> Result<()> {
        check_mesh(&model.mesh)?;
        let (vertex_buffer, index_buffer) = build_buffers(&model.mesh, &renderer.graphics_queue)?;
        model.mesh.set_index_buffer(index_buffer);
        model.mesh.set_vertex_buffer(vertex_buffer);

        Ok(())
    }

    fn graphics_pipeline(&self) -> Arc<GraphicsPipelineAbstract + Send + Sync>{
        self.pipeline.clone()
    }
}
 

#[derive(Debug, Clone)]
struct BasicVertex {
    position: [f32; 3],
    color: [f32; 4],
}
impl_vertex!(BasicVertex, position, color);

impl BasicVertex {
    fn from_vertex(vertex: &Vertex) -> BasicVertex {
        BasicVertex {
            position: vertex.position.clone(),
            color: vertex.color.clone().unwrap(),
        }
    }
}

fn check_mesh(mesh: &Mesh) -> Result<()> {
    let vertex = &mesh.vertices[0];

    if vertex.color.is_none() {
        bail!("Basic Materials need Colors defined in the vertex data");
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
    let vertices: Vec<BasicVertex> = mesh
        .vertices
        .iter()
        .map(|vertex| BasicVertex::from_vertex(vertex))
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

fn build_pipline(
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

    let grapics_pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<BasicVertex>()
            .vertex_shader(vert_shader_module.main_entry_point(), ())
            .triangle_list()
            .primitive_restart(false)
            .viewports(vec![viewport]) // NOTE: also sets scissor to cover whole viewport
            .fragment_shader(frag_shader_module.main_entry_point(), ())
            .depth_clamp(false)
            // NOTE: there's an outcommented .rasterizer_discard() in Vulkano...
            .polygon_mode_fill() // = default
            .line_width(1.0) // = default
            .cull_mode_back()
            .front_face_clockwise()
            // NOTE: no depth_bias here, but on pipeline::raster::Rasterization
            .blend_pass_through() // = default
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    info!("Graphics Pipeline Created!");

    Ok(grapics_pipeline)
}

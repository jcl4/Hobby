mod colored_mesh;

pub use colored_mesh::{ColoredMeshModel, ColoredMeshPipeline, ColoredMeshVertex};

enum Pipelines {
    ColoredMesh,
    Textured,
}

pub trait Updatable {
    fn update(&mut self) {}
}

pub trait RenderObject {
    fn get_pipeline(&self) -> Pipelines;

    fn draw(&mut self, render_pass: &mut wgpu::RenderPass);

    fn update_uniform_buffer(&mut self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder);

    fn build_buffers(
        &mut self,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
    );
}

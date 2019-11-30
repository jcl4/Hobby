use crate::{core::MaterialType, math::Transform, scene::Mesh, Application};

/// Object Builder
pub struct ObjectBuilder {
    mesh: Mesh,
    transform: Transform,
    material: MaterialType,
}

impl ObjectBuilder {
    pub fn new() -> Self {
        ObjectBuilder {
            mesh: Mesh::default(),
            transform: Transform::default(),
            material: MaterialType::SolidColor,
        }
    }

    #[allow(unused_variables)]
    pub fn with_mesh(mut self, mesh: Mesh) -> Self {
        self.mesh = mesh;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_material(mut self, material: MaterialType) -> Self {
        self.material = material;
        self
    }

    #[allow(unused_variables)]
    pub fn build(self, app: &mut Application) -> Object {
        let buffer_group = app
            .renderer
            .get_object_buffer_group(&self.mesh, self.material);

        Object {
            mesh: self.mesh,
            transform: self.transform,
            material: self.material,
            buffer_group,
        }
    }
}

/// Object
#[derive(Debug)]
pub struct Object {
    mesh: Mesh,
    transform: Transform,
    material: MaterialType,
    buffer_group: ObjectBufferGroup,
}

impl Object {
    pub fn update(&mut self) {}

    pub fn draw(&self) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub(crate) struct ObjectBufferGroup {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

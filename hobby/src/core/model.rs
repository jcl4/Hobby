use crate::Result;
use crate::{
    core::{MaterialType, Mesh, Transform},
    graphics::{
        pipelines::{mvp, BasicPipeline, Mvp, MvpBuffers, Pipeline},
        Renderer,
    },
    na,
};
use ash::{version::DeviceV1_0, vk};
use log::debug;

pub struct Model {
    pub mesh: Mesh,
    pub material_type: MaterialType,
    pipeline: Option<Box<dyn Pipeline>>,
    pub transform: Transform,
    // Update function closure called in model update inputs: Transform: Model transform, f32: Update Time in ms, bool: debug update
    model_update: Box<dyn FnMut(Transform, f32, bool) -> Transform>,
    mvp_buffers: MvpBuffers,
    mvp_layout: vk::DescriptorSetLayout,
    mvp_sets: Vec<vk::DescriptorSet>,
}

impl Model {
    pub fn new(mesh: Mesh, material_type: MaterialType) -> Model {
        let transform = Transform::default();
        let model_update = Box::new(|transform, _dt, _debug| transform);
        let mvp_buffers = MvpBuffers::default();
        let mvp_sets = vec![];
        let mvp_layout = vk::DescriptorSetLayout::null();

        Model {
            mesh,
            material_type,
            pipeline: None,
            transform,
            model_update,
            mvp_buffers,
            mvp_layout,
            mvp_sets,
        }
    }

    pub(crate) fn build(&mut self, renderer: &Renderer) -> Result<()> {
        let mut pipeline = match self.material_type {
            MaterialType::Basic => BasicPipeline::default(),
        };

        self.mvp_layout = mvp::create_mvp_layout(&renderer.device, 0)?;
        pipeline.create_pipeline(
            &renderer.device,
            renderer.swapchain_data.extent,
            renderer.render_pass,
            self.mvp_layout,
        )?;

        self.mvp_buffers.build(renderer)?;

        let mvp_sets = create_descriptor_sets(
            renderer.swapchain_data.image_views.len() as u32,
            &renderer.device,
            renderer.ubo_pool,
            self.mvp_layout,
            &self.mvp_buffers,
        )?;

        self.mesh.build_mesh(renderer, &self.material_type)?;

        self.pipeline = Some(Box::new(pipeline));
        self.mvp_sets = mvp_sets;

        Ok(())
    }

    pub(crate) fn draw(
        &self,
        cb: vk::CommandBuffer,
        device: &ash::Device,
        index: usize,
    ) -> Result<()> {
        unsafe {
            device.cmd_bind_pipeline(
                cb,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.as_ref().unwrap().get_pipeline(),
            );
            let descriptor_set = [self.mvp_sets[index]];
            device.cmd_bind_descriptor_sets(
                cb,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.as_ref().unwrap().get_layout(),
                0,
                &descriptor_set,
                &[],
            );
        }

        self.mesh.draw(cb);

        Ok(())
    }

    pub(crate) fn update(&mut self, dt: f32, debug_display: bool) {
        self.transform = (self.model_update)(self.transform.clone(), dt, debug_display);
    }

    pub(crate) fn update_mvp(
        &mut self,
        view: na::Matrix4<f32>,
        proj: na::Matrix4<f32>,
        current_image: usize,
        device: &ash::Device,
    ) -> Result<()> {
        let ubos = [Mvp {
            model: self.transform.get_model_matrix(),
            view,
            proj,
        }];

        let buffer_size = std::mem::size_of::<Mvp>() as u64;

        unsafe {
            let data_ptr = device.map_memory(
                self.mvp_buffers.buffers_memory[current_image],
                0,
                buffer_size,
                vk::MemoryMapFlags::empty(),
            )?;

            let mut align =
                ash::util::Align::new(data_ptr, std::mem::align_of::<Mvp>() as u64, buffer_size);
            align.copy_from_slice(&ubos);
            device.unmap_memory(self.mvp_buffers.buffers_memory[current_image]);
        }
        Ok(())
    }

    pub(crate) fn cleanup(&self, device: &ash::Device) -> Result<()> {
        unsafe {
            device.destroy_descriptor_set_layout(self.mvp_layout, None);
            debug!("Layout Destroyed");
        }
        self.pipeline.as_ref().unwrap().cleanup(device)?;
        self.mvp_buffers.cleanup(device);
        self.mesh.cleanup();
        Ok(())
    }
}

fn create_descriptor_sets(
    num_sets: u32,
    device: &ash::Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
    mvp_buffers: &MvpBuffers,
) -> Result<Vec<vk::DescriptorSet>> {
    let range = 0..num_sets;

    let layouts: Vec<vk::DescriptorSetLayout> = range.clone().map(|_i| layout).collect();

    let allocate_info = vk::DescriptorSetAllocateInfo::builder()
        .set_layouts(&layouts)
        .descriptor_pool(pool);

    let sets;
    unsafe {
        sets = device.allocate_descriptor_sets(&allocate_info)?;
    }

    for i in range {
        let buffer_info = vk::DescriptorBufferInfo::builder()
            .range(std::mem::size_of::<Mvp>() as u64)
            .offset(0)
            .buffer(mvp_buffers.buffers[i as usize])
            .build();

        let buffer_infos = [buffer_info];

        let descriptor_write = vk::WriteDescriptorSet::builder()
            .buffer_info(&buffer_infos)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .dst_array_element(0)
            .dst_binding(0)
            .dst_set(sets[i as usize])
            .build();

        let descriptor_writes = [descriptor_write];

        unsafe {
            device.update_descriptor_sets(&descriptor_writes, &[]);
        }
    }

    Ok(sets)
}

//     pub fn add_update_fn(&mut self, f: Box<dyn FnMut(Transform, f32, bool) -> Transform>) {
//         self.model_update = f;
//     }

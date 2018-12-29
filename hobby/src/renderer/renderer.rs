use std::sync::Arc;
use winit::{EventsLoop, Window};

use crate::glm;
use crate::na;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::descriptor::descriptor_set::StdDescriptorPool;
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{FramebufferAbstract, RenderPassAbstract};
use vulkano::image::swapchain::SwapchainImage;
use vulkano::instance::debug::DebugCallback;
use vulkano::instance::Instance;
use vulkano::swapchain::{acquire_next_image, AcquireError, Surface, Swapchain};
use vulkano::sync;
use vulkano::sync::GpuFuture;

use crate::core::Model;
use crate::renderer::base;
use crate::renderer::render_pass;
use crate::renderer::swapchain;
use crate::{HobbySettings, Result};

pub struct Renderer {
    instance: Arc<Instance>,
    _debug_callback: Option<DebugCallback>,

    surface: Arc<Surface<Window>>,
    physical_device_index: usize,

    pub device: Arc<Device>,
    pub graphics_queue: Arc<Queue>,
    present_queue: Arc<Queue>,

    pub swapchain: Arc<Swapchain<Window>>,
    swapchain_images: Vec<Arc<SwapchainImage<Window>>>,

    pub render_pass: Arc<RenderPassAbstract + Send + Sync>,
    framebuffer: Vec<Arc<FramebufferAbstract + Send + Sync>>,

    previous_frame_end: Option<Box<GpuFuture>>,

    pub uniform_pool: StdDescriptorPool,

    recreate_swapchain: bool,

    projection: na::Perspective3<f32>,
}

impl Renderer {
    pub fn new(hobby_settings: &HobbySettings, events_loop: &EventsLoop) -> Result<Renderer> {
        let instance = base::create_instance(&hobby_settings.app_info)?;
        let debug_callback = base::setup_debug_callback(&instance);
        let surface =
            base::create_surface(events_loop, &hobby_settings.window_settings, &instance)?;
        let physical_device_index = base::pick_physical_device(&instance, &surface)?;

        let (device, graphics_queue, present_queue) =
            base::create_logical_device(&instance, &surface, physical_device_index)?;

        let (swapchain, swapchain_images) = swapchain::create_swapchain(
            &instance,
            &surface,
            physical_device_index,
            &device,
            &graphics_queue,
            &present_queue,
            None,
        )?;

        let render_pass = render_pass::create_render_pass(&device, swapchain.format());
        let framebuffer = swapchain::create_framebuffers(&swapchain_images, &render_pass);

        let previous_frame_end = Some(create_sync_objects(&device));
        let uniform_pool = StdDescriptorPool::new(device.clone());

        let aspect_ratio = swapchain.dimensions()[0] as f32 / swapchain.dimensions()[1] as f32;
        let fovy = glm::quarter_pi();
        let projection = na::Perspective3::new(aspect_ratio, fovy, 0.1, 10.0);

        Ok(Renderer {
            instance,
            _debug_callback: debug_callback,
            surface,
            physical_device_index,
            device,
            graphics_queue,
            present_queue,
            swapchain,
            swapchain_images,
            render_pass,
            framebuffer,
            previous_frame_end,
            uniform_pool,
            recreate_swapchain: false,
            projection,
        })
    }

    pub fn draw_frame(&mut self, models: &mut Vec<Model>, view: &na::Isometry3<f32>) -> Result<()> {
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

        if self.recreate_swapchain {
            self.recreate_swapchain(models)?;
            self.recreate_swapchain = false;
        }

        let (image_index, acquire_future) = match acquire_next_image(self.swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                self.recreate_swapchain = true;
                return Ok(());
            }
            Err(err) => panic!("{:?}", err),
        };

        let mut cb = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.graphics_queue.family(),
        )?;

        let clear_values = vec![[0.1, 0.2, 0.1, 1.0].into()];
        cb = cb.begin_render_pass(self.framebuffer[image_index].clone(), false, clear_values)?;

        for model in models {
            cb = model.draw(
                cb,
                view.to_homogeneous().into(),
                self.projection.to_homogeneous().into(),
            )?;
        }
        cb = cb.end_render_pass()?;
        let command_buffer = cb.build()?;

        let future = self
            .previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(self.graphics_queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                self.present_queue.clone(),
                self.swapchain.clone(),
                image_index,
            )
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                self.previous_frame_end = Some(Box::new(future) as Box<_>);
            }
            Err(vulkano::sync::FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end =
                    Some(Box::new(vulkano::sync::now(self.device.clone())) as Box<_>);
            }
            Err(e) => {
                println!("{:?}", e);
                self.previous_frame_end =
                    Some(Box::new(vulkano::sync::now(self.device.clone())) as Box<_>);
            }
        }

        Ok(())
    }

    fn recreate_swapchain(&mut self, models: &mut Vec<Model>) -> Result<()> {
        let (swapchain, swapchain_images) = swapchain::create_swapchain(
            &self.instance,
            &self.surface,
            self.physical_device_index,
            &self.device,
            &self.graphics_queue,
            &self.present_queue,
            Some(self.swapchain.clone()),
        )?;

        self.swapchain = swapchain;
        self.swapchain_images = swapchain_images;

        let render_pass = render_pass::create_render_pass(&self.device, self.swapchain.format());
        let framebuffer = swapchain::create_framebuffers(&self.swapchain_images, &render_pass);

        self.render_pass = render_pass;
        self.framebuffer = framebuffer;

        for model in models {
            model.build(&self)?;
        }
        Ok(())
    }
}

fn create_sync_objects(device: &Arc<Device>) -> Box<GpuFuture> {
    Box::new(sync::now(device.clone())) as Box<GpuFuture>
}

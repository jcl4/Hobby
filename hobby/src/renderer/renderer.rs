use super::{
    base, base::QueueData, command_buffer, pipeline, render_pass, swapchain,
    swapchain::SwapchainData,
};
use crate::{HobbySettings, Result};
use ash::{
    extensions::{ext::DebugReport, khr::Surface},
    version::{DeviceV1_0, InstanceV1_0},
    vk,
};
use failure::bail;
use log::debug;
use winit::{EventsLoop, Window};

const FRAMES_IN_FLIGHT: u32 = 2;

pub struct Renderer {
    _window: Window,
    _entry: ash::Entry,
    instance: ash::Instance,
    surface: vk::SurfaceKHR,
    surface_loader: Surface,
    debug_callback: vk::DebugReportCallbackEXT,
    debug_loader: DebugReport,
    physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    queue_data: QueueData,
    swapchain_data: SwapchainData,
    render_pass: vk::RenderPass,
    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffers: Vec<vk::CommandBuffer>,
    img_available_semaphores: Vec<vk::Semaphore>,
    render_finished_sempahores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    current_frame: u32,
    pub is_resized: bool,
}

impl Renderer {
    pub fn new(hobby_settings: &HobbySettings, events_loop: &EventsLoop) -> Result<Renderer> {
        let entry = ash::Entry::new().unwrap();

        let window = base::create_window(&events_loop, &hobby_settings.window_settings)?;
        let instance = base::create_instance(&hobby_settings.app_info, &entry)?;
        let surface = unsafe { base::create_surface(&entry, &instance, &window)? };
        let surface_loader = Surface::new(&entry, &instance);
        let (debug_callback, debug_loader) = base::setup_debug_callback(&entry, &instance)?;
        let physical_device = unsafe { instance.enumerate_physical_devices()?.remove(0) };
        let (device, queue_data) =
            base::create_device_and_queues(&instance, &physical_device, &surface_loader, &surface)?;
        let swapchain_data = swapchain::create_swapchain_and_image_views(
            surface_loader.clone(),
            physical_device,
            surface,
            &instance,
            &device,
        )?;

        let render_pass =
            render_pass::create_render_pass(swapchain_data.surface_format.format, device.clone())?;

        let (pipeline, pipeline_layout) = pipeline::create_graphics_pipeline(
            device.clone(),
            &swapchain_data.extent,
            render_pass.clone(),
        )?;

        let framebuffers = base::create_framebuffers(&swapchain_data, render_pass, &device)?;
        let command_pool = command_buffer::create_command_pool(&queue_data, &device)?;
        let command_buffers = command_buffer::create_command_buffers(
            command_pool,
            swapchain_data.image_views.len() as u32,
            render_pass,
            pipeline,
            &swapchain_data,
            &framebuffers,
            &device,
        )?;

        let (img_available_semaphores, render_finished_sempahores, in_flight_fences) =
            create_sync_objects(&device)?;

        Ok(Renderer {
            _window: window,
            _entry: entry,
            instance,
            surface,
            surface_loader,
            debug_callback,
            debug_loader,
            physical_device,
            device,
            queue_data,
            swapchain_data,
            render_pass,
            pipeline,
            pipeline_layout,
            framebuffers,
            command_pool,
            command_buffers,
            img_available_semaphores,
            render_finished_sempahores,
            in_flight_fences,
            current_frame: 0,
            is_resized: false,
        })
    }

    pub fn draw_frame(&mut self) -> Result<()> {
        unsafe {
            self.device.wait_for_fences(
                &[self.in_flight_fences[self.current_frame as usize]],
                true,
                std::u64::MAX,
            )?;

            let (img_index, _) = {
                let result = self.swapchain_data.swapchain_loader.acquire_next_image(
                    self.swapchain_data.swapchain,
                    std::u64::MAX,
                    self.img_available_semaphores[self.current_frame as usize],
                    vk::Fence::null(),
                );
                match result {
                    Ok(img_index) => img_index,
                    Err(vk_result) => match vk_result {
                        vk::Result::ERROR_OUT_OF_DATE_KHR => {
                            self.recreate_swapchain()?;
                            return Ok(());
                        }
                        _ => bail!("Failed to aquire swapchain image!"),
                    },
                }
            };

            let wait_semaphores = [self.img_available_semaphores[self.current_frame as usize]];
            let signal_semaphores = [self.render_finished_sempahores[self.current_frame as usize]];
            let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

            let submit_info = vk::SubmitInfo::builder()
                .signal_semaphores(&signal_semaphores)
                .command_buffers(&[self.command_buffers[img_index as usize]])
                .wait_dst_stage_mask(&wait_stages)
                .wait_semaphores(&wait_semaphores)
                .build();

            self.device
                .reset_fences(&[self.in_flight_fences[self.current_frame as usize]])?;

            self.device.queue_submit(
                self.queue_data.graphics_queue,
                &[submit_info],
                self.in_flight_fences[self.current_frame as usize],
            )?;

            let indices = [img_index];
            let swapchains = [self.swapchain_data.swapchain];

            let present_info = vk::PresentInfoKHR::builder()
                .image_indices(&indices)
                .swapchains(&swapchains)
                .wait_semaphores(&signal_semaphores);

            let result = self
                .swapchain_data
                .swapchain_loader
                .queue_present(self.queue_data.present_queue, &present_info);

            let resized = match result {
                Ok(_) => self.is_resized,
                Err(vk_result) => match vk_result {
                    vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR => true,
                    _ => bail!("Failed to execute queue present!"),
                },
            };

            if resized {
                self.is_resized = false;
                self.recreate_swapchain()?;
            }
        }

        self.current_frame = (self.current_frame + 1) % FRAMES_IN_FLIGHT;

        Ok(())
    }

    fn clean_up_swapchian(&self) {
        debug!("Cleanup Swapchain Called");
        unsafe {
            for frame_buffer in self.framebuffers.iter() {
                self.device.destroy_framebuffer(*frame_buffer, None);
            }

            self.device
                .free_command_buffers(self.command_pool, &self.command_buffers);

            self.device.destroy_pipeline(self.pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);

            for img_view in self.swapchain_data.image_views.iter() {
                self.device.destroy_image_view(*img_view, None);
            }

            self.swapchain_data
                .swapchain_loader
                .destroy_swapchain(self.swapchain_data.swapchain, None);
        }
    }

    fn recreate_swapchain(&mut self) -> Result<()> {
        debug!("Recreate Swapchain Called");

        unsafe {
            self.device.device_wait_idle()?;
            self.clean_up_swapchian();
        }

        debug!("Recreating Swapchain");
        self.swapchain_data = swapchain::create_swapchain_and_image_views(
            self.surface_loader.clone(),
            self.physical_device,
            self.surface,
            &self.instance,
            &self.device,
        )?;

        self.render_pass = render_pass::create_render_pass(
            self.swapchain_data.surface_format.format,
            self.device.clone(),
        )?;

        let pipeline_data = pipeline::create_graphics_pipeline(
            self.device.clone(),
            &self.swapchain_data.extent,
            self.render_pass.clone(),
        )?;

        self.pipeline = pipeline_data.0;
        self.pipeline_layout = pipeline_data.1;

        self.framebuffers =
            base::create_framebuffers(&self.swapchain_data, self.render_pass, &self.device)?;

        self.command_buffers = command_buffer::create_command_buffers(
            self.command_pool,
            self.swapchain_data.image_views.len() as u32,
            self.render_pass,
            self.pipeline,
            &self.swapchain_data,
            &self.framebuffers,
            &self.device,
        )?;

        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.clean_up_swapchian();

            for i in 0..FRAMES_IN_FLIGHT {
                self.device
                    .destroy_semaphore(self.render_finished_sempahores[i as usize], None);
                self.device
                    .destroy_semaphore(self.img_available_semaphores[i as usize], None);
                self.device
                    .destroy_fence(self.in_flight_fences[i as usize], None);
            }
            self.device.destroy_command_pool(self.command_pool, None);

            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);

            self.debug_loader
                .destroy_debug_report_callback(self.debug_callback, None);

            self.instance.destroy_instance(None);
        }
    }
}

fn create_sync_objects(
    device: &ash::Device,
) -> Result<(Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>)> {
    let semaphore_create_info = vk::SemaphoreCreateInfo::default();
    let fence_create_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    let mut img_available_semaphores = vec![];
    let mut render_finished_semaphores = vec![];
    let mut in_flight_fences = vec![];

    unsafe {
        for _i in 0..FRAMES_IN_FLIGHT {
            img_available_semaphores.push(device.create_semaphore(&semaphore_create_info, None)?);
            render_finished_semaphores.push(device.create_semaphore(&semaphore_create_info, None)?);
            in_flight_fences.push(device.create_fence(&fence_create_info, None)?);
        }
    }

    Ok((
        img_available_semaphores,
        render_finished_semaphores,
        in_flight_fences,
    ))
}

// fn create_projection(extent: &Extent2D) -> na::Perspective3<f32> {
//     let aspect_ratio = extent.width as f32 / extent.height as f32;
//     let fovy = glm::quarter_pi();
//     let projection = na::Perspective3::new(aspect_ratio, fovy, 0.1, 10.0);

//     projection
// }

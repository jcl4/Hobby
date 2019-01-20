use super::{
    base, base::QueueData, command_buffer::CommandBufferData, render_pass, swapchain,
    swapchain::SwapchainData,
};
use crate::{core::Model, settings::HobbySettings, Result};
use ash;
use ash::{
    extensions::{ext::DebugReport, khr::Surface},
    version::{DeviceV1_0, InstanceV1_0},
    vk,
};
use failure::bail;
use log::{debug, info, trace};
use winit::{EventsLoop, Window};

const FRAMES_IN_FLIGHT: u32 = 2;

pub(crate) struct Renderer {
    _window: Window,
    _entry: ash::Entry,
    instance: ash::Instance,
    surface: vk::SurfaceKHR,
    surface_loader: Surface,
    debug_callback: vk::DebugReportCallbackEXT,
    debug_loader: DebugReport,
    pub(crate) physical_device: vk::PhysicalDevice,
    pub(crate) device: ash::Device,
    pub(crate) queue_data: QueueData,
    pub(crate) swapchain_data: SwapchainData,
    pub(crate) command_buffer_data: CommandBufferData,
    pub(crate) framebuffers: Vec<vk::Framebuffer>,
    pub(crate) render_pass: vk::RenderPass,
    img_available_semaphores: Vec<vk::Semaphore>,
    render_finished_sempahores: Vec<vk::Semaphore>,
    in_flight_fences: Vec<vk::Fence>,
    current_frame: u32,
    pub(crate) is_resized: bool,
}

impl Renderer {
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new(
        hobby_settings: &HobbySettings,
        events_loop: &EventsLoop,
    ) -> Result<Renderer> {
        let entry = ash::Entry::new().unwrap();

        let window = base::create_window(&events_loop, &hobby_settings.window_settings)?;
        let instance = base::create_instance(&hobby_settings.app_info, &entry)?;
        let surface = unsafe { base::create_surface(&entry, &instance, &window)? };
        let surface_loader = Surface::new(&entry, &instance);
        let (debug_callback, debug_loader) = base::setup_debug_callback(&entry, &instance)?;
        let physical_device = unsafe { instance.enumerate_physical_devices()?.remove(0) };
        let (device, queue_data) =
            base::create_device_and_queues(&instance, physical_device, &surface_loader, surface)?;
        let swapchain_data = swapchain::create_swapchain_and_image_views(
            &surface_loader,
            physical_device,
            surface,
            &instance,
            &device,
        )?;

        let command_buffer_data = CommandBufferData::new(&device, &queue_data, &swapchain_data)?;

        let render_pass =
            render_pass::create_render_pass(swapchain_data.surface_format.format, &device)?;

        let framebuffers = base::create_framebuffers(&swapchain_data, render_pass, &device)?;

        let (img_available_semaphores, render_finished_sempahores, in_flight_fences) =
            create_sync_objects(&device)?;

        let renderer = Renderer {
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
            command_buffer_data,
            render_pass,
            framebuffers,
            img_available_semaphores,
            render_finished_sempahores,
            in_flight_fences,
            current_frame: 0,
            is_resized: false,
        };

        Ok(renderer)
    }

    pub fn draw_frame(&mut self, models: &mut [Model]) -> Result<()> {
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
                            self.recreate_swapchain(models)?;
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
                .command_buffers(&[self.command_buffer_data.command_buffers[img_index as usize]])
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
                self.recreate_swapchain(models)?;
            }
        }

        self.current_frame = (self.current_frame + 1) % FRAMES_IN_FLIGHT;

        Ok(())
    }

    fn clean_up_swapchian(&self, models: &[Model]) -> Result<()> {
        info!("Cleanup Swapchain Called");
        unsafe {
            for frame_buffer in self.framebuffers.iter() {
                self.device.destroy_framebuffer(*frame_buffer, None);
            }

            for model in models {
                model.cleanup(&self.device)?;
            }

            self.command_buffer_data.cleanup_buffers(&self.device);

            self.device.destroy_render_pass(self.render_pass, None);

            for img_view in self.swapchain_data.image_views.iter() {
                self.device.destroy_image_view(*img_view, None);
            }

            self.swapchain_data
                .swapchain_loader
                .destroy_swapchain(self.swapchain_data.swapchain, None);
        }

        Ok(())
    }

    fn recreate_swapchain(&mut self, models: &mut [Model]) -> Result<()> {
        info!("Recreate Swapchain Called");

        unsafe {
            self.device.device_wait_idle()?;
            self.clean_up_swapchian(models)?;
        }

        info!("Recreating Swapchain");
        self.swapchain_data = swapchain::create_swapchain_and_image_views(
            &self.surface_loader,
            self.physical_device,
            self.surface,
            &self.instance,
            &self.device,
        )?;

        self.render_pass = render_pass::create_render_pass(
            self.swapchain_data.surface_format.format,
            &self.device,
        )?;

        for model in models.iter_mut() {
            model.build(&self)?;
        }

        self.framebuffers =
            base::create_framebuffers(&self.swapchain_data, self.render_pass, &self.device)?;

        self.command_buffer_data
            .recreate_command_buffers(&self.device, &self.swapchain_data)?;

        self.command_buffer_data.build_cb(
            &self.device,
            &self.swapchain_data,
            &self.framebuffers,
            self.render_pass,
            models,
        )?;

        Ok(())
    }

    pub fn find_memory_type(
        &self,
        type_filter: u32,
        required_properties: vk::MemoryPropertyFlags,
    ) -> Result<u32> {
        unsafe {
            let mem_properties = self
                .instance
                .get_physical_device_memory_properties(self.physical_device);
            let mut debug_str = String::from("Find Memeory Type\n");
            debug_str.push_str(&format!(
                "Type Filter: {}, Required Properties:{:?}\n",
                type_filter, required_properties
            ));
            for (i, memory_type) in mem_properties.memory_types.iter().enumerate() {
                debug_str.push_str(&format!("Memory Index:{}\n{:#?}\n", i, memory_type,));

                if (type_filter & (1 << i)) > 0
                    && (memory_type.property_flags.contains(required_properties))
                {
                    trace!("{}", debug_str);
                    return Ok(i as u32);
                }
            }
        }

        bail!(
            "Unable to find suitable memory type, TypeFilter: {}\nRequired Properties: {:?}\n ",
            type_filter,
            required_properties
        )
    }

    pub(crate) fn cleanup(&self, models: &[Model]) -> Result<()> {
        unsafe {
            self.clean_up_swapchian(models)?;

            for i in 0..FRAMES_IN_FLIGHT {
                self.device
                    .destroy_semaphore(self.render_finished_sempahores[i as usize], None);
                self.device
                    .destroy_semaphore(self.img_available_semaphores[i as usize], None);
                self.device
                    .destroy_fence(self.in_flight_fences[i as usize], None);
            }
            self.command_buffer_data.cleanup_command_pool(&self.device);

            debug!("Destroy Device");
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);

            self.debug_loader
                .destroy_debug_report_callback(self.debug_callback, None);

            self.instance.destroy_instance(None);
        }
        Ok(())
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

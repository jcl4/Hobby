use log::info;
use std::sync::Arc;
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::framebuffer::{RenderPassAbstract, RenderPassDesc};
use vulkano::single_pass_renderpass;


pub(crate) fn create_render_pass(
    device: &Arc<Device>,
    color_format: Format,
) -> Arc<RenderPassAbstract + Send + Sync> {
    let render_pass = Arc::new(
        single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: color_format,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap(),
    );

    let render_pass_desc = render_pass.desc();

    info!("Render Pass Created");
    info!(
        "\tNumber of attachements: {}",
        render_pass_desc.num_attachments()
    );

    for (index, attachment_desc) in render_pass_desc.attachment_descs().enumerate() {
        info!(
            "\tAttachement {} Description: {:#?}",
            index, attachment_desc
        );
    }

    info!(
        "\tNumber of subpasses: {}",
        render_pass_desc.num_attachments()
    );

    for (index, subpass) in render_pass_desc.subpass_descs().enumerate() {
        info!("\tSubpass {} Description: {:#?}", index, subpass);
    }

    info!(
        "\tNumber of dependencies {}",
        render_pass_desc.num_dependencies()
    );

    for (index, dependency) in render_pass_desc.dependency_descs().enumerate() {
        info!("Dependance {} Description: {:#?}", index, dependency);
    }

    render_pass
}

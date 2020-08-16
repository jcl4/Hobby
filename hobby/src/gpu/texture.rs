use crate::gpu::Context;
use image::GenericImageView;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(
        bytes: &[u8],
        context: &Context,
        label: &str,
    ) -> (Texture, wgpu::CommandBuffer) {
        let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Jpeg)
            .expect("Unable to load texture from memory");
        // match &img {
        //     image::DynamicImage::ImageRgb8(x) => log::warn!("RGB8 Image"),
        //     image::DynamicImage::ImageRgba8(x) => log::warn!("RGBA8 Image"),
        //     _ => log::warn!("Unknow Image Format"),
        // };

        Texture::from_image(context, &img, label)
    }

    pub fn from_image(
        context: &Context,
        img: &image::DynamicImage,
        label: &str,
    ) -> (Texture, wgpu::CommandBuffer) {
        log::info!("Createing Texture: {}", label);
        let rgba = img.to_rgba();

        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };

        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let buffer = context
            .device
            .create_buffer_with_data(&rgba, wgpu::BufferUsage::COPY_SRC);

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("texture_buffer_copy_encoder"),
            });

        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            size,
        );
        let cmd_buffer = encoder.finish();

        let view = texture.create_default_view();
        let sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Always,
        });

        (
            Texture {
                texture,
                view,
                sampler,
            },
            cmd_buffer,
        )
    }
}

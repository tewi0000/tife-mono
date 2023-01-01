use std::{path::Path, fs};

use image::{GenericImageView, DynamicImage};
use wgpu::{Device, Queue, FilterMode, BindGroupLayout};
use color_eyre::Result;

use super::bindable::Bindable;

pub struct Texture {
    pub sampler : wgpu::Sampler,
    pub texture : wgpu::Texture,
    pub view    : wgpu::TextureView,
    
    bind_group        : wgpu::BindGroup,
    bind_group_layout : wgpu::BindGroupLayout,
}

impl Texture {
    pub fn from_path(device      : &Device,
                     queue       : &Queue,
                     path        : impl AsRef<Path>,
                     filter_mode : FilterMode,
                     label       : &str) -> Result<Self> {
        let bytes = fs::read(path)?;

        return Self::from_bytes(device, queue, &bytes, filter_mode, label);
    }

    pub fn from_bytes(device      : &Device,
                      queue       : &Queue,
                      bytes       : &[u8],
                      filter_mode : FilterMode,
                      label       : &str) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        return Self::from_image(device, queue, &img, filter_mode, Some(label));
    }

    pub fn from_image(device      : &Device,
                      queue       : &Queue,
                      img         : &DynamicImage,
                      filter_mode : FilterMode,
                      label       : Option<&str>) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
        let size = wgpu::Extent3d {
            width                 : dimensions.0,
            height                : dimensions.1,
            depth_or_array_layers : 1,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label,
                size,
                mip_level_count : 1,
                sample_count    : 1,
                dimension       : wgpu::TextureDimension::D2,
                format          : wgpu::TextureFormat::Rgba8UnormSrgb,
                usage           : wgpu::TextureUsages::TEXTURE_BINDING
                                | wgpu::TextureUsages::COPY_DST,
            }
        );

        // Upload texture
        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect    : wgpu::TextureAspect::All,
                texture   : &texture,
                mip_level : 0,
                origin    : wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset         : 0,
                bytes_per_row  : std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image : std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        // Shaders stuff
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u : wgpu::AddressMode::ClampToEdge,
                address_mode_v : wgpu::AddressMode::ClampToEdge,
                address_mode_w : wgpu::AddressMode::ClampToEdge,
                mag_filter     : filter_mode,
                min_filter     : filter_mode,
                mipmap_filter  : wgpu::FilterMode::Nearest,
                .. Default::default()
            }
        );

        // Binding stuff
        let bind_group_layout = Self::default_layout(device);
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding  : 0,
                        resource : wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding  : 1,
                        resource : wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );
        
        return Ok(Self { texture, view, sampler, bind_group, bind_group_layout });
    }

    pub fn default_layout(device: &wgpu::Device) -> BindGroupLayout {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding    : 0,
                    visibility : wgpu::ShaderStages::FRAGMENT,
                    ty         : wgpu::BindingType::Texture {
                        multisampled   : false,
                        view_dimension : wgpu::TextureViewDimension::D2,
                        sample_type    : wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count      : None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding    : 1,
                    visibility : wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the corresponding Texture entry above.
                    ty         : wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count      : None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        return bind_group_layout;
    }
}

impl Bindable<[[f32; 4]; 4]> for Texture {
    fn bind<'a, 'b: 'a>(&'b self, render_pass: &mut wgpu::RenderPass<'a>, index: u32) {
        render_pass.set_bind_group(index, &self.bind_group, &[]);
    }

    fn layout(&self) -> &wgpu::BindGroupLayout {
        return &self.bind_group_layout;
    }

    fn group(&self) -> &wgpu::BindGroup {
        return &self.bind_group;
    }

    #[allow(unused_variables)] // TODO: remove this annotation
     fn update(&self, queue: &wgpu::Queue, value: &[[f32; 4]; 4]) {
        todo!() // Upload new texture data here, I guess
    }

}
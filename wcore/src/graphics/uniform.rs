use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use super::bindable::Bindable;

pub struct Uniform<T: Default + Clone + Pod + Zeroable = [[f32; 4]; 4]> {
    buffer            : wgpu::Buffer,
    bind_group        : wgpu::BindGroup,
    bind_group_layout : wgpu::BindGroupLayout,
    _0                : PhantomData<T>,
}

impl<T: Default + Clone + Pod + Zeroable> Uniform<T> {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::bytes_of(&T::default()),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
        });

        return Self {
            buffer,
            bind_group,
            bind_group_layout,
            _0: Default::default()
        };
    }
}

impl<T: Default + Clone + Pod + Zeroable> Bindable<T> for Uniform<T> {
    fn bind<'a, 'b: 'a>(&'b self, render_pass: &mut wgpu::RenderPass<'a>, index: u32) {
        render_pass.set_bind_group(index, &self.bind_group, &[]);
    }

    fn layout(&self) -> &wgpu::BindGroupLayout {
        return &self.bind_group_layout;
    }

    fn group(&self) -> &wgpu::BindGroup {
        return &self.bind_group;
    }
    
    /// Loads uniform with a new value.
    /// The value is updated at the end of the RenderPass.
    fn update(&self, queue: &wgpu::Queue, value: &T) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(value));
    }
}
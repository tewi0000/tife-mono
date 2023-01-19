use std::marker::PhantomData;

use bytemuck::{Zeroable, Pod};
use wgpu::util::DeviceExt;

use crate::graphics::drawable::Drawable;

use super::simple::Mesh;

/* Instanced Mesh */
pub trait Instance {
    type InstanceRaw: Pod + Zeroable;
    fn to_raw(&self) -> Self::InstanceRaw;
}

pub struct InstancedMesh<I: Instance, V: Pod + Zeroable> {
        buffer          : wgpu::Buffer,
        instance_buffer : wgpu::Buffer,

    pub vertices        : Vec<V>,
    pub instances       : Vec<I>,
}

impl<I: Instance, V: Pod + Zeroable> InstancedMesh<I, V> {
    pub fn new(device: &wgpu::Device, vertices: Vec<V>, instances: Vec<I>) -> Self {
        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = InstancedMesh::<I, V>::make_buffer(device, &instance_data);
        let buffer = InstancedMesh::<I, V>::make_buffer(device, &vertices);

        return Self {
            buffer,
            instance_buffer,

            vertices,
            instances,
        };
    }

    pub fn bake(&mut self, device: &wgpu::Device) {
        self.buffer = Mesh::make_buffer(device, &self.vertices);
    }

    pub fn bake_instances(&mut self, device: &wgpu::Device) {
        let instance_data = self.instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        self.instance_buffer = InstancedMesh::<I, V>::make_buffer(device, &instance_data);
    }

    pub fn update(&mut self, data: &[V], queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data));
    }

    pub fn update_instances(&mut self, data: &[I::InstanceRaw], queue: &wgpu::Queue) {
        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(data));
    }

    fn make_buffer<T: Pod + Zeroable>(device: &wgpu::Device, data: &[T]) -> wgpu::Buffer {
        return device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
    }
}

impl<I: Instance, V: Pod + Zeroable> Drawable for InstancedMesh<I, V> {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.instances.len() > 0 {
            render_pass.set_vertex_buffer(0, self.buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.draw(0 .. self.vertices.len() as u32, 0 .. self.instances.len() as u32);
        }
    }
}

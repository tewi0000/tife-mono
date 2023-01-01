use bytemuck::{Zeroable, Pod};
use wgpu::util::DeviceExt;

use crate::graphics::drawable::Drawable;

/* Mesh */
pub struct Mesh<V: Pod + Zeroable> {
        buffer   : wgpu::Buffer,
    pub vertices : Vec<V>
}

impl<V: Pod + Zeroable> Mesh<V> {
    pub fn new(device: &wgpu::Device, vertices: Vec<V>) -> Self {
        let buffer = Mesh::make_buffer(device, &vertices);

        return Self {
            buffer,
            vertices,
        };
    }

    // TODO: opt for a safer approach and make vertices private?
    pub fn bake(&mut self, device: &wgpu::Device) {
        self.buffer = Mesh::make_buffer(device, &self.vertices);
    }

    pub fn update(&mut self, data: &[V], queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&data));
    }

    pub(crate) fn make_buffer(device: &wgpu::Device, vertices: &[V]) -> wgpu::Buffer {
        return device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );
    }
}

impl<V: Pod + Zeroable> Drawable for Mesh<V> {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        let slice = self.buffer.slice(..);
        render_pass.set_vertex_buffer(0, slice);
        render_pass.draw(0 .. self.vertices.len() as u32, 0 .. 1);
    }
}
use bytemuck::{Pod, Zeroable};

pub trait Bindable<T: Default + Clone + Pod + Zeroable = [[f32; 4]; 4]> {
    fn bind<'a, 'b: 'a>(&'b self, render_pass: &mut wgpu::RenderPass<'a>, index: u32);
    fn layout(&self) -> &wgpu::BindGroupLayout;
    fn group(&self) -> &wgpu::BindGroup;
    fn update(&self, queue: &wgpu::Queue, value: &T);
}
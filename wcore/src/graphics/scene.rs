use cgmath::Matrix4;

use super::{camera::{ProjectionOrthographic, Camera2D, Transformation}, uniform::Uniform, bindable::Bindable};

pub trait Scene: Bindable<[[f32; 4]; 4]> + Transformation {}

impl Scene for Scene2D {}
pub struct Scene2D {
    pub projection : ProjectionOrthographic,
    pub camera     : Camera2D,
    pub uniform    : Uniform,
}

impl Scene2D {
    pub fn new(graphics: &wgpu::Device, width: u32, height: u32) -> Self {
        return Self {
            projection : ProjectionOrthographic::new(width, height, -100.0, 100.0),
            camera     : Camera2D { position: (0.0, 0.0, -50.0).into() },
            uniform    : Uniform::new(graphics),
        };
    }

    pub fn update(&self, queue: &wgpu::Queue) {
        Bindable::update(self, queue, &(self.apply()).into());
    }
}

impl Bindable<[[f32; 4]; 4]> for Scene2D {
    fn bind<'a, 'b: 'a>(&'b self, render_pass: &mut wgpu::RenderPass<'a>, index: u32) {
        self.uniform.bind(render_pass, index);
    }

    fn layout(&self) -> &wgpu::BindGroupLayout {
        return self.uniform.layout();
    }

    fn group(&self) -> &wgpu::BindGroup {
        return self.uniform.group();
    }

    fn update(&self, queue: &wgpu::Queue, value: &[[f32; 4]; 4]) {
        self.uniform.update(queue, value);
    }

}

impl Transformation for Scene2D {
    fn apply(&self) -> Matrix4<f32> {
        return self.projection.apply() * self.camera.apply();
    }
}
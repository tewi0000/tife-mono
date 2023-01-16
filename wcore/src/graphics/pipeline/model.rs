use wgpu::include_wgsl;

use crate::{graphics::{texture::Texture, scene::Scene, primitive::mesh::data::{vertex::Vertex, model::ModelRaw}, uniform::Uniform, bindable::Bindable, utils}};

use super::{shader::scene::SceneSlot, Pipeline};

pub struct ModelPipeline {
    pipeline: wgpu::RenderPipeline,
    
    scene_uniform: Uniform<[[f32; 4]; 4]>
}

impl Pipeline for ModelPipeline {
    fn attach<'a, 'b: 'a>(&'b self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        self.scene_uniform.bind(render_pass, 0);
    }
}

impl ModelPipeline {
    pub fn new(device: &wgpu::Device, surface_configuration: &wgpu::SurfaceConfiguration, scene: &impl Scene, depth: bool) -> Self {
        let shader = device.create_shader_module(include_wgsl!("model.wgsl"));
        
        let bind_layout = &[
            scene.layout(),
            &Texture::default_layout(device),
        ];

        let buffer_layout = &[
            Vertex::describe(),
            ModelRaw::describe(),
        ];

        let pipeline = utils::pipeline(
            device,
            &shader,
            surface_configuration,
            bind_layout,
            buffer_layout,
            depth
        );

        let scene_uniform = Uniform::new(device);

        return Self {
            pipeline,
            
            scene_uniform,
        };
    }
}

unsafe impl SceneSlot for ModelPipeline {
    fn update(&self, queue: &wgpu::Queue, scene: &impl Scene) {
        self.scene_uniform.update(queue, &scene.apply().into());
    }
}
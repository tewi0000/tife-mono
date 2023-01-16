use std::path::PathBuf;

use wcore::graphics::{screen::Screen, context::GraphicsContext, utils, pipeline::{model::ModelPipeline, Pipeline, shader::scene::SceneSlot}, scene::Scene2D, texture::Texture, bindable::Bindable, primitive::mesh::{instanced::InstancedMesh, data::{model::{Model, ModelRaw}, vertex::Vertex}}, drawable::Drawable, camera::Projection};
use cgmath::{vec3, Quaternion, Zero, vec4};

use crate::state::State;

pub struct TestScreen {
    pub scene: Scene2D,

    pub texture_test: Texture,

    pub pipeline_model: ModelPipeline,
    pub mesh_model: InstancedMesh<Model, ModelRaw, Vertex>,
}

impl TestScreen {
    pub fn new(graphics: &GraphicsContext) -> Self {
        let width = graphics.surface_configuration.width;
        let height = graphics.surface_configuration.height;
        let scene = Scene2D::new(&graphics.device, width, height);

        let pipeline_model = ModelPipeline::new(&graphics.device, &graphics.surface_configuration, &scene, false);
        let mesh_model = InstancedMesh::new(&graphics.device, Vertex::vertices_rect(-0.5, 0.5), vec![
            Model { position: vec3(200.0, 200.0, 0.0), rotation: Quaternion::zero(), scale: vec3(100.0, 100.0, 1.0), color: vec4(1.0, 1.0, 1.0, 1.0) }
        ]);

        let path = PathBuf::from("resources/textures");
        let texture_test = Texture::from_path(&graphics.device, &graphics.queue, path.join("test.png"), wgpu::FilterMode::Linear, "test").unwrap();

        pipeline_model.update(&graphics.queue, &scene);
        
        return Self {
            scene,

            texture_test,

            pipeline_model,
            mesh_model,
        };
    }
}

impl Screen<State> for TestScreen {
    fn render(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut GraphicsContext) {
        utils::submit(&graphics.queue, &graphics.device, |encoder| {
            utils::render(encoder, &view, None, |mut render_pass| {
                self.mesh_model.instances = vec![
                    Model { position: vec3(200.0, 200.0, 0.0), rotation: Quaternion::zero(), scale: vec3(100.0, 100.0, 1.0), color: vec4(1.0, 1.0, 1.0, 1.0) },
                    Model { position: vec3(400.0, 200.0, 0.0), rotation: Quaternion::zero(), scale: vec3(100.0, 100.0, 1.0), color: vec4(1.0, 1.0, 1.0, 1.0) },
                    Model { position: vec3(600.0, 200.0, 0.0), rotation: Quaternion::zero(), scale: vec3(100.0, 100.0, 1.0), color: vec4(1.0, 1.0, 1.0, 1.0) }
                ]; self.mesh_model.bake_instances(&graphics.device);

                self.texture_test.bind(&mut render_pass, 1);
                
                self.pipeline_model.attach(&mut render_pass);
                self.mesh_model.draw(&mut render_pass); // Only .draw() is affected by current pipeline
            });
        });
    }

    fn resize(&mut self, state: &mut State, width: u32, height: u32, graphics: &mut GraphicsContext) {
        self.scene.projection.resize(width, height);
        self.pipeline_model.update(&graphics.queue, &self.scene);
    }
}
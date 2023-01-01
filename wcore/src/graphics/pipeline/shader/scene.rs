use crate::graphics::scene::Scene;

pub unsafe trait SceneSlot {
    fn update(&self, queue: &wgpu::Queue, scene: &impl Scene);
}
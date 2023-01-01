pub mod model;
pub mod shader;

pub trait Pipeline {
    fn attach<'a, 'b: 'a>(&'b self, render_pass: &mut wgpu::RenderPass<'a>);
}
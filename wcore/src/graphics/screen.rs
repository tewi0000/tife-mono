use super::context::GraphicsContext;

pub trait Screen<State> {
	fn render(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut GraphicsContext) {}
	fn resize(&mut self, state: &mut State, width: u32, height: u32, graphics: &mut GraphicsContext) {}
}
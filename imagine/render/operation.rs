use crate::render::RenderContext;

pub trait RenderOperation {
  fn input(&self) -> Vec<String>;
  fn output(&self) -> Vec<String>;
  fn run(
    &self,
    context: &mut RenderContext
  ) -> wgpu::CommandBuffer;
}
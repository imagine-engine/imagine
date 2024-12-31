use crate::render::RenderContext;

pub trait NodeBuilder {
  type Op;
  fn build(&self, context: &mut RenderContext) -> Self::Op;
}

pub trait RenderOperation {
  fn input(&self) -> Vec<String>;
  fn output(&self) -> Vec<String>;
  fn run(&self, context: &mut RenderContext);
}
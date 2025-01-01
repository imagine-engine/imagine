use crate::render::RenderContext;

pub trait NodeBuilder {
  type Node;
  fn build(&self, context: &mut RenderContext) -> Self::Node;
}

pub trait RenderOperation {
  fn input(&self) -> Vec<String>;
  fn output(&self) -> Vec<String>;
  fn run(&self, context: &mut RenderContext);
}
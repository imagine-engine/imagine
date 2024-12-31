pub use nalgebra::Vector3;
use crate::render::{
  RenderGraph,
  RenderContext,
  RenderOperation,
  NodeBuilder
};

use approx::assert_abs_diff_eq;

#[derive(Clone)]
struct MatrixOp {
  pub a: String,
  pub b: String,
  pub result: String,
  pub op: String
}
type MatrixOpBuilder = MatrixOp;

impl NodeBuilder for MatrixOpBuilder {
  type Op = MatrixOp;

  fn build(&self, context: &mut RenderContext) -> Self::Op {
    context.add_resource(&self.a, Vector3::<f32>::repeat(1.0));
    context.add_resource(&self.b, Vector3::<f32>::repeat(1.0));
    context.add_resource(&self.result, Vector3::<f32>::zeros());
    self.clone()
  }
}

impl RenderOperation for MatrixOp {
  fn input(&self) -> Vec<String> {
    vec![self.a.clone(), self.b.clone()]
  }
  fn output(&self) -> Vec<String> {
    vec![self.result.clone()]
  }
  fn run(&self, context: &mut RenderContext) {
    let m1: Option<Vector3<f32>> = context.get(&self.a).copied();
    let m2: Option<Vector3<f32>> = context.get(&self.b).copied();
    if let (Some(a), Some(b), Some(result)) = (
      m1,
      m2,
      context.get_mut(&self.result)
    ) {
      *result = a + b;
    }
  }
}

#[test]
fn basic_one_op() {
  let mut graph = RenderGraph::empty();
  graph.add_node("matrix_add", MatrixOpBuilder {
    a: "matrix_1".to_string(),
    b: "matrix_2".to_string(),
    result: "result".to_string(),
    op: "+".to_string()
  });

  assert!(graph.get_resource::<Vector3<f32>>("matrix_1").is_some());
  assert!(graph.get_resource::<Vector3<f32>>("matrix_2").is_some());
  assert!(graph.get_resource::<Vector3<f32>>("result").is_some());
  let m1 = Vector3::<f32>::new_random();
  let m2 = Vector3::<f32>::new_random();
  let result = m1 + m2;
  *graph.get_resource_mut("matrix_1").unwrap() = m1;
  *graph.get_resource_mut("matrix_2").unwrap() = m2;
  graph.run();
  assert_abs_diff_eq!(result, graph.get_resource("result").unwrap());
}
pub use nalgebra::Vector3;
use crate::render::RenderGraph;
use super::util::MatrixOpBuilder;

use approx::assert_abs_diff_eq;

use rand::Rng;

#[test]
fn single_one_op() {
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

#[test]
fn parallel_all_one_op() {
  let mut graph = RenderGraph::empty();
  for i in 0..10 {
    graph.add_node(&format!("add_{}", i), MatrixOpBuilder {
      a: format!("matrix_1_{}", i),
      b: format!("matrix_2_{}", i),
      result: format!("result_{}", i),
      op: "+".to_string()
    });
  }

  let mut results = Vec::new();
  for i in 0..10 {
    let m1 = format!("matrix_1_{}", i);
    let m2 = format!("matrix_2_{}", i);
    assert!(graph.get_resource::<Vector3<f32>>(&m1).is_some());
    assert!(graph.get_resource::<Vector3<f32>>(&m2).is_some());
    let new_m1 = Vector3::<f32>::new_random();
    let new_m2 = Vector3::<f32>::new_random();
    *graph.get_resource_mut(&m1).unwrap() = new_m1;
    *graph.get_resource_mut(&m2).unwrap() = new_m2;
    results.push(new_m1 + new_m2);
  }

  graph.run();

  for i in 0..10 {
    let res = format!("result_{}", i);
    assert_abs_diff_eq!(results[i], graph.get_resource(&res).unwrap());
  }
}

#[test]
fn parallel_select_one_op() {
  let mut graph = RenderGraph::empty();
  for i in 0..10 {
    graph.add_node(&format!("add_{}", i), MatrixOpBuilder {
      a: format!("matrix_1_{}", i),
      b: format!("matrix_2_{}", i),
      result: format!("result_{}", i),
      op: "+".to_string()
    });
  }

  let mut expected = Vec::new();
  let mut rng = rand::thread_rng();
  let selected: Vec<usize> = (0..rng.gen_range(1..5)).map(|x| rng.gen_range(0..10)).collect();
  for i in 0..10 {
    let m1 = format!("matrix_1_{}", i);
    let m2 = format!("matrix_2_{}", i);
    let res = format!("result_{}", i);
    assert!(graph.get_resource::<Vector3<f32>>(&m1).is_some());
    assert!(graph.get_resource::<Vector3<f32>>(&m2).is_some());
    assert!(graph.get_resource::<Vector3<f32>>(&res).is_some());
    let expected_result = if selected.contains(&i) {
      let new_m1 = Vector3::<f32>::new_random();
      let new_m2 = Vector3::<f32>::new_random();
      *graph.get_resource_mut(&m1).unwrap() = new_m1;
      *graph.get_resource_mut(&m2).unwrap() = new_m2;
      new_m1 + new_m2
    } else {
      graph.get_resource::<Vector3<f32>>(&res).unwrap().clone()
    };
    expected.push((res, expected_result));
  }

  graph.run();

  for (res_name, expected_result) in expected.iter() {
    assert_abs_diff_eq!(
      expected_result,
      graph.get_resource(&res_name).unwrap()
    );
  }
}
use crate::Color;
use pyo3::prelude::*;
use crate::math::Vector;

#[pyclass]
pub struct Ellipse {
  pub id: i32,
  #[pyo3(get, set)]
  pub fill: Color,
  #[pyo3(get, set)]
  pub stroke: Color,
  #[pyo3(get, set)]
  pub scale: Vector,
  #[pyo3(get, set)]
  pub position: Vector,
  pub rotation: f32
}

// #[pyfunction]
// #[pyo3(name="Circle", signature=(radius=10.0))]
// pub fn circle(radius: f32) -> Ellipse {
//   Ellipse::new(radius, radius)
// }
use pyo3::prelude::*;

use crate::Color;
use crate::math::Vector;

#[pyclass]
pub struct Path {
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
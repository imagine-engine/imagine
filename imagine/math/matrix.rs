use pyo3::prelude::*;
use nalgebra::base::Matrix4;

#[pyclass]
pub struct Matrix {
  #[pyo3(get, set)]
  pub dim: u8,
  pub elements: Vec<Vec<f32>>
}

// impl Matrix {
//   pub fn to_matrix4(&self) -> Matrix4 {
//   }
// }

#[pymethods]
impl Matrix {
  #[new]
  pub fn new(elements: Vec<Vec<f32>>) -> Self {
    Self {
      dim: elements.len(),
      elements
    }
  }
}
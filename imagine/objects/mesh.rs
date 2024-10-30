use pyo3::prelude::*;
use crate::math::vector::Vector;
use crate::render::primitives::Vertex3D;

#[pyclass]
pub struct Mesh {
  #[pyo3(get)]
  pub scale: Vector,
  #[pyo3(get)]
  pub position: Vector,
  #[pyo3(get)]
  pub rotation: Vector
}

impl Mesh {
  pub fn new(positions: Vec<Vertex3D>, indices: Vec<u32>) -> Self {
    Self {
      scale: Vector::new(1.0, 1.0, 1.0),
      position: Vector::new(0.0, 0.0, 0.0),
      rotation: Vector::new(1.0, 1.0, 1.0)
    }
  }
}
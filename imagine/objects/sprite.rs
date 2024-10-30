use pyo3::prelude::*;
use nalgebra::{Vector2, Matrix3};
use crate::math::vector::Vector;
use crate::instance::IMAGINE;
use crate::render::primitives::SpriteConfig;

#[pyclass]
pub struct Sprite {
  #[pyo3(get)]
  pub scale: Vector,
  #[pyo3(get)]
  pub position: Vector,
  #[pyo3(get)]
  pub rotation: f32
}

#[pymethods]
impl Sprite {
  pub fn new() -> Self {
    Self {
      scale: Vector::new(1.0, 1.0, 1.0),
      position: Vector::new(0.0, 0.0, 0.0),
      rotation: Vector::new(1.0, 1.0, 1.0)
    }
  }
}
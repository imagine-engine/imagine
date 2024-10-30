use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct Vector {
  #[pyo3(get, set)]
  pub x: f32,
  #[pyo3(get, set)]
  pub y: f32,
  #[pyo3(get, set)]
  pub z: f32
}

#[pymethods]
impl Vector {
  #[new]
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self {
      x,
      y,
      z
    }
  }

  fn __str__(&self) -> PyResult<String> {
    Ok(format!("({}, {}, {})", self.x, self.y, self.z).to_string())
  }

  fn __repr__(&self) -> PyResult<String> {
    self.__str__()
  }
}
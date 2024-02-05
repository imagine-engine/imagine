/*******************************************************************************
  vector.rs
********************************************************************************
  Copyright 2024 Menelik Eyasu

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.
*******************************************************************************/

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

  fn __repr__(&self) -> PyResult<String> {
    Ok(format!("({}, {}, {})", self.x, self.y, self.z).to_string())
  }
}
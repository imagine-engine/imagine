/*******************************************************************************
  scene_children.rs
********************************************************************************
  Copyright 2023 Menelik Eyasu

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
use nalgebra::Matrix4;

#[pyclass]
#[derive(Clone)]
pub struct SceneObject {
  #[pyo3(get, set)]
  pub vertices: Vec<f32>,
  #[pyo3(get, set)]
  pub indices: Vec<i32>,
  pub transform: Matrix4<f32>
}

#[pyclass]
#[derive(Clone)]
pub struct SceneLight {
  #[pyo3(get, set)]
  pub position: Vec<f32>
}

#[pymethods]
impl SceneObject {
  #[getter(transform)]
  fn get_transform(&self) -> PyResult<Vec<Vec<f32>>> {
    Ok(vec![
      vec![self.transform.m11, self.transform.m12,
           self.transform.m13, self.transform.m14],
      vec![self.transform.m21, self.transform.m22,
           self.transform.m23, self.transform.m24],
      vec![self.transform.m31, self.transform.m32,
           self.transform.m33, self.transform.m34],
      vec![self.transform.m41, self.transform.m42,
           self.transform.m43, self.transform.m44]
    ])
  }
}
/*******************************************************************************
  svg.rs
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

use std::path::Path;
use pyo3::prelude::*;
use nalgebra::Matrix4;
use pyo3::exceptions::PyValueError;
use crate::scene_children::SceneObject;
use super::component::ImagineComponent;

#[pyfunction]
#[pyo3(name = "SVG")]
pub fn svg(source: String) -> PyResult<ImagineComponent> {
  let path = Path::new(&source);
  if !path.exists() {
    return Err(PyValueError::new_err("path does not exist"));
  }

  let scene_object = SceneObject {
    vertices: vec![
      -0.5, -0.5, 0.0,
      0.5, -0.5, 0.0,
      0.0, 0.5, 0.0
    ],
    indices: vec![0, 1, 2],
    transform: Matrix4::identity()
  };

  Ok(ImagineComponent::new(scene_object))
}
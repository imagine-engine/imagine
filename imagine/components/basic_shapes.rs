/*******************************************************************************
  square.rs
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
use crate::scene_children::SceneObject;
use super::component::ImagineComponent;

#[pyfunction]
#[pyo3(name = "Triangle")]
pub fn triangle() -> PyResult<ImagineComponent> {
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

#[pyfunction]
#[pyo3(name = "Square", signature = (size=1.0))]
pub fn square(size: f32) -> PyResult<ImagineComponent> {
  let half_size = size/2.0;
  let scene_object = SceneObject {
    vertices: vec![
      half_size, half_size, 0.0,
      half_size, -half_size, 0.0,
      -half_size, -half_size, 0.0,
      -half_size, half_size, 0.0
    ],
    indices: vec![0, 1, 3, 1, 2, 3],
    transform: Matrix4::identity()
  };

  Ok(ImagineComponent::new(scene_object))
}

#[pyfunction]
#[pyo3(name = "Rectangle", signature = (width=1.0, height=1.0))]
pub fn rectangle(width: f32, height: f32) -> PyResult<ImagineComponent> {
  let half_width = width/2.0;
  let half_height = height/2.0;
  let scene_object = SceneObject {
    vertices: vec![
      half_width, half_height, 0.0,
      half_width, -half_height, 0.0,
      -half_width, -half_height, 0.0,
      -half_width, half_height, 0.0
    ],
    indices: vec![0, 1, 3, 1, 2, 3],
    transform: Matrix4::identity()
  };

  Ok(ImagineComponent::new(scene_object))
}
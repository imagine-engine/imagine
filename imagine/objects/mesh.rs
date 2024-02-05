/*******************************************************************************
  mesh.rs
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
use crate::instance::IMAGINE;
use crate::math::vector::Vector;
use nalgebra::{Vector3, Matrix4};
use crate::controller::Object3DController;
use crate::render::primitives::{Vertex3D, Object3D};

#[pyclass]
pub struct Mesh {
  #[pyo3(get)]
  pub scale: Vector,
  #[pyo3(get)]
  pub position: Vector,
  #[pyo3(get)]
  pub rotation: Vector,
  pub world_object: Object3DController
}

impl Mesh {
  pub fn new(positions: Vec<Vertex3D>, indices: Vec<u32>) -> Self {
    Self {
      scale: Vector::new(1.0, 1.0, 1.0),
      position: Vector::new(0.0, 0.0, 0.0),
      rotation: Vector::new(1.0, 1.0, 1.0),
      world_object: IMAGINE.lock().unwrap().world.add_mesh(Object3D {
        vertices: Vec::new(),
        indices,
        scale: Vector3::new(1.0, 1.0, 1.0),
        position: Vector3::zeros(),
        rotation: Vector3::zeros(),
        transform: Matrix4::identity(),
        material: String::from("brick-phong")
      })
    }
  }
}
/*******************************************************************************
  camera.rs
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
use nalgebra::{Vector2, Matrix3};
use crate::render::primitives::Camera2D;
use crate::controller::Camera2DController;

#[pyclass]
pub struct Camera {
  pub world_object: Camera2DController
}

#[pymethods]
impl Camera {
  #[staticmethod]
  pub fn new() -> Self {
    Self {
      world_object: IMAGINE.lock().unwrap().world.add_camera2d(Camera2D {
        aspect: 16.0 / 9.0,
        rotation: 0.0,
        scale: Vector2::new(1.0, 1.0),
        position: Vector2::new(0.0, 0.0),
        view: Matrix3::identity()
      })
    }
  }

  // pub fn set_default(&self) {
  //   IMAGINE.lock().unwrap().world.default_camera_2d = self.world_object.id;
  // }

  // #[pyo3(signature = (filename="snapshot.png"))]
  // pub fn snapshot(&self, filename: &str) {
  //   let current_cam = IMAGINE.lock().unwrap().world.default_camera_2d;
  //   self.set_default();
  //   // IMAGINE.lock().unwrap().snapshot()
  //   IMAGINE.lock().unwrap().world.default_camera_2d = current_cam;
  // }
}
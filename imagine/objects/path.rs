/*******************************************************************************
  path.rs
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
use nalgebra::{Vector2, Matrix3};
use crate::render::primitives::PathConfig;
use crate::controller::Object2DController;

// pub enum PathSegment {
//   MoveTo(f32, f32),
//   LineTo(f32, f32),
//   QuadTo(f32, f32, f32, f32),
//   CubicTo(f32, f32, f32, f32),
//   Close
// }

#[pyclass]
pub struct Path {
  pub world_object: Object2DController
}

#[pymethods]
impl Path {
  #[new]
  #[pyo3(signature=(path=String::new()))]
  pub fn new(path: String) -> Self {
    Self {
      world_object: IMAGINE.lock().unwrap().world.add_path(PathConfig {
        segments: 0,
        bounds: [0.0, 0.0, 0.0, 0.0],
        scale: Vector2::new(1.0, 1.0),
        position: Vector2::zeros(),
        rotation: 0.0,
        transform: Matrix3::identity()
      })
    }
  }

  pub fn move_to(&self, x: f32, y: f32) {}
  pub fn line_to(&self, x: f32, y: f32) {}
  // pub fn quad_to(&self, x: f32, y: f32, cx: f32, cy: f32) {}
  // pub fn cubic_to(&self, x: f32, y: f32, cx1: f32, cy1: f32, cx2: f32, cy2: f32) {}
  // pub fn arc_to(&self) {}
  pub fn close(&self) {}
}

impl Path {
  pub fn empty() -> Self {
    Self {
      world_object: IMAGINE.lock().unwrap().world.add_path(PathConfig {
        segments: 0,
        bounds: [0.0, 0.0, 0.0, 0.0],
        scale: Vector2::new(1.0, 1.0),
        position: Vector2::zeros(),
        rotation: 0.0,
        transform: Matrix3::identity()
      })
    }
  }
}
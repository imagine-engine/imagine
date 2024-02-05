/*******************************************************************************
  polygon.rs
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
use delaunator::{Point, triangulate};
use crate::controller::Object2DController;
use crate::render::primitives::Object2D;

#[pyclass]
pub struct Polygon {
  #[pyo3(get)]
  pub scale: Vector,
  #[pyo3(get)]
  pub position: Vector,
  #[pyo3(get)]
  pub rotation: f32,
  pub world_object: Object2DController
}

impl Polygon {
  pub fn new(positions: Vec<[f32; 2]>) -> Self {
    let mut min_x = positions[0][0];
    let mut max_x = positions[0][0];
    let mut min_y = positions[0][1];
    let mut max_y = positions[0][1];
    let mut points = Vec::new();
    for i in 0..positions.len() {
      if positions[i][0] > max_x { max_x = positions[i][0]; }
      else if positions[i][0] < min_x { min_x = positions[i][0]; }
      else if positions[i][1] > max_y { max_y = positions[i][1]; }
      else if positions[i][1] < min_y { min_y = positions[i][1]; }

      points.push(Point {
        x: positions[i][0] as f64,
        y: positions[i][1] as f64
      });
    }

    let width = max_x - min_x;
    let height = max_x - min_x;
    let vertices = positions.iter().map(|position| Vertex2D {
      position: *position,
      uv: [
        (position[0]-min_x).abs() / width,
        (position[1]-min_y).abs() / height
      ]
    }).collect();

    Self {
      scale: Vector::new(1.0, 1.0, 1.0),
      position: Vector::new(0.0, 0.0, 0.0),
      rotation: 0.0,
      world_object: IMAGINE.lock().unwrap().world.add_path(Object2D {
        vertices,
        indices: triangulate(&points).triangles.iter().map(|i| *i as _).collect(),
        scale: Vector2::new(1.0, 1.0),
        position: Vector2::zeros(),
        rotation: 0.0,
        transform: Matrix3::identity(),
        material: String::from("checkerboard")
      })
    }
  }
}

#[pymethods]
impl Polygon {
  #[setter(scale)]
  fn set_scale(&mut self, new_scale: Vector) {
    self.scale = new_scale;
    self.world_object.update_transform(
      self.scale.clone(),
      self.position.clone(),
      self.rotation
    );
  }
}
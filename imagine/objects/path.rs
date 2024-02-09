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
use std::f32::consts::PI;
use crate::instance::IMAGINE;
use crate::math::vector::Vector;
use nalgebra::{Vector2, Matrix3};
use crate::render::primitives::PathConfig;
use crate::controller::Object2DController;

#[pyclass]
pub struct Path {
  segment_closed: bool,
  temp_bounds: [f32; 4],
  temp_points: Vec<f32>,
  temp_segments: Vec<u8>,
  pub world_object: Option<Object2DController>
}

#[pymethods]
impl Path {
  #[new]
  #[pyo3(signature=(path=String::new()))]
  pub fn new(path: String) -> Self {
    let mut path = Path::empty();
    // TODO: Parse string to path
    path
  }

  #[getter(bounds)]
  fn get_bounds(&self) -> PyResult<Option<[f32; 4]>> {
    if let Some(world_object) = &self.world_object {
      if let Some(path) = IMAGINE.lock().unwrap().world.paths.get(&world_object.id) {
        return Ok(Some(path.bounds));
      }
    }

    Ok(None)
  }

  fn update_bounds(&mut self, x: f32, y: f32) {
    if x < self.temp_bounds[0] {
      self.temp_bounds[0] = x;
    } else if y < self.temp_bounds[1] {
      self.temp_bounds[1] = y;
    } else if x > self.temp_bounds[2] {
      self.temp_bounds[2] = x;
    } else if y > self.temp_bounds[3] {
      self.temp_bounds[3] = y;
    }
  }

  pub fn move_to(&mut self, x: f32, y: f32) {
    self.temp_points.push(x);
    self.temp_points.push(y);

    self.update_bounds(x, y);
    self.segment_closed = false;
  }

  pub fn line_to(&mut self, x: f32, y: f32) {
    if self.segment_closed {
      let last_x = self.temp_points[self.temp_points.len()-2];
      let last_y = self.temp_points[self.temp_points.len()-1];
      self.temp_points.push(last_x);
      self.temp_points.push(last_y);
    }

    self.temp_points.push(x);
    self.temp_points.push(y);

    self.update_bounds(x, y);
    self.temp_segments.push(0);
    self.segment_closed = true;
  }

  pub fn quad_to(&mut self, x: f32, y: f32, cx: f32, cy: f32) {
    if self.segment_closed {
      let last_x = self.temp_points[self.temp_points.len()-2];
      let last_y = self.temp_points[self.temp_points.len()-1];
      self.temp_points.push(last_x);
      self.temp_points.push(last_y);
    }

    self.temp_points.push(cx);
    self.temp_points.push(cy);
    self.temp_points.push(x);
    self.temp_points.push(y);

    self.update_bounds(x, y);
    self.temp_segments.push(1);
    self.segment_closed = true;
  }

  pub fn cubic_to(&mut self, x: f32, y: f32, cx1: f32, cy1: f32, cx2: f32, cy2: f32) {
    if self.segment_closed {
      let last_x = self.temp_points[self.temp_points.len()-2];
      let last_y = self.temp_points[self.temp_points.len()-1];
      self.temp_points.push(last_x);
      self.temp_points.push(last_y);
    }

    self.temp_points.push(cx1);
    self.temp_points.push(cy1);
    self.temp_points.push(cx2);
    self.temp_points.push(cy2);
    self.temp_points.push(x);
    self.temp_points.push(y);

    self.update_bounds(x, y);
    self.temp_segments.push(2);
    self.segment_closed = true;
  }

  pub fn close(&mut self) {
    self.line_to(self.temp_points[0], self.temp_points[1]);

    let config = PathConfig {
      opacity: 1.0,
      bounds: self.temp_bounds,
      scale: Vector2::new(1.0, 1.0),
      position: Vector2::zeros(),
      rotation: 0.0,
      transform: Matrix3::identity(),
      path_segments: self.temp_segments.len()
    };

    self.world_object = Some(IMAGINE.lock().unwrap().world.add_path(
      &self.temp_points,
      &self.temp_segments,
      config
    ));

    self.temp_points.clear();
    self.temp_segments.clear();
    self.segment_closed = false;
    self.temp_bounds = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];
  }

  #[pyo3(signature=(t=1.0, angle=2.0*PI))]
  pub fn rotate(&self, t: f32, angle: f32) {
    if let Some(object) = &self.world_object {
      object.rotate(t, angle);
    }
  }
}

impl Path {
  pub fn empty() -> Self {
    Self {
      world_object: None,
      segment_closed: false,
      temp_points: Vec::new(),
      temp_segments: Vec::new(),
      temp_bounds: [f32::MAX, f32::MAX, f32::MIN, f32::MIN]
    }
  }
}
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

pub enum PathSegment {
  MoveTo(f32, f32),
  LineTo(f32, f32),
  QuadTo(f32, f32, f32, f32),
  CubicTo(f32, f32, f32, f32, f32, f32)
}

#[pyclass]
pub struct Path {
  pub temp_segments: Option<Vec<PathSegment>>,
  pub world_object: Option<Object2DController>
}

#[pymethods]
impl Path {
  #[new]
  #[pyo3(signature=(path=String::new()))]
  pub fn new(path: String) -> Self {
    Self {
      temp_segments: Some(Vec::new()),
      world_object: None
    }
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

  pub fn move_to(&mut self, x: f32, y: f32) {
    if let Some(ref mut segments) = self.temp_segments {
      segments.push(PathSegment::MoveTo(x, y));
    }
  }

  pub fn line_to(&mut self, x: f32, y: f32) {
    if let Some(ref mut segments) = self.temp_segments {
      segments.push(PathSegment::LineTo(x, y));
    }
  }

  pub fn quad_to(&mut self, x: f32, y: f32, cx: f32, cy: f32) {
    if let Some(ref mut segments) = self.temp_segments {
      segments.push(PathSegment::QuadTo(x, y, 0.0, 0.0));
    }
  }

  pub fn cubic_to(&mut self, x: f32, y: f32, cx1: f32, cy1: f32, cx2: f32, cy2: f32) {
    if let Some(ref mut segments) = self.temp_segments {
      segments.push(PathSegment::CubicTo(x, y, 0.0, 0.0, 0.0, 0.0));
    }
  }

  // pub fn arc_to(&self) {}

  pub fn close(&mut self) {
    if let Some(temp_segments) = &self.temp_segments {
      let mut segments: Vec<f32> = Vec::new();
      let mut windings: Vec<i32> = Vec::new();
      let mut bounds = [f32::MAX, f32::MAX, f32::MIN, f32::MIN];

      for segment in temp_segments.iter() {
        if let Some(last_y) = segments.last() {
          match segment {
            PathSegment::MoveTo(_, _) => (),
            PathSegment::LineTo(_, y) => windings.push(if y < last_y { 1 } else { -1 }),
            PathSegment::QuadTo(_, y, _, _) => windings.push(if y < last_y { 1 } else { -1 }),
            PathSegment::CubicTo(_, y, _, _, _, _) => windings.push(if y < last_y { 1 } else { -1 })
          }
        }

        if segments.len() % 4 == 0 {
          match segment {
            PathSegment::MoveTo(x, y) => segments.extend([x, y]),
            PathSegment::LineTo(x, y) => {
              segments.extend([segments[segments.len()-2], segments[segments.len()-1], *x, *y]);
            },
            PathSegment::QuadTo(x, y, _, _) => {
              segments.extend([segments[segments.len()-2], segments[segments.len()-1], *x, *y]);
            },
            PathSegment::CubicTo(x, y, _, _, _, _) => {
              segments.extend([segments[segments.len()-2], segments[segments.len()-1], *x, *y]);
            }
          }
        } else {
          segments.extend(match segment {
            PathSegment::MoveTo(x, y) => [x, y],
            PathSegment::LineTo(x, y) => [x, y],
            PathSegment::QuadTo(x, y, _, _) => [x, y],
            PathSegment::CubicTo(x, y, _, _, _, _) => [x, y]
          });
        }
      }

      for i in (0..segments.len()).step_by(2) {
        if segments[i] < bounds[0] {
          bounds[0] = segments[i];
        } else if segments[i+1] < bounds[1] {
          bounds[1] = segments[i+1];
        } else if segments[i] > bounds[2] {
          bounds[2] = segments[i];
        } else if segments[i+1] > bounds[3] {
          bounds[3] = segments[i+1];
        }
      }

      let config = PathConfig {
        bounds,
        segments: (segments.len() / 4) as u32,
        scale: Vector2::new(1.0, 1.0),
        position: Vector2::zeros(),
        rotation: 0.0,
        transform: Matrix3::identity()
      };

      self.world_object = Some(IMAGINE.lock().unwrap().world.add_path(
        segments,
        windings,
        config
      ));

      self.temp_segments = None;
    }
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
      temp_segments: Some(Vec::new()),
      world_object: None
    }
  }
}
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

// use std::cell::Cell;
use pyo3::prelude::*;
use crate::animation::*;
use crate::math::Vector;
use std::f32::consts::PI;
use svgtypes::PathParser;
use svgtypes::PathSegment;
use svgtypes::PathSegment::*;
use crate::instance::IMAGINE;
use nalgebra::{Vector2, Matrix3};
use crate::render::primitives::PathConfig;

// pub enum Alignment {
//   TopLeft,
//   TopRight,
//   BottomLeft,
//   BottomRight,
//   Center
// }

#[pyclass]
pub struct PathBuilder {
  segment_closed: bool,
  temp_bounds: Option<[f32; 4]>,
  temp_points: Vec<f32>,
  temp_segments: Vec<u8>
}

#[pyclass]
pub struct Path {
  pub id: i32,
  #[pyo3(get, set)]
  pub opacity: f32,
  #[pyo3(get, set)]
  pub scale: Py<Vector>,
  #[pyo3(get, set)]
  pub position: Py<Vector>,
  pub rotation: f32,
  // pub rotation: Cell<f32>
}

#[pymethods]
impl Path {
  #[new]
  #[pyo3(signature=(d=""))]
  pub fn new(d: &str) -> Self {
    let mut path = PathBuilder::new();
    for seg in PathParser::from(d) {
      if let Ok(segment) = seg {
        path.push(segment);
      }
    }

    path.build()
  }

  #[getter(bounds)]
  fn get_bounds(&self) -> PyResult<Option<[f32; 4]>> {
    match IMAGINE.lock().unwrap().world.paths.get(&self.id) {
      Some(path) => Ok(Some(path.bounds)),
      None => Ok(None)
    }
  }

  #[getter(rotation)]
  fn get_rotation(&self) -> PyResult<f32> {
    Ok(self.rotation)
    // Ok(self.rotation.get())
  }

  #[setter(rotation)]
  fn set_rotation(&mut self, new_rotation: f32) {
    self.rotation = new_rotation;
    // self.rotation.set(new_rotation);
  }

  // #[pyo3(signature=(py_args="*", t=1.0))]
  // pub fn grow(&self, py_args: &PyTuple, t: f32) {
  #[pyo3(signature=(x, y, t=1.0))]
  pub fn grow(&self, x: f32, y: f32, t: f32) {
    IMAGINE.lock().unwrap().run(t, &[
      Animation {
        duration: t,
        update: AnimationUpdate::Transform2D(
          self.id,
          Some(Vector2::new(x, y)),
          None,
          None
        ),
        interpolation: Interpolation::Linear
      }
    ]);
  }

  #[pyo3(signature=(x, y, t=1.0))]
  pub fn translate(&self, x: f32, y: f32, t: f32) {
    IMAGINE.lock().unwrap().run(t, &[
      Animation {
        duration: t,
        update: AnimationUpdate::Transform2D(
          self.id,
          None,
          Some(Vector2::new(x, y)),
          None
        ),
        interpolation: Interpolation::Linear
      }
    ]);
  }

  #[pyo3(signature=(t=1.0, angle=2.0*PI))]
  pub fn rotate(&self, t: f32, angle: f32) {
    IMAGINE.lock().unwrap().run(t, &[
      Animation {
        duration: t,
        update: AnimationUpdate::Transform2D(
          self.id,
          None,
          None,
          Some(angle)
        ),
        interpolation: Interpolation::Linear
      }
    ]);
  }
}

#[pymethods]
impl PathBuilder {
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

  pub fn h_line_to(&mut self, x: f32) {
    let last_y = self.temp_points[self.temp_points.len()-1];
    if self.segment_closed {
      let last_x = self.temp_points[self.temp_points.len()-2];
      self.temp_points.push(last_x);
      self.temp_points.push(last_y);
    }

    self.temp_points.push(x);
    self.temp_points.push(last_y);

    self.update_bounds(x, last_y);
    self.temp_segments.push(0);
    self.segment_closed = true;
  }

  pub fn v_line_to(&mut self, y: f32) {
    let last_x = self.temp_points[self.temp_points.len()-2];
    if self.segment_closed {
      let last_y = self.temp_points[self.temp_points.len()-1];
      self.temp_points.push(last_x);
      self.temp_points.push(last_y);
    }

    self.temp_points.push(last_x);
    self.temp_points.push(y);

    self.update_bounds(last_x, y);
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
    if let Some(bounds) = self.temp_bounds {
      self.line_to(self.temp_points[0], self.temp_points[1]);

      let cx = (bounds[0] + bounds[2]) / 2.0;
      let cy = (bounds[1] + bounds[3]) / 2.0;
      for i in (0..self.temp_points.len()-1).step_by(2) {
        self.temp_points[i] -= cx;
        self.temp_points[i+1] -= cy;
      }
    }
  }

  pub fn build(&mut self) -> Path {
    let bounds = match self.temp_bounds {
      Some(bbox) => {
        let cx = (bbox[2] + bbox[0]) / 2.0;
        let cy = (bbox[3] + bbox[1]) / 2.0;
        [bbox[0]-cx, bbox[1]-cy, bbox[2]-cx, bbox[3]-cy]
      },
      None => [0.0, 0.0, 0.0, 0.0]
    };

    let path = IMAGINE.lock().unwrap().world.add_path(
      &self.temp_points,
      &self.temp_segments,
      bounds,
      self.temp_segments.len()
    );

    self.temp_points.clear();
    self.temp_segments.clear();
    self.segment_closed = false;
    self.temp_bounds = None;

    path
  }
}

impl PathBuilder {
  pub fn new() -> Self {
    Self {
      segment_closed: false,
      temp_points: Vec::new(),
      temp_segments: Vec::new(),
      temp_bounds: None
    }
  }

  fn update_bounds(&mut self, x: f32, y: f32) {
    if let Some(ref mut bounds) = self.temp_bounds {
      if x < bounds[0] { bounds[0] = x; }
      else if x > bounds[2] { bounds[2] = x; }

      if y < bounds[1] { bounds[1] = y; }
      else if y > bounds[3] { bounds[3] = y; }
    } else {
      self.temp_bounds = Some([x, y, x, y]);
    }
  }

  fn push(&mut self, segment: PathSegment) {
    let absolute = match segment {
      // MoveTo { abs, x, y } => abs,
      LineTo { abs, x, y } => abs,
      HorizontalLineTo { abs, x } => abs,
      VerticalLineTo { abs, y } => abs,
      Quadratic { abs, x1, y1, x, y } => abs,
      CurveTo { abs, x1, y1, x2, y2, x, y } => abs,
      // ClosePath { abs } => abs,
      _ => true
    };

    let rx = if absolute { 0.0 } else { self.temp_points[self.temp_points.len()-2] as f64 };
    let ry = if absolute { 0.0 } else { self.temp_points[self.temp_points.len()-1] as f64 };
    match segment {
      MoveTo { abs, x, y } => self.move_to((rx+x) as f32, (rx-y) as f32),
      LineTo { abs, x, y } => self.line_to((rx+x) as f32, (ry-y) as f32),
      HorizontalLineTo { abs, x } => self.h_line_to((rx+x) as f32),
      VerticalLineTo { abs, y } => self.v_line_to((ry-y) as f32),
      Quadratic { abs, x1, y1, x, y } => self.quad_to(
        (rx+x) as f32, (ry-y) as f32,
        (rx+x1) as f32, (ry-y1) as f32
      ),
      CurveTo { abs, x1, y1, x2, y2, x, y } => self.cubic_to(
        (rx+x) as f32, (ry-y) as f32,
        (rx+x1) as f32, (ry-y1) as f32,
        (rx+x2) as f32, (ry-y2) as f32
      ),
      ClosePath { abs } => self.close(),
      _ => ()
    }
  }
}